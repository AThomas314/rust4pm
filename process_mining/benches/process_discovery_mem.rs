use process_mining::core::event_data::case_centric::utils::activity_projection::EventLogActivityProjection;
use process_mining::discovery::case_centric::alphappp::full::{
    alphappp_discover_petri_net, AlphaPPPConfig,
};
use process_mining::{test_utils::get_test_data_path, EventLog, Importable};
use std::error::Error;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> Result<(), Box<dyn Error>> {
    let root = get_test_data_path().join("ocel");
    let input_path = root.join("order-management.sqlite");
    let log = EventLog::import_from_path(&input_path)?;
    let _profiler = dhat::Profiler::builder()
        .file_name("dhat-process_discovery.json")
        .build();
    let projection = EventLogActivityProjection::from(&log);
    let config = AlphaPPPConfig::default();
    let petri_net = alphappp_discover_petri_net(&projection, config);

    drop(_profiler);
    println!(
        "Discovered Petri net with {} places and {} transitions.",
        petri_net.places.len(),
        petri_net.transitions.len()
    );
    Ok(())
}
