use crate::index_pages::index_page_integrity_request::DerivedIndexAuthorityEvidence;
use crate::{
    AuthorityDamageBoundary, DerivedDamageClassification, DerivedIndexIntegrityInspectionRequest,
    DerivedRebuildInput, IndeterminatePhysicalDamage, IndexPageIntegrityCounters,
    IndexPageIntegrityDenial, IndexPageIntegrityDenialKind, IndexPageIntegrityReport,
    IntactIndexPageBoundary, ManifestIntegrityDenialKind, ManifestReferenceBasis,
    PhysicalScopeBasis, RebuildabilityPrerequisite, RebuildableDerivedDamage,
    RebuildableDerivedDamagePrerequisites, ScopedPhysicalValidatorInput,
    UnrecoverableAuthorityDamage,
};
use worth_store_physical_format::{
    PhysicalGenerationOwner, PhysicalReferenceScope, PhysicalScopeFamily,
};

const DERIVED_INDEX_SENTINEL: &[u8; 4] = b"DIDX";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DerivedIndexIntegrityAuthority;

impl DerivedIndexIntegrityAuthority {
    pub const fn new() -> Self {
        Self
    }

    pub fn inspect(
        self,
        request: DerivedIndexIntegrityInspectionRequest<'_>,
    ) -> Result<IndexPageIntegrityReport, IndexPageIntegrityDenial> {
        let input = request.input();
        let derived_basis = input.admission().basis().clone();
        let mut counters = IndexPageIntegrityCounters::start().with_index_page_header_check();
        reject_wrong_family(input, counters)?;
        let damaged_index = derived_index_page_is_damaged(input)?;

        match request.authority_evidence() {
            DerivedIndexAuthorityEvidence::Intact(authority_basis) => {
                counters = counters.with_authority_basis_check();
                counters = counters.with_generation_link_check();
                reject_mismatched_authority_root(&derived_basis, authority_basis, counters)?;
                let authority_owner =
                    authority_owner_for_derived_scope(authority_basis, derived_basis.scope())
                        .ok_or_else(|| {
                            missing_generation_link_denial(
                                &derived_basis,
                                authority_basis,
                                counters,
                            )
                        })?;
                if authority_owner.generation() != derived_basis.scope().owner().generation() {
                    return Err(stale_index_generation_denial(
                        &derived_basis,
                        authority_owner,
                        counters,
                    ));
                }
                if damaged_index {
                    let prerequisites = RebuildableDerivedDamagePrerequisites::new(
                        derived_basis.scope(),
                        authority_basis.clone(),
                        authority_owner,
                    );
                    let rebuild_input =
                        DerivedRebuildInput::new(derived_basis.scope(), authority_owner);
                    let damage = RebuildableDerivedDamage::new(
                        derived_basis.scope(),
                        prerequisites,
                        rebuild_input,
                    );
                    let counters = counters
                        .with_rebuildable_classification()
                        .with_skipped_semantic_index_lookup();
                    Ok(IndexPageIntegrityReport::new(
                        derived_basis,
                        DerivedDamageClassification::RebuildableDerived(Box::new(damage)),
                        counters,
                    ))
                } else {
                    let intact_scope = derived_basis.scope();
                    Ok(IndexPageIntegrityReport::new(
                        derived_basis,
                        DerivedDamageClassification::IntactIndexPage(IntactIndexPageBoundary::new(
                            intact_scope,
                        )),
                        counters,
                    ))
                }
            }
            DerivedIndexAuthorityEvidence::Missing => {
                let counters = counters
                    .with_authority_basis_check()
                    .with_indeterminate_classification()
                    .with_skipped_semantic_index_lookup();
                let damage = IndeterminatePhysicalDamage::new(
                    derived_basis.scope(),
                    RebuildabilityPrerequisite::CurrentAuthorityBasis,
                );
                Err(IndexPageIntegrityDenial::new(
                    IndexPageIntegrityDenialKind::MissingAuthorityBasis,
                    counters,
                )
                .with_derived_basis(derived_basis)
                .with_indeterminate(damage))
            }
            DerivedIndexAuthorityEvidence::Damaged(manifest_denial) => {
                let counters = counters
                    .with_authority_basis_check()
                    .with_authority_damage_denial()
                    .with_skipped_semantic_index_lookup();
                let damage = UnrecoverableAuthorityDamage::new(
                    authority_boundary(manifest_denial.kind()),
                    manifest_denial.locality(),
                );
                Err(IndexPageIntegrityDenial::new(
                    IndexPageIntegrityDenialKind::DamagedAuthority,
                    counters,
                )
                .with_derived_basis(derived_basis)
                .with_authority_damage(damage)
                .with_manifest_denial(manifest_denial.clone()))
            }
        }
    }
}

impl Default for DerivedIndexIntegrityAuthority {
    fn default() -> Self {
        Self::new()
    }
}

fn reject_wrong_family(
    input: &ScopedPhysicalValidatorInput<'_>,
    counters: IndexPageIntegrityCounters,
) -> Result<(), IndexPageIntegrityDenial> {
    if input.family() == PhysicalScopeFamily::DerivedIndex {
        return Ok(());
    }
    Err(
        IndexPageIntegrityDenial::new(IndexPageIntegrityDenialKind::WrongPhysicalFamily, counters)
            .with_derived_basis(input.admission().basis().clone()),
    )
}

fn derived_index_page_is_damaged(
    input: &ScopedPhysicalValidatorInput<'_>,
) -> Result<bool, IndexPageIntegrityDenial> {
    let Some(page) = input.admission().checked_page() else {
        return Ok(true);
    };
    Ok(!page
        .checked_bytes()
        .as_bytes()
        .starts_with(DERIVED_INDEX_SENTINEL))
}

fn reject_mismatched_authority_root(
    derived_basis: &PhysicalScopeBasis,
    authority_basis: &ManifestReferenceBasis,
    counters: IndexPageIntegrityCounters,
) -> Result<(), IndexPageIntegrityDenial> {
    let Some(authority_root) = authority_basis.root_owner() else {
        return Err(missing_generation_link_denial(
            derived_basis,
            authority_basis,
            counters,
        ));
    };
    let derived_root = derived_basis.membership().root_owner();
    if authority_root == derived_root {
        return Ok(());
    }
    Err(mismatched_authority_root_denial(
        derived_basis,
        derived_root,
        authority_root,
        counters,
    ))
}

fn authority_owner_for_derived_scope(
    authority_basis: &ManifestReferenceBasis,
    scope: PhysicalReferenceScope,
) -> Option<PhysicalGenerationOwner> {
    authority_basis
        .physical_owners()
        .iter()
        .copied()
        .find(|owner| owner_matches_derived_scope(*owner, scope))
}

fn owner_matches_derived_scope(
    owner: PhysicalGenerationOwner,
    scope: PhysicalReferenceScope,
) -> bool {
    owner.segment_id() == scope.owner().segment_id() && owner.page_id() == scope.owner().page_id()
}

fn missing_generation_link_denial(
    derived_basis: &PhysicalScopeBasis,
    authority_basis: &ManifestReferenceBasis,
    counters: IndexPageIntegrityCounters,
) -> IndexPageIntegrityDenial {
    let prerequisite = if authority_basis.root_owner().is_some() {
        RebuildabilityPrerequisite::GenerationLink
    } else {
        RebuildabilityPrerequisite::CurrentAuthorityBasis
    };
    let damage = IndeterminatePhysicalDamage::new(derived_basis.scope(), prerequisite);
    IndexPageIntegrityDenial::new(
        IndexPageIntegrityDenialKind::MissingGenerationLink,
        counters
            .with_indeterminate_classification()
            .with_skipped_semantic_index_lookup(),
    )
    .with_derived_basis(derived_basis.clone())
    .with_indeterminate(damage)
}

fn mismatched_authority_root_denial(
    derived_basis: &PhysicalScopeBasis,
    expected_root: PhysicalGenerationOwner,
    actual_root: PhysicalGenerationOwner,
    counters: IndexPageIntegrityCounters,
) -> IndexPageIntegrityDenial {
    let damage = IndeterminatePhysicalDamage::new(
        derived_basis.scope(),
        RebuildabilityPrerequisite::ExecutedManifestAuthority,
    );
    IndexPageIntegrityDenial::new(
        IndexPageIntegrityDenialKind::MismatchedAuthorityRoot,
        counters
            .with_indeterminate_classification()
            .with_skipped_semantic_index_lookup(),
    )
    .with_derived_basis(derived_basis.clone())
    .with_expected_actual_owner(expected_root, actual_root)
    .with_indeterminate(damage)
}

fn stale_index_generation_denial(
    derived_basis: &PhysicalScopeBasis,
    expected: PhysicalGenerationOwner,
    counters: IndexPageIntegrityCounters,
) -> IndexPageIntegrityDenial {
    let damage = IndeterminatePhysicalDamage::new(
        derived_basis.scope(),
        RebuildabilityPrerequisite::GenerationLink,
    );
    IndexPageIntegrityDenial::new(
        IndexPageIntegrityDenialKind::StaleIndexGeneration,
        counters
            .with_indeterminate_classification()
            .with_skipped_semantic_index_lookup(),
    )
    .with_derived_basis(derived_basis.clone())
    .with_expected_actual_owner(expected, derived_basis.scope().owner())
    .with_indeterminate(damage)
}

fn authority_boundary(kind: ManifestIntegrityDenialKind) -> AuthorityDamageBoundary {
    match kind {
        ManifestIntegrityDenialKind::MissingRootPage
        | ManifestIntegrityDenialKind::DamagedRoot
        | ManifestIntegrityDenialKind::TornRootPointer
        | ManifestIntegrityDenialKind::MultipleValidRoots
        | ManifestIntegrityDenialKind::RootGenerationMismatch
        | ManifestIntegrityDenialKind::ResidueRootRejected
        | ManifestIntegrityDenialKind::RecoveryBlockingRootDamage => {
            AuthorityDamageBoundary::RootManifest
        }
        ManifestIntegrityDenialKind::WrongSegmentId
        | ManifestIntegrityDenialKind::StaleManifestGeneration => {
            AuthorityDamageBoundary::SegmentManifest
        }
        ManifestIntegrityDenialKind::MismatchedExtentId
        | ManifestIntegrityDenialKind::SourcePrecedenceViolation => {
            AuthorityDamageBoundary::ManifestReferenceTable
        }
        ManifestIntegrityDenialKind::DamagedAllocationMap => AuthorityDamageBoundary::AllocationMap,
        ManifestIntegrityDenialKind::BackendResidueFallback => {
            AuthorityDamageBoundary::BackendResidue
        }
    }
}
