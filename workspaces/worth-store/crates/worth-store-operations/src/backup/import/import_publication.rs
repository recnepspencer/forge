use worth_proof::{DenialTransitionOutcome, TransitionOutcome};
use worth_store_authority::{StoreCurrentAuthorityIdentity, StoreCurrentAuthorityWitness};
use worth_store_layout_indexes::materialization::{
    AdmittedLayoutMaterialization, LayoutMaterializationSourceKind,
};
use worth_store_physical_isolation::{
    CopyOnWritePublicationBinding, CopyOnWritePublicationPlan, ReadCopyUpdateRootPublication,
};

pub type ImportPublicationReadinessOutcome =
    DenialTransitionOutcome<ImportPublicationReadiness, ImportPublicationDenial>;
pub type ImportPublicationCompletionOutcome =
    DenialTransitionOutcome<PublishedImportedLayout, ImportPublicationDenial>;

#[derive(Debug, Clone)]
pub struct ImportPublicationReadiness {
    materialization: AdmittedLayoutMaterialization,
    physical_binding: CopyOnWritePublicationBinding,
    authority_identity: StoreCurrentAuthorityIdentity,
}

#[derive(Debug, Clone)]
pub struct PublishedImportedLayout {
    materialization: AdmittedLayoutMaterialization,
    physical_binding: CopyOnWritePublicationBinding,
    physical_publication: ReadCopyUpdateRootPublication,
    authority_identity: StoreCurrentAuthorityIdentity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportPublicationDenial {
    RestoredArtifactMaterializationRequired,
    CurrentStoreAuthorityMismatch,
    PhysicalPublicationBindingMismatch,
}

pub fn admit_import_publication_readiness(
    materialization: AdmittedLayoutMaterialization,
    plan: &CopyOnWritePublicationPlan,
    current_authority: &StoreCurrentAuthorityWitness,
) -> ImportPublicationReadinessOutcome {
    if !matches!(
        materialization.source().kind(),
        LayoutMaterializationSourceKind::RestoredArtifact(_)
    ) {
        return TransitionOutcome::denied(
            ImportPublicationDenial::RestoredArtifactMaterializationRequired,
        );
    }
    let authority_identity = current_authority.authority_identity();
    let physical_binding = plan.binding();
    if materialization.family().authority_identity() != authority_identity
        || physical_binding.store_authority_identity() != authority_identity
    {
        return TransitionOutcome::denied(ImportPublicationDenial::CurrentStoreAuthorityMismatch);
    }
    TransitionOutcome::success(ImportPublicationReadiness {
        materialization,
        physical_binding,
        authority_identity,
    })
}

pub fn complete_import_publication(
    readiness: ImportPublicationReadiness,
    publication: ReadCopyUpdateRootPublication,
) -> ImportPublicationCompletionOutcome {
    if !publication_matches_binding(&publication, readiness.physical_binding) {
        return TransitionOutcome::denied(
            ImportPublicationDenial::PhysicalPublicationBindingMismatch,
        );
    }
    TransitionOutcome::success(PublishedImportedLayout {
        materialization: readiness.materialization,
        physical_binding: readiness.physical_binding,
        physical_publication: publication,
        authority_identity: readiness.authority_identity,
    })
}

fn publication_matches_binding(
    publication: &ReadCopyUpdateRootPublication,
    binding: CopyOnWritePublicationBinding,
) -> bool {
    let receipt = publication.receipt();
    receipt.old_root() == binding.old_root()
        && receipt.new_root() == binding.new_root()
        && receipt.old_root_validation() == binding.old_root_validation()
        && receipt.new_root_validation() == binding.new_root_validation()
        && receipt.new_root().store_authority_identity() == binding.store_authority_identity()
}

impl ImportPublicationReadiness {
    pub const fn materialization(&self) -> &AdmittedLayoutMaterialization {
        &self.materialization
    }

    pub const fn physical_binding(&self) -> CopyOnWritePublicationBinding {
        self.physical_binding
    }

    pub const fn authority_identity(&self) -> &StoreCurrentAuthorityIdentity {
        &self.authority_identity
    }
}

impl PublishedImportedLayout {
    pub const fn materialization(&self) -> &AdmittedLayoutMaterialization {
        &self.materialization
    }

    pub const fn physical_publication(&self) -> &ReadCopyUpdateRootPublication {
        &self.physical_publication
    }

    pub const fn physical_binding(&self) -> CopyOnWritePublicationBinding {
        self.physical_binding
    }

    pub const fn authority_identity(&self) -> &StoreCurrentAuthorityIdentity {
        &self.authority_identity
    }
}
