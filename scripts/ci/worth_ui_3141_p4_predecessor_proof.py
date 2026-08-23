from __future__ import annotations

from typing import Any

from worth_ui_3141_closure_sources import PREDECESSOR_EXECUTION_SOURCES


CERT_ROOT = "workspaces/worth-ui/crates/worth-ui-certification/tests"


def predecessor_proof(
    proof_type: Any, control_type: Any, predecessor_artifact: str
) -> Any:
    validator = f"{CERT_ROOT}/milestone_3141_phase1_ledger/predecessor_artifact.rs"
    handoff = f"{CERT_ROOT}/milestone_3141_phase1_ledger/predecessor_handoff.rs"
    return proof_type(
        "worth-ui-certification",
        ("test", "topology_contracts"),
        "milestone_3141_phase1_ledger::predecessor_handoff::phase_four_predecessor_handoff_is_current",
        f"{validator}::validate",
        f"{handoff}::phase_four_predecessor_handoff_is_current",
        (
            validator,
            f"{CERT_ROOT}/milestone_3141_phase1_ledger/predecessor_artifact/causal_reuse.rs",
            f"{CERT_ROOT}/milestone_3141_phase1_ledger/predecessor_artifact/mapping_digest.rs",
            f"{CERT_ROOT}/milestone_3141_phase1_ledger/predecessor_artifact/ledger_basis.rs",
            f"{CERT_ROOT}/milestone_3141_phase1_ledger/runner_artifact_authentication.rs",
            handoff,
            "scripts/ci/worth_ui_3141_p4_predecessor_proof.py",
            "scripts/ci/verify_worth_ui_3141_ledger.py",
            "scripts/ci/worth_ui_predecessor_causal_refresh.py",
            "scripts/ci/worth_ui_predecessor_candidate.py",
            "scripts/ci/worth_ui_predecessor_refresh_order.py",
            "scripts/ci/worth_ui_predecessor_refresh_runtime.py",
            "scripts/ci/worth_ui_ledger_causal_revalidation.py",
            "scripts/ci/worth_ui_ledger_candidate_basis.py",
            "scripts/ci/worth_ui_ledger_execution_observation_retention.py",
            "scripts/ci/worth_ui_ledger_execution_runner.py",
            "scripts/ci/worth_ui_ledger_governed_snapshot.py",
            "scripts/ci/worth_ui_predecessor_handoff_currentness.py",
            "scripts/ci/worth_ui_ledger_execution_identity.py",
            "scripts/ci/worth_ui_ledger_portfolio_snapshot.py",
            "scripts/ci/worth_ui_ledger_row_cache.py",
            "scripts/ci/worth_ui_ledger_runner_authentication.py",
            "scripts/ci/worth_ui_ledger_atomic_closure.py",
            "scripts/ci/worth_ui_ledger_phase_two_closure.py",
            "scripts/ci/worth_ui_ledger_phase_three_portfolio.py",
            "scripts/ci/worth_ui_ledger_phase_four_portfolio.py",
            "scripts/ci/worth_ui_ledger_operational_successors.py",
            "scripts/ci/worth_ui_predecessor_handoff.py",
            "scripts/ci/worth_ui_ledger_source_state.py",
            *PREDECESSOR_EXECUTION_SOURCES,
            predecessor_artifact,
        ),
        control=control_type(
            "worth-ui-certification",
            ("test", "topology_contracts"),
            "milestone_3141_phase1_ledger::predecessor_artifact::tests::phase_four_stale_source_or_missing_row_is_rejected",
            validator,
        ),
    )
