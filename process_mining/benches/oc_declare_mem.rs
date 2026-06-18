use process_mining::{
    core::event_data::object_centric::linked_ocel::SlimLinkedOCEL,
    discovery::object_centric::oc_declare::{
        discover_behavior_constraints, OCDeclareDiscoveryOptions,
    },
    test_utils::get_test_data_path,
    Importable, OCEL,
};

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

pub fn main() {
    let root = get_test_data_path().join("ocel");
    let path = root.join("ocel2-p2p.xml");
    let _profiler = dhat::Profiler::builder()
        .file_name("dhat-oc_declare.json")
        .build();
    let ocel = OCEL::import_from_path(&path).expect("Failed to import OCEL.");
    let locel = SlimLinkedOCEL::from_ocel(ocel);
    let _discovered_constraints = discover_behavior_constraints(
        &locel,
        OCDeclareDiscoveryOptions {
            ..Default::default()
        },
    );
    drop(_profiler);
}
