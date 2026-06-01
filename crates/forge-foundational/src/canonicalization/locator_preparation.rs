use forge_proof::TransitionOutcome;

use crate::aspects::{AspectKey, CanonicalFieldPath};
use crate::locators::{
    AspectContractLocator, AspectFieldLocator, AspectLocator, AspectMaskLocator,
    BoundaryArtifactField, BoundaryArtifactLocator, BoundaryMismatchLocator, BoundarySourceLocator,
    FoundationalTransitionLocator, LocatorAuthority,
};
use crate::transitions::FoundationalCommitParentBasis;

use super::{
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact, CanonicalBasisValue,
    CanonicalIntegerWidth, CanonicalizationRuleVersion,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalLocatorInput {
    BoundaryArtifact(BoundaryArtifactLocator),
    Aspect(AspectLocator),
    AspectField(AspectFieldLocator),
    AspectContract(AspectContractLocator),
    Source(BoundarySourceLocator),
    Mismatch(BoundaryMismatchLocator),
    Transition(FoundationalTransitionLocator),
}

pub fn prepare_locator_for_canonical_basis(
    version: CanonicalizationRuleVersion,
    locator: CanonicalLocatorInput,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, super::CanonicalBasisConstructionDenial> {
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Locator,
        canonical_locator_entries(locator),
    )
}

pub fn locator_canonical_basis_entries(
    ready: &CanonicalBasisReadyArtifact,
) -> &[CanonicalBasisEntry] {
    ready.payload().entries()
}

fn canonical_locator_entries(locator: CanonicalLocatorInput) -> Vec<CanonicalBasisEntry> {
    match locator {
        CanonicalLocatorInput::BoundaryArtifact(locator) => {
            boundary_artifact_locator_entries("boundary_artifact", locator)
        }
        CanonicalLocatorInput::Aspect(locator) => aspect_locator_entries("aspect", &locator),
        CanonicalLocatorInput::AspectField(locator) => {
            aspect_field_locator_entries("aspect_field", &locator)
        }
        CanonicalLocatorInput::AspectContract(locator) => {
            vec![
                locator_text_entry("aspect_contract.kind", "aspect_contract"),
                aspect_key_entry("aspect_contract.aspect_key", locator.aspect_key()),
            ]
        }
        CanonicalLocatorInput::Source(locator) => match locator {
            BoundarySourceLocator::Aspect(locator) => {
                aspect_locator_entries("source.aspect", &locator)
            }
            BoundarySourceLocator::AspectField(locator) => {
                aspect_field_locator_entries("source.aspect_field", &locator)
            }
            BoundarySourceLocator::BoundaryArtifact(locator) => {
                boundary_artifact_locator_entries("source.boundary_artifact", locator)
            }
        },
        CanonicalLocatorInput::Mismatch(locator) => match locator {
            BoundaryMismatchLocator::Aspect(locator) => {
                aspect_locator_entries("mismatch.aspect", &locator)
            }
            BoundaryMismatchLocator::AspectField(locator) => {
                aspect_field_locator_entries("mismatch.aspect_field", &locator)
            }
            BoundaryMismatchLocator::BoundaryArtifact(locator) => {
                boundary_artifact_locator_entries("mismatch.boundary_artifact", locator)
            }
        },
        CanonicalLocatorInput::Transition(locator) => transition_locator_entries(locator),
    }
}

fn boundary_artifact_locator_entries(
    prefix: &'static str,
    locator: BoundaryArtifactLocator,
) -> Vec<CanonicalBasisEntry> {
    vec![
        locator_text_entry(concat_locus(prefix, "kind"), "boundary_artifact"),
        locator_integer_entry(
            concat_locus(prefix, "artifact_id"),
            u128::from(locator.artifact_id().get()),
        ),
        locator_text_entry(
            concat_locus(prefix, "field"),
            boundary_artifact_field_name(locator.field()),
        ),
    ]
}

fn aspect_locator_entries(
    prefix: &'static str,
    locator: &AspectLocator,
) -> Vec<CanonicalBasisEntry> {
    vec![
        locator_text_entry(concat_locus(prefix, "kind"), "aspect"),
        locator_text_entry(
            concat_locus(prefix, "authority"),
            locator_authority_name(locator.authority()),
        ),
        aspect_key_entry(concat_locus(prefix, "aspect_key"), locator.aspect_key()),
    ]
}

fn aspect_field_locator_entries(
    prefix: &'static str,
    locator: &AspectFieldLocator,
) -> Vec<CanonicalBasisEntry> {
    let mut entries = aspect_locator_entries(prefix, locator.aspect());
    entries.push(field_path_entry(
        concat_locus(prefix, "field_path"),
        locator.field_path(),
    ));
    entries
}

fn aspect_key_entry(locus: impl Into<String>, key: &AspectKey) -> CanonicalBasisEntry {
    locator_text_entry(locus, key.as_str())
}

fn field_path_entry(locus: impl Into<String>, path: &CanonicalFieldPath) -> CanonicalBasisEntry {
    let value = path
        .fields()
        .iter()
        .map(|field| field.as_str())
        .collect::<Vec<_>>()
        .join(".");

    locator_text_entry(locus, value)
}

fn locator_text_entry(locus: impl Into<String>, value: impl Into<String>) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Locator,
        CanonicalBasisLocus::Named(locus.into().into()),
        CanonicalBasisEntryKind::Locator,
        CanonicalBasisValue::ExactText(value.into().into()),
    )
}

fn locator_integer_entry(locus: impl Into<String>, value: u128) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Locator,
        CanonicalBasisLocus::Named(locus.into().into()),
        CanonicalBasisEntryKind::Locator,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value,
        },
    )
}

fn boundary_artifact_field_name(field: BoundaryArtifactField) -> &'static str {
    match field {
        BoundaryArtifactField::Payload => "payload",
        BoundaryArtifactField::Proofs => "proofs",
        BoundaryArtifactField::Basis => "basis",
    }
}

fn locator_authority_name(authority: LocatorAuthority) -> &'static str {
    match authority {
        LocatorAuthority::Authoritative => "authoritative",
        LocatorAuthority::Derived => "derived",
        LocatorAuthority::Projected => "projected",
        LocatorAuthority::SupportOnly => "support_only",
        LocatorAuthority::Planned => "planned",
        LocatorAuthority::ReceiptBearing => "receipt_bearing",
    }
}

fn concat_locus(prefix: &str, suffix: &str) -> String {
    format!("{prefix}.{suffix}")
}

pub fn projection_mask_locator_canonical_basis_entries(
    locator: &AspectMaskLocator<crate::aspects::ProjectionMask>,
) -> Vec<CanonicalBasisEntry> {
    mask_locator_entries(
        "projection_mask",
        locator.authority(),
        locator.aspect_key(),
        locator.paths(),
    )
}

pub fn mutation_mask_locator_canonical_basis_entries(
    locator: &AspectMaskLocator<crate::aspects::MutationMask>,
) -> Vec<CanonicalBasisEntry> {
    mask_locator_entries(
        "mutation_mask",
        locator.authority(),
        locator.aspect_key(),
        locator.paths(),
    )
}

pub fn diagnostic_mask_locator_canonical_basis_entries(
    locator: &AspectMaskLocator<crate::aspects::DiagnosticMask>,
) -> Vec<CanonicalBasisEntry> {
    mask_locator_entries(
        "diagnostic_mask",
        locator.authority(),
        locator.aspect_key(),
        locator.paths(),
    )
}

fn mask_locator_entries(
    prefix: &'static str,
    authority: LocatorAuthority,
    aspect_key: &AspectKey,
    paths: &[CanonicalFieldPath],
) -> Vec<CanonicalBasisEntry> {
    let mut entries = vec![
        locator_text_entry(concat_locus(prefix, "kind"), prefix),
        locator_text_entry(
            concat_locus(prefix, "authority"),
            locator_authority_name(authority),
        ),
        aspect_key_entry(concat_locus(prefix, "aspect_key"), aspect_key),
    ];

    entries.extend(
        paths
            .iter()
            .enumerate()
            .map(|(index, path)| field_path_entry(format!("{prefix}.path.{index}"), path)),
    );
    entries
}

fn transition_locator_entries(locator: FoundationalTransitionLocator) -> Vec<CanonicalBasisEntry> {
    match locator {
        FoundationalTransitionLocator::BranchCandidate(locator) => vec![
            transition_locator_text_entry("transition.branch_candidate.kind", "branch-candidate"),
            transition_locator_text_entry(
                "transition.branch_candidate.branch_id",
                locator.branch_id().as_str(),
            ),
            transition_locator_integer_entry(
                "transition.branch_candidate.candidate_id",
                u128::from(locator.candidate_id().handle().get()),
            ),
        ],
        FoundationalTransitionLocator::MergeConflict(locator) => vec![
            transition_locator_text_entry("transition.merge_conflict.kind", "merge-conflict"),
            transition_locator_text_entry(
                "transition.merge_conflict.source_branch",
                locator.source_branch().as_str(),
            ),
            transition_locator_text_entry(
                "transition.merge_conflict.target_branch",
                locator.target_branch().as_str(),
            ),
            transition_locator_text_entry(
                "transition.merge_conflict.category",
                locator.conflict_locus().category(),
            ),
            transition_locator_text_entry(
                "transition.merge_conflict.source_detail",
                locator.conflict_locus().source_detail(),
            ),
            transition_locator_text_entry(
                "transition.merge_conflict.target_detail",
                locator.conflict_locus().target_detail(),
            ),
        ],
        FoundationalTransitionLocator::CommitParentage(locator) => vec![
            transition_locator_text_entry("transition.parentage.kind", "commit-parentage"),
            transition_locator_integer_entry(
                "transition.parentage.commit_id",
                u128::from(locator.commit_id().handle().get()),
            ),
            parent_basis_locator_entry("transition.parentage.parent_basis", locator.parent_basis()),
        ],
        FoundationalTransitionLocator::CommittedDelta(locator) => vec![
            transition_locator_text_entry("transition.delta.kind", "committed-delta"),
            transition_locator_integer_entry(
                "transition.delta.commit_id",
                u128::from(locator.commit_id().handle().get()),
            ),
            transition_locator_text_entry(
                "transition.delta.category",
                locator.delta_locus().category(),
            ),
            transition_locator_text_entry(
                "transition.delta.detail",
                locator.delta_locus().detail(),
            ),
        ],
        FoundationalTransitionLocator::MergeScope(locator) => vec![
            transition_locator_text_entry("transition.merge_scope.kind", "merge-scope"),
            transition_locator_text_entry(
                "transition.merge_scope.source_branch",
                locator.source_branch().as_str(),
            ),
            transition_locator_text_entry(
                "transition.merge_scope.target_branch",
                locator.target_branch().as_str(),
            ),
            transition_locator_text_entry(
                "transition.merge_scope.family",
                merge_scope_family_name(locator.scope_family()),
            ),
        ],
        FoundationalTransitionLocator::SelectedNodeScope(locator) => vec![
            transition_locator_text_entry(
                "transition.selected_node_scope.kind",
                "selected-node-scope",
            ),
            transition_locator_text_entry(
                "transition.selected_node_scope.source_branch",
                locator.source_branch().as_str(),
            ),
            transition_locator_text_entry(
                "transition.selected_node_scope.target_branch",
                locator.target_branch().as_str(),
            ),
            transition_locator_text_entry(
                "transition.selected_node_scope.node",
                locator.selected_node().as_str(),
            ),
        ],
        FoundationalTransitionLocator::SelectedAspectScope(locator) => vec![
            transition_locator_text_entry(
                "transition.selected_aspect_scope.kind",
                "selected-aspect-scope",
            ),
            transition_locator_text_entry(
                "transition.selected_aspect_scope.source_branch",
                locator.source_branch().as_str(),
            ),
            transition_locator_text_entry(
                "transition.selected_aspect_scope.target_branch",
                locator.target_branch().as_str(),
            ),
            transition_locator_text_entry(
                "transition.selected_aspect_scope.node",
                locator.selected_aspect().node().as_str(),
            ),
            transition_locator_text_entry(
                "transition.selected_aspect_scope.aspect",
                locator.selected_aspect().aspect().as_str(),
            ),
        ],
    }
}

fn transition_locator_text_entry(
    locus: impl Into<String>,
    value: impl Into<String>,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Locator,
        CanonicalBasisLocus::Named(locus.into().into()),
        CanonicalBasisEntryKind::TransitionLocator,
        CanonicalBasisValue::ExactText(value.into().into()),
    )
}

fn transition_locator_integer_entry(locus: impl Into<String>, value: u128) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Locator,
        CanonicalBasisLocus::Named(locus.into().into()),
        CanonicalBasisEntryKind::TransitionLocator,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value,
        },
    )
}

fn parent_basis_locator_entry(
    locus: impl Into<String>,
    basis: FoundationalCommitParentBasis,
) -> CanonicalBasisEntry {
    transition_locator_integer_entry(locus, u128::from(basis.basis_id().get()))
}

fn merge_scope_family_name(
    family: crate::transitions::FoundationalMergeScopeFamily,
) -> &'static str {
    match family {
        crate::transitions::FoundationalMergeScopeFamily::FullBranch => "full-branch",
        crate::transitions::FoundationalMergeScopeFamily::SelectedNodes => "selected-nodes",
        crate::transitions::FoundationalMergeScopeFamily::SelectedAspects => "selected-aspects",
    }
}
