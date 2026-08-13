from __future__ import annotations

from typing import Any


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
            handoff,
            "scripts/ci/worth_ui_3141_p4_predecessor_proof.py",
            "scripts/ci/verify_worth_ui_3141_ledger.py",
            "scripts/ci/worth_ui_ledger_phase_two_portfolio.py",
            "scripts/ci/worth_ui_ledger_phase_three_portfolio.py",
            "scripts/ci/worth_ui_ledger_phase_four_portfolio.py",
            "scripts/ci/worth_ui_ledger_operational_successors.py",
            "scripts/ci/worth_ui_predecessor_handoff.py",
            "scripts/ci/worth_ui_ledger_source_state.py",
            predecessor_artifact,
        ),
        control=control_type(
            "worth-ui-certification",
            ("test", "topology_contracts"),
            "milestone_3141_phase1_ledger::predecessor_artifact::tests::phase_four_stale_source_or_missing_row_is_rejected",
            validator,
        ),
    )
