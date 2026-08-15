from __future__ import annotations

from typing import Any


CERT_ROOT = "workspaces/worth-ui/crates/worth-ui-certification/tests"
LEDGER = f"{CERT_ROOT}/milestone_3141_phase1_ledger"


def build_p5_proofs(
    proof_type: Any,
    control_type: Any,
    predecessor_artifact: str,
) -> dict[str, Any]:
    return {
        "P5-PREDECESSOR-01": predecessor_proof(
            proof_type, control_type, predecessor_artifact
        ),
    }


def predecessor_proof(
    proof_type: Any, control_type: Any, predecessor_artifact: str
) -> Any:
    validator = f"{LEDGER}/predecessor_artifact.rs"
    handoff = f"{LEDGER}/predecessor_handoff.rs"
    return proof_type(
        "worth-ui-certification",
        ("test", "topology_contracts"),
        "milestone_3141_phase1_ledger::predecessor_handoff::phase_five_predecessor_handoff_is_current",
        f"{validator}::validate",
        f"{handoff}::phase_five_predecessor_handoff_is_current",
        (
            validator,
            handoff,
            "scripts/ci/worth_ui_3141_p5_proofs.py",
            "scripts/ci/verify_worth_ui_3141_ledger.py",
            "scripts/ci/worth_ui_ledger_phase_five_portfolio.py",
            predecessor_artifact,
        ),
        control=control_type(
            "worth-ui-certification",
            ("test", "topology_contracts"),
            "milestone_3141_phase1_ledger::predecessor_artifact::tests::phase_five_stale_source_or_missing_row_is_rejected",
            validator,
        ),
    )
