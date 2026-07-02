use std::fs;
use std::path::Path;

use super::{CompiledProductReuseScanPattern, ObservedPatterns};
use crate::workload_composition::compiled_product_reuse_inventory::error::CompiledProductReuseInventoryError;

pub(super) fn scan_file(
    path: &Path,
    relative_path: &str,
    observed: &mut ObservedPatterns,
) -> Result<(), CompiledProductReuseInventoryError> {
    observed.scanned_file_count += 1;
    let text = fs::read_to_string(path).map_err(|error| {
        CompiledProductReuseInventoryError::SourceScanFailure(format!(
            "cannot read {}: {error}",
            path.display()
        ))
    })?;
    let mut impl_context = ImplContext::default();
    for line in text.lines() {
        impl_context.observe_opening(line);
        let identifier = declared_identifier(line, impl_context.current_type());
        for pattern in CompiledProductReuseScanPattern::all() {
            if matches_line(pattern, relative_path, line, identifier.as_deref()) {
                *observed
                    .pattern_counts
                    .entry((relative_path.to_string(), pattern))
                    .or_default() += 1;
            }
        }
        impl_context.observe_closing(line);
    }
    Ok(())
}

fn matches_line(
    pattern: CompiledProductReuseScanPattern,
    path: &str,
    line: &str,
    identifier: Option<&str>,
) -> bool {
    match pattern {
        CompiledProductReuseScanPattern::ReuseIdentifier => {
            (path.contains("/evidence_lookup_index_product/")
                && matches_exact_identifier(identifier, "reuse_evidence_lookup_index_product"))
                || (path_contains_any(
                    path,
                    &[
                        "/evidence_lookup_index_product/",
                        "/evidence_lookup_public_closeout/",
                        "/public_closeout/",
                        "/worth_workload/",
                    ],
                ) && (matches_identifier(identifier, &["cache_key", "reuse_key"])
                    || line_contains_any(line, &["cache_key", "reuse_key"])))
        }
        CompiledProductReuseScanPattern::EquivalenceIdentifier => {
            path.ends_with("/projection/diagnostic_surfaces/equivalence_contract.rs")
                && (matches_exact_identifier(identifier, "build_derived_equivalence_contract")
                    || matches_exact_identifier(
                        identifier,
                        "build_derived_equivalence_contract_report",
                    )
                    || matches_exact_identifier(
                        identifier,
                        "compare_derived_equivalence_contracts",
                    ))
        }
        CompiledProductReuseScanPattern::ParityIdentifier => {
            path.ends_with("/retained_replay_workload/replay_parity.rs")
                && matches_exact_identifier(
                    identifier,
                    "ReplayParityReport::from_retained_projection_match",
                )
        }
        CompiledProductReuseScanPattern::RebuildSuppressionLine => {
            line.contains("DerivedTopologyUpdatePosture::BoundedRebuildRequired")
                && line.contains("Self::BoundedRebuild")
        }
        CompiledProductReuseScanPattern::RowCountShortcutLine => {
            (path.ends_with("/retained_replay_workload/replay_parity.rs")
                && matches_identifier(identifier, &["::row_count"]))
                || line.contains("format!(\"basis-rows:{}\"")
                || (line.contains("row_count")
                    && line_contains_any(line, &["==", "!="])
                    && !is_cardinality_completeness_check(line))
        }
        CompiledProductReuseScanPattern::RenderedShapeEqualityLine => {
            line.trim_start().starts_with("equivalent_derived_meaning:")
                || (line.contains("rendered") && line_contains_any(line, &["==", "!="]))
        }
        CompiledProductReuseScanPattern::PointerIdentityLine => {
            line.contains("ptr::eq(")
                || line.contains("Arc::ptr_eq(")
                || line.contains("Rc::ptr_eq(")
                || (line.contains(".as_ptr()") && line_contains_any(line, &["==", "!="]))
        }
        CompiledProductReuseScanPattern::RetainedFolkloreIdentifier => {
            path.contains("/retained_replay_workload/")
                && (matches_exact_identifier(
                    identifier,
                    "RetainedArtifactCaptureReceipt::from_artifacts",
                ) || matches_exact_identifier(
                    identifier,
                    "ReplayWorkload::with_captured_retained_workload",
                ) || matches_exact_identifier(identifier, "replay_capture_receipt")
                    || matches_identifier(identifier, &["_helper"]))
        }
        CompiledProductReuseScanPattern::PublicReadModelReuseLine => {
            (path.ends_with("/projection/runtime_boundary/read_execution/basis_context.rs")
                && line.contains("retained_reuse"))
                || (path.ends_with(
                    "/replay_undo_consumer_cutover/public_closeout/inventory_classification.rs",
                ) && matches_exact_identifier(
                    identifier,
                    "ReplayUndoPublicCloseoutInventoryRow::from_inventory",
                ))
        }
        CompiledProductReuseScanPattern::LookupConsumerIdentifier => {
            path_contains_any(
                path,
                &[
                    "/worth_workload/lookup_consumed_workload/mod.rs",
                    "/worth_workload/ordinary_consumer_sweep/lookup_consumed_cluster.rs",
                ],
            ) && (matches_exact_identifier(identifier, "LookupConsumedWorkloadComposition::admit")
                || matches_exact_identifier(
                    identifier,
                    "WorthWorkload::admit_lookup_consumed_workload",
                )
                || matches_exact_identifier(
                    identifier,
                    "WorthWorkload::admit_lookup_consumed_batch_execution_cluster",
                ))
        }
        CompiledProductReuseScanPattern::CloseoutConsumerIdentifier => {
            path_contains_any(
                path,
                &[
                    "/planner_owned_routing/public_closeout_route/current.rs",
                    "/worth_workload/ordinary_consumer_sweep/current_cutover.rs",
                    "/public_closeout/public_closeout.rs",
                ],
            ) && (matches_exact_identifier(identifier, "current_evidence_lookup_public_closeout")
                || matches_exact_identifier(
                    identifier,
                    "current_evidence_lookup_public_closeout_assembly_input",
                )
                || matches_exact_identifier(
                    identifier,
                    "current_worth_workload_ordinary_consumer_cutover",
                )
                || matches_exact_identifier(
                    identifier,
                    "current_worth_touched_graph_conflict_public_closeout",
                )
                || matches_exact_identifier(
                    identifier,
                    "current_worth_touched_graph_conflict_milestone_fifteen_seed",
                ))
        }
    }
}

fn matches_exact_identifier(identifier: Option<&str>, expected: &str) -> bool {
    identifier == Some(expected)
}

fn matches_identifier(identifier: Option<&str>, needles: &[&str]) -> bool {
    let Some(identifier) = identifier else {
        return false;
    };
    needles.iter().any(|needle| identifier.contains(needle))
}

fn line_contains_any(line: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| line.contains(needle))
}

fn path_contains_any(path: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| path.contains(needle))
}

fn is_cardinality_completeness_check(line: &str) -> bool {
    line.contains(".len()")
        && line_contains_any(line, &["== expected_", "expected_"])
        && line.contains("row_count")
}

fn declared_identifier(line: &str, impl_type: Option<&str>) -> Option<String> {
    let rest = strip_declaration_prefixes(line.trim_start());
    for keyword in ["fn ", "struct ", "enum ", "mod "] {
        if let Some(name) = rest.strip_prefix(keyword) {
            let identifier = name
                .split(|ch: char| !(ch == '_' || ch.is_ascii_alphanumeric()))
                .next()
                .filter(|value| !value.is_empty())?;
            if keyword == "fn " {
                if let Some(impl_type) = impl_type {
                    return Some(format!("{impl_type}::{identifier}"));
                }
            }
            return Some(identifier.to_string());
        }
    }
    None
}

fn strip_declaration_prefixes(mut rest: &str) -> &str {
    loop {
        let next = rest
            .strip_prefix("pub(crate) ")
            .or_else(|| rest.strip_prefix("pub(super) "))
            .or_else(|| rest.strip_prefix("pub "))
            .or_else(|| rest.strip_prefix("async "))
            .or_else(|| rest.strip_prefix("const "));
        match next {
            Some(value) => rest = value.trim_start(),
            None => return rest,
        }
    }
}

#[derive(Default)]
struct ImplContext {
    current_type: Option<String>,
    brace_depth: i32,
}

impl ImplContext {
    fn current_type(&self) -> Option<&str> {
        self.current_type.as_deref()
    }

    fn observe_opening(&mut self, line: &str) {
        if self.current_type.is_none() {
            if let Some(type_name) = impl_type_name(line.trim_start()) {
                self.current_type = Some(type_name.to_string());
            }
        }
    }

    fn observe_closing(&mut self, line: &str) {
        if self.current_type.is_none() {
            return;
        }
        self.brace_depth += brace_delta(line);
        if self.brace_depth <= 0 {
            self.current_type = None;
            self.brace_depth = 0;
        }
    }
}

fn impl_type_name(line: &str) -> Option<&str> {
    let rest = line.strip_prefix("impl ")?;
    let candidate = rest
        .split(|ch: char| ch.is_whitespace() || ch == '{' || ch == '<')
        .next()?;
    if candidate.is_empty() || candidate.contains("::") {
        return None;
    }
    Some(candidate)
}

fn brace_delta(line: &str) -> i32 {
    line.chars().fold(0, |depth, ch| match ch {
        '{' => depth + 1,
        '}' => depth - 1,
        _ => depth,
    })
}
