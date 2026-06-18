use process_mining::{test_utils::get_test_data_path, Importable, OCEL};
use std::error::Error;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> Result<(), Box<dyn Error>> {
    let root = get_test_data_path().join("ocel");
    let input_path = root.join("order-management.sqlite");
    let _profiler = dhat::Profiler::builder()
        .file_name("dhat-ocel_csv_export.json")
        .build();
    let _ocel = OCEL::import_from_path(&input_path)?;
    drop(_profiler);
    Ok(())
}
