use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use process_mining::{test_utils::get_test_data_path, Exportable, Importable, OCEL};
use std::collections::HashSet;

pub fn bench_ocel_duckdb_export(c: &mut Criterion) {
    let root = get_test_data_path().join("ocel");
    let sqlite_path = root.join("order-management.sqlite");

    let mut ocel = OCEL::import_from_path(&sqlite_path).expect("Failed to import OCEL");

    let all_obj_ids: HashSet<_> = ocel.objects.iter().map(|o| &o.id).collect();
    for e in &mut ocel.events {
        e.relationships
            .retain(|r| all_obj_ids.contains(&r.object_id));
    }

    let duckdb_path = root.join("order-management.duckdb");

    c.bench_function("ocel_duckdb_export", |b| {
        b.iter_batched(
            || {
                let _ = std::fs::remove_file(&duckdb_path);
            },
            |_| {
                ocel.export_to_path(&duckdb_path).unwrap();
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, bench_ocel_duckdb_export);
criterion_main!(benches);
