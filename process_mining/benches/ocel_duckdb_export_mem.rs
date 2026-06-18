use std::collections::HashSet;

use process_mining::{test_utils::get_test_data_path, Exportable, Importable, OCEL};
use std::error::Error;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

pub fn main() -> Result<(), Box<dyn Error>> {
    let root = get_test_data_path().join("ocel");
    let mut path = root.join("order-management.sqlite");
    let mut ocel = OCEL::import_from_path(&path)?;
    // Including invalid E2O relations (i.e., to objects that do not exist) can cause corrupted or incomplete SQL exports
    // Thus, we filter the E2O relations to only keep valid ones
    let _profiler = dhat::Profiler::builder()
        .file_name("dhat-ocel_duckdb_export.json")
        .build();
    let all_obj_ids: HashSet<_> = ocel.objects.iter().map(|o| &o.id).collect();
    for e in &mut ocel.events {
        e.relationships
            .retain(|r| all_obj_ids.contains(&r.object_id));
    }
    // Export
    path.set_file_name(format!(
        "{}.duckdb",
        path.file_name()
            .and_then(|p| p.to_str())
            .unwrap_or_default()
    ));
    ocel.export_to_path(&path)?;
    drop(_profiler);
    if path.exists() {
        std::fs::remove_file(&path)?;
    }
    Ok(())
}
