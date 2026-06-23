use crate::query_obligation_selection::boundary_inventory::inventory_record::{
    QuerySelectionAuthorityPosture as Posture, QuerySelectionBoundaryInventoryRow,
    QuerySelectionDeletionAction as Action, QuerySelectionProofStrength as Proof,
    QuerySelectionSurfaceClassification as Class,
};
use crate::query_obligation_selection::boundary_inventory::row_constructors::forge_query;

pub(super) fn rows() -> Vec<QuerySelectionBoundaryInventoryRow> {
    vec![
        forge_query(
            "graph_obligation_consumer_kit",
            "kit.rs",
            Class::QueryOwnedSelection,
            Posture::SelectedObligationExecutionEvidence,
            Proof::ExecutionEnvelope,
            Action::KeepAsQueryOwnedSelection,
        ),
        forge_query(
            "ForgeQueryGraphObligationConsumerRegistrationDeclaration",
            "consumer_declaration.rs",
            Class::QueryOwnedSelection,
            Posture::RegistrationDeclaration,
            Proof::RegistrationOnly,
            Action::KeepAsQueryOwnedSelection,
        ),
        forge_query(
            "ForgeQueryGraphObligationSelectorCoverageDeclaration",
            "selector_coverage.rs",
            Class::QueryOwnedSelection,
            Posture::SelectorCoverageDeclaration,
            Proof::RegistrationOnly,
            Action::KeepAsQueryOwnedSelection,
        ),
        forge_query(
            "ForgeQueryGraphObligationSupportPin",
            "mod.rs",
            Class::CertificationOnlySupport,
            Posture::SupportPin,
            Proof::SupportOnly,
            Action::CertificationOnly,
        ),
        forge_query(
            "ForgeQueryGraphObligationLocalCeremonyAudit",
            "local_ceremony_audit.rs",
            Class::CertificationOnlySupport,
            Posture::LocalCeremonyAudit,
            Proof::LocalCeremonyOnly,
            Action::CertificationOnly,
        ),
        forge_query(
            "ForgeQueryGraphObligationResidueManifest",
            "residue_manifest.rs",
            Class::QueryOwnedSelection,
            Posture::ResidueManifest,
            Proof::ResidueOnly,
            Action::KeepAsQueryOwnedSelection,
        ),
        forge_query(
            "ForgeQueryGraphObligationInMemoryProof",
            "in_memory_proof/mod.rs",
            Class::CertificationOnlySupport,
            Posture::InMemorySelectionAdoption,
            Proof::InMemorySelection,
            Action::CertificationOnly,
        ),
        forge_query(
            "ForgeQueryGraphObligationExecutionBackedAdoptionProof",
            "mod.rs",
            Class::QueryOwnedSelection,
            Posture::ExecutionBackedSelectionAdoption,
            Proof::ExecutionBackedAdoption,
            Action::KeepAsQueryOwnedSelection,
        ),
    ]
}
