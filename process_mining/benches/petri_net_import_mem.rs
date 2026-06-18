use process_mining::{test_utils::get_test_data_path, Importable, PetriNet};
use std::error::Error;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> Result<(), Box<dyn Error>> {
    let root = get_test_data_path().join("ocel");
    let input_path = root.join("order-management.sqlite");

    println!("Importing Petri net from {:?}", input_path);
    let _profiler = dhat::Profiler::builder()
        .file_name("dhat-petri_net_import.json")
        .build();
    let _petri_net = PetriNet::import_from_path(&input_path)?;
    drop(_profiler);
    Ok(())
}
