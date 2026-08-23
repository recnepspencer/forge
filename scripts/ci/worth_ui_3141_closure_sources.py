from __future__ import annotations


CERT_LEDGER = (
    "workspaces/worth-ui/crates/worth-ui-certification/tests/"
    "milestone_3141_phase1_ledger"
)

PREDECESSOR_EXECUTION_SOURCES = (
    "scripts/ci/worth_ui_3141_closure_sources.py",
    "scripts/ci/worth_ui_3141_proof_plan.py",
    f"{CERT_LEDGER}/predecessor_artifact/execution_identity.rs",
    f"{CERT_LEDGER}/predecessor_artifact/execution_observation.rs",
    f"{CERT_LEDGER}/predecessor_artifact/causal_reuse.rs",
    "scripts/ci/worth_ui_ledger_execution_binding.py",
    "scripts/ci/worth_ui_ledger_execution_identity.py",
    "scripts/ci/worth_ui_ledger_execution_observation.py",
    "scripts/ci/worth_ui_ledger_execution_observation_store.py",
    "scripts/ci/worth_ui_ledger_execution_observation_migration.py",
    "scripts/ci/worth_ui_ledger_execution_reference_validation.py",
    "scripts/ci/worth_ui_ledger_portfolio_executions.py",
    "scripts/ci/worth_ui_ledger_artifact_transaction.py",
    "scripts/ci/worth_ui_ledger_artifact_identity.py",
    "scripts/ci/worth_ui_ledger_artifact_publication.py",
    "scripts/ci/worth_ui_ledger_candidate_basis.py",
    "scripts/ci/worth_ui_ledger_causal_revalidation.py",
    "scripts/ci/worth_ui_ledger_execution_observation_retention.py",
    "scripts/ci/worth_ui_ledger_legacy_execution_archive.py",
    "scripts/ci/worth_ui_ledger_portfolio_snapshot.py",
    "scripts/ci/worth_ui_ledger_row_cache.py",
    "scripts/ci/worth_ui_ledger_runner_authentication.py",
    "scripts/ci/worth_ui_predecessor_candidate.py",
    "scripts/ci/worth_ui_predecessor_causal_refresh.py",
    "scripts/ci/worth_ui_predecessor_handoff.py",
    "scripts/ci/worth_ui_predecessor_refresh_order.py",
    "scripts/ci/worth_ui_predecessor_refresh_runtime.py",
)

CLOSURE_PROTOCOL_SOURCES = PREDECESSOR_EXECUTION_SOURCES + (
    f"{CERT_LEDGER}/execution_contract.rs",
    "scripts/ci/close_worth_ui_3141_ledger.py",
    "scripts/ci/worth_ui_ledger_acceptance.py",
    "scripts/ci/worth_ui_ledger_atomic_closure.py",
    "scripts/ci/worth_ui_ledger_closure_selection.py",
    "scripts/ci/worth_ui_ledger_closure_storage.py",
    "scripts/ci/worth_ui_ledger_retained_portfolio.py",
    "scripts/ci/worth_ui_ledger_row_cache.py",
    "scripts/ci/worth_ui_ledger_row_evidence.py",
    "scripts/ci/worth_ui_ledger_row_execution.py",
    "scripts/ci/worth_ui_ledger_shared_execution_lineage.py",
    "scripts/ci/test_worth_ui_ledger_phase_selection.py",
    "scripts/ci/test_worth_ui_3141_proof_source_inventory.py",
    "scripts/ci/test_worth_ui_ledger_staged_lineage.py",
    "scripts/ci/worth_ui_ledger_operational_verification_tests.py",
    "scripts/ci/test_worth_ui_ledger_legacy_execution_archive.py",
)
