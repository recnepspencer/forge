use crate::query_obligation_selection::boundary_inventory::inventory_record::{
    QuerySelectionAuthorityPosture as Posture, QuerySelectionBoundaryInventoryRow,
    QuerySelectionDeletionAction as Action, QuerySelectionProofStrength as Proof,
    QuerySelectionSurfaceClassification as Class,
};
use crate::query_obligation_selection::boundary_inventory::row_constructors::{
    primitive, primitive_residue, primitive_with_caller,
};

pub(super) fn rows() -> Vec<QuerySelectionBoundaryInventoryRow> {
    vec![
        primitive("primitive_construction_birth_touch_descriptor", "catalog.rs", Class::SourceDescriptor, Posture::DescriptorInput, Proof::SourceDescriptorOnly, Action::KeepAsSourceDescriptor),
        primitive("primitive_construction_graph_obligation_catalog", "catalog.rs", Class::MigrationProjection, Posture::RegistrationDeclaration, Proof::RegistrationOnly, Action::MigrateToParallelSelectionSubstrate),
        primitive("primitive_construction_graph_obligation_registration_declaration", "catalog.rs", Class::MigrationProjection, Posture::RegistrationDeclaration, Proof::RegistrationOnly, Action::MigrateToParallelSelectionSubstrate),
        primitive("primitive_construction_graph_obligation_selector_coverage", "catalog.rs", Class::MigrationProjection, Posture::SelectorCoverageDeclaration, Proof::RegistrationOnly, Action::CollapseToQueryOwnedSelection),
        primitive("primitive_construction_graph_obligation_support_pin", "catalog.rs", Class::CertificationOnlySupport, Posture::SupportPin, Proof::SupportOnly, Action::CertificationOnly),
        primitive("primitive_construction_graph_obligation_support_matrix", "catalog.rs", Class::CertificationOnlySupport, Posture::SupportMatrix, Proof::SupportOnly, Action::CertificationOnly),
        primitive("primitive_construction_graph_obligation_local_ceremony_audit", "residue.rs", Class::CertificationOnlySupport, Posture::LocalCeremonyAudit, Proof::LocalCeremonyOnly, Action::CertificationOnly),
        primitive("primitive_construction_graph_obligation_audit_sources", "residue.rs", Class::CertificationOnlySupport, Posture::LocalCeremonyAudit, Proof::LocalCeremonyOnly, Action::CertificationOnly),
        primitive_residue("primitive_construction_graph_obligation_residue_manifest", "3 rows max until primitive lane deletes handoff/preflight/family-count residue", "primitive construction still has handoff-only, preflight, and family-cardinality residue", "parallel primitive lane proves execution-backed selection and deletes or narrows every residue class"),
        primitive_residue("primitive_construction_graph_obligation_residue_contract", "3 rows max until primitive lane deletes handoff/preflight/family-count residue", "primitive construction still has handoff-only, preflight, and family-cardinality residue", "parallel primitive lane proves execution-backed selection and deletes or narrows every residue class"),
        primitive("primitive_construction_graph_obligation_selector_precision_matrix", "selector_matrix.rs", Class::CertificationOnlySupport, Posture::SelectorPrecisionCounters, Proof::CounterOnly, Action::CertificationOnly),
        primitive_with_caller("PrimitiveConstructionGraphObligationExecutionMatrixRow", "family_execution_matrix.rs", Class::CertificationOnlySupport, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, "worth-kernel primitive construction execution closeout", Action::CertificationOnly),
        primitive_with_caller("PrimitiveConstructionGraphObligationExecutionMatrixRow::envelope_digest", "family_execution_matrix.rs", Class::CertificationOnlySupport, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, "worth-kernel primitive construction execution closeout", Action::CertificationOnly),
        primitive_with_caller("PrimitiveConstructionGraphObligationExecutionMatrixRow::selected_count", "family_execution_matrix.rs", Class::CertificationOnlySupport, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, "worth-kernel primitive construction execution closeout", Action::CertificationOnly),
        primitive_with_caller("primitive_construction_graph_obligation_execution_matrix", "family_execution_matrix.rs", Class::CertificationOnlySupport, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, "worth-kernel primitive construction execution closeout", Action::CertificationOnly),
        primitive_with_caller("primitive_construction_graph_obligation_replay_pair", "family_execution_matrix.rs", Class::CertificationOnlySupport, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, "worth-kernel primitive construction execution closeout", Action::CertificationOnly),
        primitive_with_caller("primitive_construction_graph_obligation_execution_closeout_passes", "family_execution_matrix.rs", Class::CertificationOnlySupport, Posture::PublicFacadeStatus, Proof::PublicStatusOnly, "worth-kernel primitive construction execution closeout", Action::CertificationOnly),
        primitive_with_caller("ExecutedPrimitiveConstructionGraphAuthorityResult", "result.rs", Class::QueryOwnedSelection, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, "worth-kernel primitive construction result surface", Action::KeepAsQueryOwnedSelection),
        primitive_with_caller("PrimitiveConstructionAcceptedOutcome::graph_obligation_envelope_digest", "outcome.rs", Class::CertificationOnlySupport, Posture::PublicFacadeStatus, Proof::PublicStatusOnly, "worth-kernel primitive construction outcome surface", Action::CertificationOnly),
        primitive_with_caller("PrimitiveConstructionAcceptedOutcome::graph_obligation_selected_count", "outcome.rs", Class::CertificationOnlySupport, Posture::PublicFacadeStatus, Proof::PublicStatusOnly, "worth-kernel primitive construction outcome surface", Action::CertificationOnly),
        primitive_with_caller("ExecutedPrimitiveConstructionGraphAuthorityOutcome", "outcome.rs", Class::QueryOwnedSelection, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, "worth-kernel primitive construction outcome surface", Action::KeepAsQueryOwnedSelection),
        primitive_with_caller("ExecutedPrimitiveConstructionGraphAuthorityOutcome::graph_obligation_envelope_digest", "outcome.rs", Class::QueryOwnedSelection, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, "worth-kernel primitive construction outcome surface", Action::KeepAsQueryOwnedSelection),
        primitive_with_caller("ExecutedPrimitiveConstructionGraphAuthorityOutcome::graph_obligation_selected_count", "outcome.rs", Class::QueryOwnedSelection, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, "worth-kernel primitive construction outcome surface", Action::KeepAsQueryOwnedSelection),
        primitive_with_caller("PrimitiveConstructionQueryDeclarationFamily::orchestration_graph_obligation_registrations", "query_authority/declaration.rs", Class::MigrationProjection, Posture::RegistrationDeclaration, Proof::RegistrationOnly, "worth-kernel primitive construction query declaration family", Action::MigrateToParallelSelectionSubstrate),
    ]
}
