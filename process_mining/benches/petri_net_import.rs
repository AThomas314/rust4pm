use criterion::{black_box, criterion_group, criterion_main, Criterion};
use process_mining::{test_utils::get_test_data_path, Importable, PetriNet};

fn bench_petri_net_import(c: &mut Criterion) {
    let root = get_test_data_path().join("ocel");
    let input_path = root.join("order-management.sqlite");

    println!("Importing Petri net from {:?}", input_path);
    let mut group = c.benchmark_group("import_petri_nets");
    group.bench_function("import_petri_nets", |b| {
        b.iter(|| {
            black_box(PetriNet::import_from_path(&input_path).expect("Failed to import petri net"))
        });
    });
}
criterion_group!(benches, bench_petri_net_import);
criterion_main!(benches);
