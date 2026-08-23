from __future__ import annotations

from typing import Any

from worth_ui_3141_closure_sources import (
    CLOSURE_PROTOCOL_SOURCES,
    PREDECESSOR_EXECUTION_SOURCES,
)
from worth_ui_3141_p5_source_owners import CERT_ROOT, LEDGER


def phase_five_close_proof(proof_type: Any, control_type: Any) -> Any:
    ledger = f"{CERT_ROOT}/milestone_3141_phase1_ledger.rs"
    mutation = f"{LEDGER}/mutation_tests.rs"
    return proof_type(
        "worth-ui-certification",
        ("test", "topology_contracts"),
        "milestone_3141_phase1_ledger::phase_five_closure_requires_every_predecessor_and_phase_five_row",
        f"{ledger}::validate_phase_closure",
        f"{ledger}::phase_five_closure_requires_every_predecessor_and_phase_five_row",
        (
            ledger,
            f"{LEDGER}/phase_progression.rs",
            mutation,
            "workspaces/worth-ui/docs/text-platform.md",
            "workspaces/worth-ui/crates/worth-ui/examples/text_platform.rs",
            "workspaces/worth-ui/crates/worth-ui/src/lib.rs",
            "workspaces/worth-ui/crates/worth-ui/src/facade/app.rs",
            "workspaces/worth-ui/crates/worth-ui/src/facade/declaration.rs",
            "scripts/ci/worth_ui_3141_p5_proofs.py",
            "scripts/ci/worth_ui_3141_p5_proof_builders.py",
            "scripts/ci/worth_ui_3141_p5_closure_proof_builders.py",
            "scripts/ci/worth_ui_3141_p5_source_owners.py",
            "scripts/ci/worth_ui_3141_p5_source_worlds.py",
            "scripts/ci/close_worth_ui_3141_ledger.py",
            "scripts/ci/worth_ui_ledger_acceptance.py",
            "scripts/ci/worth_ui_ledger_closure_selection.py",
            "scripts/ci/verify_worth_ui_3141_ledger.py",
            "scripts/ci/worth_ui_3141_proof_plan.py",
            "scripts/ci/worth_ui_ledger_command.py",
            "scripts/ci/worth_ui_ledger_artifact_transaction.py",
            "scripts/ci/worth_ui_ledger_causal_revalidation.py",
            "scripts/ci/worth_ui_ledger_closure_storage.py",
            "scripts/ci/worth_ui_ledger_execution_observation_retention.py",
            "scripts/ci/worth_ui_ledger_runner_authentication.py",
            "scripts/ci/worth_ui_ledger_row_cache.py",
            "scripts/ci/worth_ui_ledger_row_execution.py",
            "scripts/ci/worth_ui_ledger_execution_runner.py",
            "scripts/ci/worth_ui_ledger_governed_snapshot.py",
            "scripts/ci/worth_ui_ledger_portfolio_snapshot.py",
            "scripts/ci/worth_ui_ledger_portfolio_executions.py",
            "scripts/ci/worth_ui_ledger_retained_portfolio.py",
            "scripts/ci/worth_ui_ledger_source_state.py",
            "scripts/ci/worth_ui_predecessor_causal_refresh.py",
            "scripts/ci/worth_ui_predecessor_candidate.py",
            "scripts/ci/worth_ui_predecessor_refresh_order.py",
            "scripts/ci/worth_ui_predecessor_refresh_runtime.py",
            "scripts/ci/worth_ui_predecessor_handoff.py",
            "scripts/ci/worth_ui_ledger_public_example.py",
            "scripts/ci/worth_ui_ledger_phase_five_portfolio.py",
            "scripts/ci/test_worth_ui_ledger_causal_revalidation.py",
            "scripts/ci/test_worth_ui_ledger_phase_selection.py",
            "scripts/ci/test_worth_ui_ledger_retained_portfolio.py",
            "scripts/ci/test_worth_ui_predecessor_causal_refresh.py",
            f"{LEDGER}/result_artifact_cost.rs",
            "_docs/worth-ui/milestone-3.14.1.md",
            "_docs/worth-ui/milestone-3.14.1-phase-5.md",
            "_docs/worth-ui/milestone-3.14.1-phase-5-implementation-plan.md",
            "_docs/worth-ui/worth_ui_roadmap.md",
            *CLOSURE_PROTOCOL_SOURCES,
        ),
        control=control_type(
            "worth-ui-certification",
            ("test", "topology_contracts"),
            "milestone_3141_phase1_ledger::mutation_tests::phase_closure_mode_rejects_open_rows_at_or_before_its_gate",
            mutation,
        ),
    )


def predecessor_proof(
    proof_type: Any, control_type: Any, predecessor_artifact: str
) -> Any:
    validator = f"{LEDGER}/predecessor_artifact.rs"
    causal_validator = f"{LEDGER}/predecessor_artifact/causal_reuse.rs"
    mapping_validator = f"{LEDGER}/predecessor_artifact/mapping_digest.rs"
    handoff = f"{LEDGER}/predecessor_handoff.rs"
    return proof_type(
        "worth-ui-certification",
        ("test", "topology_contracts"),
        "milestone_3141_phase1_ledger::predecessor_handoff::phase_five_predecessor_handoff_is_current",
        f"{validator}::validate",
        f"{handoff}::phase_five_predecessor_handoff_is_current",
        (
            validator,
            causal_validator,
            mapping_validator,
            f"{LEDGER}/predecessor_artifact/ledger_basis.rs",
            handoff,
            f"{LEDGER}/runner_artifact_authentication.rs",
            "scripts/ci/worth_ui_3141_p5_proofs.py",
            "scripts/ci/worth_ui_3141_p5_proof_builders.py",
            "scripts/ci/worth_ui_3141_p5_closure_proof_builders.py",
            "scripts/ci/worth_ui_3141_p5_source_owners.py",
            "scripts/ci/worth_ui_3141_p5_source_worlds.py",
            "scripts/ci/worth_ui_predecessor_handoff.py",
            "scripts/ci/worth_ui_predecessor_causal_refresh.py",
            "scripts/ci/worth_ui_predecessor_candidate.py",
            "scripts/ci/worth_ui_predecessor_refresh_order.py",
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
            "scripts/ci/verify_worth_ui_3141_ledger.py",
            "scripts/ci/worth_ui_ledger_operational_successors.py",
            "scripts/ci/worth_ui_ledger_phase_five_portfolio.py",
            "scripts/ci/worth_ui_ledger_portfolio_row.py",
            "scripts/ci/worth_ui_ledger_source_state.py",
            *PREDECESSOR_EXECUTION_SOURCES,
            predecessor_artifact,
        ),
        control=control_type(
            "worth-ui-certification",
            ("test", "topology_contracts"),
            "milestone_3141_phase1_ledger::predecessor_artifact::tests::phase_five_stale_source_or_missing_row_is_rejected",
            validator,
        ),
    )
