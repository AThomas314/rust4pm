use criterion::{black_box, criterion_group, criterion_main, Criterion};
use process_mining::core::event_data::object_centric::ocel_csv::export_ocel_csv_to_path;
use process_mining::{test_utils::get_test_data_path, Importable, OCEL};

fn bench_ocel_csv_export(c: &mut Criterion) {
    let root = get_test_data_path().join("ocel");
    let input_path = root.join("order-management.sqlite");
    let output_path = input_path.with_extension("ocel.csv");
    let ocel = OCEL::import_from_path(&input_path).expect("Failed to import OCEL from path");
    let mut group = c.benchmark_group("oc_declare");
    group.sample_size(100);
    group.bench_function("bench_ocel_csv_export", |b| {
        b.iter(|| black_box(export_ocel_csv_to_path(&ocel, &output_path)))
    });
}
criterion_group!(benches, bench_ocel_csv_export);
criterion_main!(benches);
