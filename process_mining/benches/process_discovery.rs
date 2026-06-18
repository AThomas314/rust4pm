use criterion::{black_box, criterion_group, criterion_main, Criterion};
use process_mining::core::event_data::case_centric::utils::activity_projection::EventLogActivityProjection;
use process_mining::discovery::case_centric::alphappp::full::{
    alphappp_discover_petri_net, AlphaPPPConfig,
};
use process_mining::{test_utils::get_test_data_path, EventLog, Importable};

fn bench_process_discovery(c: &mut Criterion) {
    let root = get_test_data_path().join("ocel");
    let input_path = root.join("order-management.sqlite");
    let log = EventLog::import_from_path(&input_path).expect("Failed to import EventLog from path");
    let projection = EventLogActivityProjection::from(&log);
    let mut group = c.benchmark_group("import_petri_nets");
    let config = AlphaPPPConfig::default();
    group.bench_function("import_petri_nets", |b| {
        b.iter(|| black_box(alphappp_discover_petri_net(&projection, config)))
    });
}
criterion_group!(benches, bench_process_discovery);
criterion_main!(benches);
