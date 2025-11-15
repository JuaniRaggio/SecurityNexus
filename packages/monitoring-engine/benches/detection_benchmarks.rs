// Performance benchmarks for monitoring engine

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use monitoring_engine::*;

fn bench_alert_creation(c: &mut Criterion) {
    c.bench_function("create_alert", |b| {
        b.iter(|| {
            let alert = Alert {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: chrono::Utc::now().timestamp() as u64,
                chain: black_box("test-chain".to_string()),
                severity: AlertSeverity::High,
                pattern: AttackPattern::FlashLoan,
                description: "Benchmark alert".to_string(),
                transaction_hash: Some("0xabc".to_string()),
                block_number: Some(1000),
                metadata: std::collections::HashMap::new(),
                recommended_actions: vec!["Action 1".to_string()],
            };
            black_box(alert);
        });
    });
}

fn bench_detection_result(c: &mut Criterion) {
    c.bench_function("detection_result_no_detection", |b| {
        b.iter(|| {
            black_box(DetectionResult::no_detection());
        });
    });

    c.bench_function("detection_result_detected", |b| {
        b.iter(|| {
            black_box(DetectionResult::detected(
                AttackPattern::FlashLoan,
                0.95,
                "Flash loan detected".to_string(),
                vec!["Evidence 1".to_string()],
            ));
        });
    });
}

fn bench_severity_comparison(c: &mut Criterion) {
    c.bench_function("severity_comparison", |b| {
        let sev1 = AlertSeverity::Critical;
        let sev2 = AlertSeverity::Medium;
        b.iter(|| {
            black_box(sev1 > sev2);
        });
    });
}

criterion_group!(
    benches,
    bench_alert_creation,
    bench_detection_result,
    bench_severity_comparison
);

criterion_main!(benches);
