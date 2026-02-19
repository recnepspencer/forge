//! CLI trace inspector for AI agent drill-down.
//!
//! Provides progressive disclosure of trace data so an AI agent can:
//! 1. `list` — See all traces with summary stats (one line each)
//! 2. `show <id>` — See spans and decision counts for a trace
//! 3. `decisions <id> [span]` — See individual decisions
//! 4. `issues [dir]` — Show only traces with non-deterministic decisions
//!
//! This avoids context bloat — the agent reads only what it needs.

use std::path::PathBuf;
use forge_view::trace_store::TraceStore;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    let command = args[1].as_str();

    match command {
        "list" => cmd_list(&args[2..]),
        "show" => cmd_show(&args[2..]),
        "decisions" => cmd_decisions(&args[2..]),
        "issues" => cmd_issues(&args[2..]),
        "help" | "--help" | "-h" => print_usage(),
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    eprintln!("forge-trace-cli — Trace inspector for AI drill-down");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("  forge-trace-cli list [DIR]             List all traces (one-line summary each)");
    eprintln!("  forge-trace-cli show <TRACE_ID> [DIR]  Show spans and stats for a trace");
    eprintln!("  forge-trace-cli decisions <TRACE_ID> [SPAN_ID] [DIR]");
    eprintln!("                                         Show decisions (optionally filtered by span)");
    eprintln!("  forge-trace-cli issues [DIR]            Show only traces with interesting decisions");
    eprintln!();
    eprintln!("DIR defaults to ./traces");
}

fn resolve_dir(args: &[String], skip: usize) -> PathBuf {
    args.get(skip)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("./traces"))
}

fn cmd_list(args: &[String]) {
    let dir = resolve_dir(args, 0);
    let mut store = TraceStore::new(dir);
    store.reload();

    let traces = store.list_traces();
    if traces.is_empty() {
        println!("No traces found.");
        return;
    }

    println!("{:<60} {:>6} {:>6} {:>6} {:>18}",
        "NAME", "DEC", "SPANS", "ISSUES", "HASH");
    println!("{}", "-".repeat(100));

    for t in traces {
        let issue_marker = if t.status == "error" {
            "❌".to_string()
        } else if t.interesting_count > 0 {
            format!("⚠ {}", t.interesting_count)
        } else {
            "✅".to_string()
        };

        println!("{:<60} {:>6} {:>6} {:>6} 0x{:016X}",
            truncate(&t.name, 58),
            t.total_decisions,
            t.span_count,
            issue_marker,
            t.state_hash,
        );
    }
}

fn cmd_show(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: forge-trace-cli show <TRACE_ID> [DIR]");
        std::process::exit(1);
    }

    let trace_id = &args[0];
    let dir = resolve_dir(args, 1);
    let mut store = TraceStore::new(dir);
    store.reload();

    let overview = match store.get_trace_overview(trace_id) {
        Some(o) => o,
        None => {
            let traces = store.list_traces();
            let matches: Vec<_> = traces.iter()
                .filter(|t| t.id.contains(trace_id.as_str()) || t.name.contains(trace_id.as_str()))
                .collect();

            if matches.is_empty() {
                eprintln!("Trace '{}' not found. Use 'list' to see available traces.", trace_id);
                std::process::exit(1);
            }

            eprintln!("Trace '{}' not found. Did you mean one of:", trace_id);
            for m in &matches {
                eprintln!("  {} ({})", m.id, m.name);
            }
            std::process::exit(1);
        }
    };

    println!("Trace: {}", overview.meta.name);
    println!("Hash:  0x{:016X}", overview.meta.state_hash);
    println!("Decisions: {}  Spans: {}  Issues: {}",
        overview.meta.total_decisions,
        overview.meta.span_count,
        overview.meta.interesting_count);
    println!();

    if overview.spans.is_empty() {
        println!("No spans recorded.");
    } else {
        println!("{:<6} {:<30} {:>6} {:>10} {:<16}",
            "SPAN", "NAME", "DEC", "TIME", "MAX_TIER");
        println!("{}", "-".repeat(72));

        for span in &overview.spans {
            println!("{:<6} {:<30} {:>6} {:>8.1}ms {:<16}",
                span.span_id,
                truncate(&span.name, 28),
                span.total_decisions,
                span.duration_micros as f64 / 1000.0,
                span.max_tier,
            );
        }
    }
}

fn cmd_decisions(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: forge-trace-cli decisions <TRACE_ID> [SPAN_ID] [DIR]");
        std::process::exit(1);
    }

    let trace_id = &args[0];

    let (span_id, dir_idx) = if args.len() > 1 {
        match args[1].parse::<u64>() {
            Ok(sid) => (Some(sid), 2),
            Err(_) => (None, 1),
        }
    } else {
        (None, 1)
    };

    let dir = resolve_dir(args, dir_idx);
    let mut store = TraceStore::new(dir);
    store.reload();

    let overview = match store.get_trace_overview(trace_id) {
        Some(o) => o,
        None => {
            eprintln!("Trace '{}' not found. Use 'list' to see available traces.", trace_id);
            std::process::exit(1);
        }
    };

    println!("Trace: {} ({})", overview.meta.name, trace_id);

    match span_id {
        Some(sid) => {
            println!("Span: {}", sid);
            println!();
            let decisions = store.get_span_decisions(trace_id, sid);
            match decisions {
                Some(decs) if !decs.is_empty() => {
                    print_decisions(&decs);
                }
                _ => println!("No decisions in span {}.", sid),
            }
        }
        None => {
            println!("All decisions:");
            println!();

            let all_span_ids: Vec<u64> = std::iter::once(0)
                .chain(overview.spans.iter().map(|s| s.span_id))
                .collect();

            let mut total_printed = 0;
            for sid in all_span_ids {
                if let Some(decs) = store.get_span_decisions(trace_id, sid) {
                    if !decs.is_empty() {
                        if sid == 0 {
                            println!("--- Unspanned ---");
                        } else {
                            let span_name = overview.spans.iter()
                                .find(|s| s.span_id == sid)
                                .map(|s| s.name.as_str())
                                .unwrap_or("?");
                            println!("--- Span {} ({}) ---", sid, span_name);
                        }
                        print_decisions(&decs);
                        total_printed += decs.len();
                        println!();
                    }
                }
            }
            if total_printed == 0 {
                println!("No decisions recorded.");
            }
        }
    }
}

fn cmd_issues(args: &[String]) {
    let dir = resolve_dir(args, 0);
    let mut store = TraceStore::new(dir);
    store.reload();

    let traces = store.list_traces();
    let issues: Vec<_> = traces.iter()
        .filter(|t| t.interesting_count > 0 || t.status == "error")
        .collect();

    if issues.is_empty() {
        println!("All traces are clean — no interesting decisions found.");
        return;
    }

    println!("{} trace(s) with interesting decisions:", issues.len());
    println!();
    println!("{:<60} {:>6} {:>6} {:>18}",
        "NAME", "DEC", "ISSUES", "ID");
    println!("{}", "-".repeat(94));

    for t in &issues {
        println!("{:<60} {:>6} {:>6} {}",
            truncate(&t.name, 58),
            t.total_decisions,
            t.interesting_count,
            t.id,
        );
    }
}

fn print_decisions(decisions: &[forge_view::trace_store::DecisionView]) {
    println!("{:<6} {:<20} {:<16} {:>10} {}",
        "ID", "KIND", "TIER", "MARGIN", "ENTITY");
    println!("{}", "-".repeat(66));

    for d in decisions {
        let entity_display = if d.entity.is_empty() || d.entity == "None" {
            "-".to_string()
        } else {
            d.entity.clone()
        };

        println!("{:<6} {:<20} {:<16} {:>10.6} {}",
            d.id,
            truncate(&d.kind, 18),
            d.tier,
            d.margin,
            entity_display,
        );
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max - 1])
    }
}
