//! Trace dump tests — writes full DecisionLog output to test_output.log.
//!
//! Run with: cargo test -p forge-kernel -- dump_trace --nocapture
//! Then open test_output.log to inspect span hierarchy, decisions, and timing.

use std::io::Write;

use super::super::test_helpers::build_cube;
use super::super::assemble::merge::execute_boolean;
use super::super::schema::{BooleanInput, BooleanOp};

fn write_section(file: &mut std::fs::File, header: &str, content: &str) {
    let bar = "=".repeat(60);
    writeln!(file, "{}", bar).unwrap();
    writeln!(file, "  {}", header).unwrap();
    writeln!(file, "{}", bar).unwrap();
    writeln!(file).unwrap();
    writeln!(file, "{}", content).unwrap();
    writeln!(file).unwrap();
}

/// Dump the full trace from a half-overlap intersection to test_output.log.
///
/// This lets you inspect:
/// 1. The span hierarchy with drill-down (display_interesting)
/// 2. The full decision list (Display)
/// 3. The raw event stream (JSON)
/// 4. The TraceSummary per-span stats
/// 5. The DecisionSummary aggregate
#[test]
fn dump_trace_intersection() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
    let (topo_b, geom_b) = build_cube([0.5, 0.0, 0.0], 1.0);

    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Intersection);
    let envelope = execute_boolean(input);
    assert!(envelope.get_value().is_ok(), "Boolean should succeed: {:?}", envelope.get_value().as_ref().err());

    let log = envelope.get_decision_log();
    let summary = log.summary();
    let trace_summary = log.to_summary(envelope.get_state_hash_after());

    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../test_output.log");
    let mut file = std::fs::File::create(path).expect("create output file");

    let inner_result = envelope.get_value().as_ref().unwrap();
    write_section(&mut file, "OPERATION: Intersection (half-overlap cubes)",
        &format!(
            "Duration: {:?}\nState hash: 0x{:016X}\nFaces in result: {}",
            envelope.get_metrics().duration,
            envelope.get_state_hash_after(),
            inner_result.topology().arena().face_count(),
        ),
    );

    write_section(&mut file, "SPAN HIERARCHY (display_interesting)", &log.display_interesting());

    write_section(&mut file, "DECISION SUMMARY", &format!("{}", summary));

    write_section(&mut file, "ALL DECISIONS (full log)", &format!("{}", log));

    let mut span_stats = String::new();
    for ss in trace_summary.get_span_summaries() {
        span_stats.push_str(&format!(
            "  {} — {} decisions, max_tier={}, {}µs\n",
            ss.name, ss.total_decisions, ss.max_tier, ss.duration_micros,
        ));
    }
    write_section(&mut file, "PER-SPAN STATS (TraceSummary)", &span_stats);

    write_section(&mut file, "RAW EVENT STREAM (JSON)",
        &serde_json::to_string_pretty(log).expect("serialize log"),
    );
}
