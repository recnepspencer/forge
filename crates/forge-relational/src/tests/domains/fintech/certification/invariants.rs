use std::collections::BTreeMap;

use forge_harness::facade::{InvariantCheck, InvariantReport};

use super::super::invariants::assert_fixture_shape;
use super::super::scales::FintechScale;
use super::session::CertifiedRelationalFintechSession;

pub(super) fn run_checks(
    session: &CertifiedRelationalFintechSession,
    checks: &[InvariantCheck],
) -> Result<Vec<InvariantReport>, String> {
    checks
        .iter()
        .map(|check| run_check(session, check))
        .collect()
}

fn run_check(
    session: &CertifiedRelationalFintechSession,
    check: &InvariantCheck,
) -> Result<InvariantReport, String> {
    let parts: Vec<_> = check.check_id.split(':').collect();
    let (passed, detail) = match parts.as_slice() {
        ["fixture_shape_smoke"] => {
            assert_fixture_shape(&session.world, FintechScale::smoke());
            (
                true,
                "fixture shape matches smoke fintech world".to_string(),
            )
        }
        ["read_nonempty", alias] => {
            let count = session
                .named_reads
                .get(*alias)
                .and_then(|value| value.get("entity_count"))
                .and_then(|value| value.as_u64())
                .unwrap_or_default();
            (
                count > 0,
                format!("read `{alias}` should contain at least one entity"),
            )
        }
        ["read_has_corrected_trade", alias] => {
            let count = session
                .named_reads
                .get(*alias)
                .and_then(|value| value.get("corrected_trade_count"))
                .and_then(|value| value.as_u64())
                .unwrap_or_default();
            (
                count > 0,
                format!("read `{alias}` should expose the corrected trade"),
            )
        }
        ["read_has_audit_truth", alias] => {
            let count = session
                .named_reads
                .get(*alias)
                .and_then(|value| value.get("audit_record_count"))
                .and_then(|value| value.as_u64())
                .unwrap_or_default();
            (
                count > 0,
                format!("read `{alias}` should expose audit truth"),
            )
        }
        ["read_matches_case", alias, case_role] => {
            let actual = session
                .named_reads
                .get(*alias)
                .and_then(|value| value.get("case_role"))
                .and_then(|value| value.as_str())
                .unwrap_or_default();
            (
                actual == *case_role,
                format!("read `{alias}` should target case `{case_role}`"),
            )
        }
        ["lineage_promotion_succeeded", alias] => {
            let resolution = session
                .named_lineage_resolutions
                .get(*alias)
                .copied()
                .unwrap_or(crate::facade::lineage::LineageResolutionStatus::Rejected);
            (
                resolution == crate::facade::lineage::LineageResolutionStatus::Promoted,
                format!("lineage promotion `{alias}` should succeed"),
            )
        }
        ["read_has_repaired_settlement", alias] => {
            let count = session
                .named_reads
                .get(*alias)
                .and_then(|value| value.get("repaired_settlement_count"))
                .and_then(|value| value.as_u64())
                .unwrap_or_default();
            (
                count > 0,
                format!("read `{alias}` should expose a repaired settlement"),
            )
        }
        ["case_correction_truth_visible", alias] => {
            let count = session
                .named_reads
                .get(*alias)
                .and_then(|value| value.get("corrected_trade_count"))
                .and_then(|value| value.as_u64())
                .unwrap_or_default();
            let audit = session
                .named_reads
                .get(*alias)
                .and_then(|value| value.get("audit_record_count"))
                .and_then(|value| value.as_u64())
                .unwrap_or_default();
            (
                count > 0 && audit > 0,
                format!("case read `{alias}` should expose correction truth and audit truth"),
            )
        }
        ["read_has_open_breach", alias] => {
            let count = session
                .named_reads
                .get(*alias)
                .and_then(|value| value.get("open_breach_count"))
                .and_then(|value| value.as_u64())
                .unwrap_or_default();
            (
                count > 0,
                format!("read `{alias}` should expose an open risk breach"),
            )
        }
        ["case_settlement_repair_visible", alias] => {
            let repaired = session
                .named_reads
                .get(*alias)
                .and_then(|value| value.get("repaired_settlement_count"))
                .and_then(|value| value.as_u64())
                .unwrap_or_default();
            let audit = session
                .named_reads
                .get(*alias)
                .and_then(|value| value.get("audit_record_count"))
                .and_then(|value| value.as_u64())
                .unwrap_or_default();
            (
                repaired > 0 && audit > 0,
                format!("case read `{alias}` should expose repaired settlement and audit truth"),
            )
        }
        ["branch_head_matches_latest", alias] => {
            let branch = session.branch(alias)?;
            let branch_head = session
                .world
                .runtime
                .history_access()
                .branch_head(&branch)
                .map(|head| head.commit_id);
            let latest = session
                .world
                .runtime
                .history_access()
                .latest_commit()
                .map(|commit| commit.commit_id);
            (
                branch_head == latest,
                format!("branch `{alias}` should point at the latest commit"),
            )
        }
        ["replay_has_no_failure", alias] => {
            let replay = session
                .named_replays
                .get(*alias)
                .ok_or_else(|| format!("unknown certified fintech replay alias `{alias}`"))?;
            (
                replay.failure.is_none(),
                format!("replay `{alias}` should complete without failure"),
            )
        }
        ["replay_targets_branch", alias, branch_alias] => {
            let replay = session
                .named_replays
                .get(*alias)
                .ok_or_else(|| format!("unknown certified fintech replay alias `{alias}`"))?;
            let branch = session.branch(branch_alias)?;
            (
                replay.requested.branch_id == branch,
                format!("replay `{alias}` should target branch `{branch_alias}`"),
            )
        }
        ["replay_has_lineage_authority_basis", alias] => {
            let replay = session
                .named_replays
                .get(*alias)
                .ok_or_else(|| format!("unknown certified fintech replay alias `{alias}`"))?;
            (
                replay.lineage_authority_basis.is_some(),
                format!("replay `{alias}` should expose lineage authority basis"),
            )
        }
        ["replay_uses_exact_lineage_digest", alias] => {
            let replay = session
                .named_replays
                .get(*alias)
                .ok_or_else(|| format!("unknown certified fintech replay alias `{alias}`"))?;
            (
                matches!(
                    replay.lineage_authority_basis.as_ref().map(|basis| basis.digest_mode()),
                    Some(crate::facade::replay::ReplayLineageDigestMode::ExactCanonicalArtifactDigest)
                ),
                format!("replay `{alias}` should use exact canonical lineage digest authority"),
            )
        }
        other => {
            return Err(format!(
                "unsupported certified fintech invariant `{}`",
                other.join(":")
            ))
        }
    };
    Ok(InvariantReport {
        check_id: check.check_id.clone(),
        boundary: check.boundary,
        passed,
        detail,
        fields: BTreeMap::new(),
    })
}
