use std::{collections::BTreeMap, path::PathBuf};

use super::workspace_source::read;
use crate::workspace_root;

const TRACE_DOCUMENT: &str = "_docs/worth-store/physical-reconstruction-c6-boundary-inventory.csv";
const TRACE_HEADER: &str = "lane,sequence,step,path,source_anchor,authority_owner";

const READ_STEPS: &[&str] = &[
    "record-entry",
    "pool-owner",
    "serving-route",
    "pool-fault",
    "canonical-adapter",
    "store-admission",
    "signal-readiness",
    "scheduler-admission",
    "executor-dispatch",
    "backend-read",
    "settlement",
];

const PUBLICATION_STEPS: &[&str] = &[
    "publication-entry",
    "candidate-residency",
    "store-admission",
    "signal-readiness",
    "scheduler-admission",
    "executor-dispatch",
    "backend-effect",
    "settlement",
    "root-eligibility",
    "publication-completion",
];

const BOOTSTRAP_STEPS: &[&str] = &[
    "bootstrap-route",
    "direct-source-construction",
    "backend-read",
];

#[test]
fn checked_in_trace_resolves_to_the_real_production_sources() {
    let rows = trace_rows().expect("parse physical residency boundary trace");
    for row in &rows {
        assert!(
            !row.owner.is_empty(),
            "{} step {} has no authority owner",
            row.lane,
            row.step
        );
        let path = workspace_root().join(&row.path);
        let source = read(&path).expect("read traced production source");
        assert!(
            source.contains(&row.anchor),
            "{} step {} lost source anchor `{}` in {}",
            row.lane,
            row.step,
            row.anchor,
            path.display()
        );
    }
}

#[test]
fn trace_has_complete_ordered_read_publication_and_bootstrap_lanes() {
    let rows = trace_rows().expect("parse physical residency boundary trace");
    validate_trace_shape(&rows).unwrap_or_else(|denial| panic!("{denial}"));
}

fn validate_trace_shape(rows: &[TraceRow]) -> Result<(), String> {
    let mut lanes = BTreeMap::<String, Vec<(u8, String)>>::new();
    for row in rows {
        lanes
            .entry(row.lane.clone())
            .or_default()
            .push((row.sequence, row.step.clone()));
    }
    for steps in lanes.values_mut() {
        steps.sort();
        for (expected, (actual, _)) in (1_u8..).zip(steps.iter()) {
            if expected != *actual {
                return Err("boundary trace sequence must be contiguous".to_owned());
            }
        }
    }
    let read = lanes
        .get("read")
        .ok_or_else(|| "boundary trace has no read lane".to_owned())?;
    let publication = lanes
        .get("publication")
        .ok_or_else(|| "boundary trace has no publication lane".to_owned())?;
    let bootstrap = lanes
        .get("bootstrap")
        .ok_or_else(|| "boundary trace has no bootstrap lane".to_owned())?;
    if step_names(read) != READ_STEPS
        || step_names(publication) != PUBLICATION_STEPS
        || step_names(bootstrap) != BOOTSTRAP_STEPS
    {
        return Err("boundary trace is incomplete or reordered".to_owned());
    }
    if lanes.len() != 3 {
        return Err("boundary trace contains an unclassified authority lane".to_owned());
    }
    Ok(())
}

#[test]
fn trace_parser_rejects_incomplete_reordered_and_ownerless_inventory() {
    let incomplete = format!("{TRACE_HEADER}\nread,1,record-entry,path.rs,anchor,Store\n");
    let rows = parse_trace(&incomplete).expect("row syntax is valid");
    assert!(validate_trace_shape(&rows).is_err());

    let ownerless = format!("{TRACE_HEADER}\nread,1,record-entry,path.rs,anchor,\n");
    assert!(parse_trace(&ownerless).is_err());

    let malformed =
        format!("{TRACE_HEADER}\nread,not-a-number,record-entry,path.rs,anchor,Store\n");
    assert!(parse_trace(&malformed).is_err());
}

fn trace_rows() -> Result<Vec<TraceRow>, String> {
    let path = repository_root().join(TRACE_DOCUMENT);
    parse_trace(&read(&path)?)
}

fn parse_trace(document: &str) -> Result<Vec<TraceRow>, String> {
    let mut lines = document.lines();
    if lines.next() != Some(TRACE_HEADER) {
        return Err("boundary trace has an invalid schema header".to_owned());
    }
    let mut rows = Vec::new();
    for (index, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let columns = line.split(',').map(str::trim).collect::<Vec<_>>();
        if columns.len() != 6 {
            return Err(format!(
                "boundary trace row {} has {} columns, expected 6",
                index + 2,
                columns.len()
            ));
        }
        if columns.iter().any(|column| column.is_empty()) {
            return Err(format!(
                "boundary trace row {} has an empty required field",
                index + 2
            ));
        }
        rows.push(TraceRow {
            lane: columns[0].to_owned(),
            sequence: columns[1]
                .parse()
                .map_err(|_| format!("boundary trace row {} has invalid sequence", index + 2))?,
            step: columns[2].to_owned(),
            path: columns[3].to_owned(),
            anchor: columns[4].to_owned(),
            owner: columns[5].to_owned(),
        });
    }
    Ok(rows)
}

fn step_names(steps: &[(u8, String)]) -> Vec<&str> {
    steps.iter().map(|(_, step)| step.as_str()).collect()
}

fn repository_root() -> PathBuf {
    workspace_root()
        .parent()
        .and_then(std::path::Path::parent)
        .expect("worth-store workspace must live under workspaces")
        .to_path_buf()
}

struct TraceRow {
    lane: String,
    sequence: u8,
    step: String,
    path: String,
    anchor: String,
    owner: String,
}
