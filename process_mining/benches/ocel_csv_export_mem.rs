use process_mining::core::event_data::object_centric::ocel_csv::export_ocel_csv_to_path;
use process_mining::{test_utils::get_test_data_path, Importable, OCEL};
use std::error::Error;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> Result<(), Box<dyn Error>> {
    let root = get_test_data_path().join("ocel");
    let input_path = root.join("order-management.sqlite");
    let output_path = input_path.with_extension("ocel.csv");
    let ocel = OCEL::import_from_path(&input_path)?;
    let _profiler = dhat::Profiler::builder()
        .file_name("dhat-ocel_csv_export.json")
        .build();
    export_ocel_csv_to_path(&ocel, &output_path)?;
    drop(_profiler);
    if output_path.exists() {
        std::fs::remove_file(&output_path)?;
    }
    Ok(())
}
