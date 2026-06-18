use criterion::{black_box, criterion_group, criterion_main, Criterion};
use process_mining::{
    core::event_data::object_centric::linked_ocel::SlimLinkedOCEL,
    discovery::object_centric::oc_declare::{
        discover_behavior_constraints, OCDeclareDiscoveryOptions,
    },
    test_utils::get_test_data_path,
    Importable, OCEL,
};
pub fn bench_oc_declare(c: &mut Criterion) {
    let root = get_test_data_path().join("ocel");
    let files = [
        "ocel2-p2p.xml",
        "ocel2-p2p.json",
        "pm4py-ocel20_example.jsonocel",
        "pm4py-ocel20_example.xmlocel",
    ];
    let mut group = c.benchmark_group("oc_declare");
    for file in files {
        let path = root.join(file);
        group.bench_function(file, |b| {
            b.iter(|| {
                let ocel =
                    black_box(OCEL::import_from_path(&path).expect("Failed to import OCEL."));
                let locel = black_box(SlimLinkedOCEL::from_ocel(ocel));
                let _discovered_constraints = black_box(discover_behavior_constraints(
                    &locel,
                    OCDeclareDiscoveryOptions {
                        ..Default::default()
                    },
                ));
            })
        });
    }
}

criterion_group!(benches, bench_oc_declare);
criterion_main!(benches);
