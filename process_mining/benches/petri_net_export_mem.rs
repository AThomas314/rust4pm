use process_mining::{test_utils::get_test_data_path, Exportable, Importable, PetriNet};
use std::error::Error;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> Result<(), Box<dyn Error>> {
    let root = get_test_data_path().join("ocel");
    let input_path = root.join("order-management.sqlite");
    let output_path = input_path.with_extension("pnml");
    let petri_net = PetriNet::import_from_path(&input_path)?;
    let _profiler = dhat::Profiler::builder()
        .file_name("dhat-petri_net_export.json")
        .build();
    petri_net.export_to_path(&output_path)?;
    drop(_profiler);
    if output_path.exists() {
        std::fs::remove_file(&output_path)?;
    }
    Ok(())
}
