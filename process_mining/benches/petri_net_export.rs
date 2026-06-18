use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use process_mining::{test_utils::get_test_data_path, Exportable, Importable, PetriNet};

fn bench_petri_net_export(c: &mut Criterion) {
    let root = get_test_data_path().join("ocel");
    let input_path = root.join("order-management.sqlite");
    let output_path = input_path.with_extension("pnml");
    let petri_net = PetriNet::import_from_path(&input_path).expect("Failed to import Petri Net");

    c.bench_function("petrinet_export", |b| {
        b.iter_batched(
            || {
                let _ = std::fs::remove_file(&output_path);
            },
            |_| {
                petri_net
                    .export_to_path(&output_path)
                    .expect("Failed to export Petri Net");
            },
            BatchSize::SmallInput,
        );
    });
}
criterion_group!(benches, bench_petri_net_export);
criterion_main!(benches);
