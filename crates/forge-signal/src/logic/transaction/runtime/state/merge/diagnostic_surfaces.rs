use forge_foundational::facade::{
    prepare_locator_for_canonical_basis, prepare_scoped_merge_diagnostic_explanation,
    CanonicalBasisConstructionDenial, CanonicalBasisReadyArtifact, CanonicalLocatorInput,
    CanonicalizationRuleVersion, FoundationalDiagnosticExplanationInput,
    FoundationalScopedMergeDiagnosticInput,
};
use forge_proof::TransitionOutcome;

use crate::logic::transaction::runtime::{
    BranchMergeRequestScopeFamily, BranchMergeScopedDenialFailureEvidence,
    BranchMergeScopedUnavailableFailureEvidence, ScopedMergeProofPacket,
};
use crate::state::SignalBranchId;

use super::canonical_basis::{foundational_denial_evidence, foundational_unavailable_posture};
use super::locator::foundational_branch_id_from_runtime_id;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalScopedMergeCanonicalLocatorBundle {
    scope: CanonicalBasisReadyArtifact,
    requested: Vec<CanonicalBasisReadyArtifact>,
    admitted: Vec<CanonicalBasisReadyArtifact>,
    skipped: Vec<CanonicalBasisReadyArtifact>,
    no_op: Vec<CanonicalBasisReadyArtifact>,
    support_closure: Vec<CanonicalBasisReadyArtifact>,
}

impl SignalScopedMergeCanonicalLocatorBundle {
    pub fn scope(&self) -> &CanonicalBasisReadyArtifact {
        &self.scope
    }

    pub fn requested(&self) -> &[CanonicalBasisReadyArtifact] {
        &self.requested
    }

    pub fn admitted(&self) -> &[CanonicalBasisReadyArtifact] {
        &self.admitted
    }

    pub fn skipped(&self) -> &[CanonicalBasisReadyArtifact] {
        &self.skipped
    }

    pub fn no_op(&self) -> &[CanonicalBasisReadyArtifact] {
        &self.no_op
    }

    pub fn support_closure(&self) -> &[CanonicalBasisReadyArtifact] {
        &self.support_closure
    }
}

impl ScopedMergeProofPacket {
    pub fn prepare_locator_canonical_basis_bundle(
        &self,
        version: CanonicalizationRuleVersion,
        source_branch_id: SignalBranchId,
        target_branch_id: SignalBranchId,
    ) -> TransitionOutcome<SignalScopedMergeCanonicalLocatorBundle, CanonicalBasisConstructionDenial>
    {
        let bundle = self.locator_bundle(source_branch_id, target_branch_id);
        let scope = match lower_locator(version.clone(), bundle.scope().clone()) {
            Ok(scope) => scope,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        let requested = match lower_locators(version.clone(), bundle.requested()) {
            Ok(requested) => requested,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        let admitted = match lower_locators(version.clone(), bundle.admitted()) {
            Ok(admitted) => admitted,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        let skipped = match lower_locators(version.clone(), bundle.skipped()) {
            Ok(skipped) => skipped,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        let no_op = match lower_locators(version.clone(), bundle.no_op()) {
            Ok(no_op) => no_op,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        let support_closure = match lower_locators(version, bundle.support_closure()) {
            Ok(support_closure) => support_closure,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        TransitionOutcome::success(SignalScopedMergeCanonicalLocatorBundle {
            scope,
            requested,
            admitted,
            skipped,
            no_op,
            support_closure,
        })
    }

    pub fn prepare_request_diagnostic_explanation(
        &self,
        source_branch_id: SignalBranchId,
        target_branch_id: SignalBranchId,
    ) -> FoundationalDiagnosticExplanationInput {
        prepare_scoped_merge_diagnostic_explanation(
            FoundationalScopedMergeDiagnosticInput::ScopeRequest {
                source_branch: foundational_branch_id_from_runtime_id(source_branch_id),
                target_branch: foundational_branch_id_from_runtime_id(target_branch_id),
                requested_scope: foundational_request_scope(self),
            },
        )
    }
}

impl BranchMergeScopedDenialFailureEvidence {
    pub fn prepare_diagnostic_explanation(
        &self,
        source_branch_id: SignalBranchId,
        target_branch_id: SignalBranchId,
    ) -> FoundationalDiagnosticExplanationInput {
        prepare_scoped_merge_diagnostic_explanation(FoundationalScopedMergeDiagnosticInput::DeniedScope(
            foundational_denial_evidence(self, source_branch_id, target_branch_id)
                .expect("retained scoped denial evidence should lower into foundational explanation input"),
        ))
    }
}

impl BranchMergeScopedUnavailableFailureEvidence {
    pub fn prepare_diagnostic_explanation(
        &self,
        source_branch_id: SignalBranchId,
        target_branch_id: SignalBranchId,
    ) -> FoundationalDiagnosticExplanationInput {
        prepare_scoped_merge_diagnostic_explanation(
            FoundationalScopedMergeDiagnosticInput::UnavailableScope(
                foundational_unavailable_posture(self, source_branch_id, target_branch_id)
                    .expect(
                        "retained scoped unavailable evidence should lower into foundational explanation input",
                    ),
            ),
        )
    }
}

fn foundational_request_scope(
    proof: &ScopedMergeProofPacket,
) -> forge_foundational::facade::FoundationalMergeScope {
    match proof.scope_family() {
        BranchMergeRequestScopeFamily::FullBranch => {
            forge_foundational::facade::FoundationalMergeScope::full_branch()
        }
        BranchMergeRequestScopeFamily::SelectedNodes => {
            forge_foundational::facade::FoundationalMergeScope::selected_nodes(
                proof
                    .requested_nodes()
                    .iter()
                    .copied()
                    .map(super::foundational_scope::foundational_denied_node_locus)
                    .collect::<Vec<_>>(),
            )
            .expect("retained selected-node request should be canonicalizable")
        }
        BranchMergeRequestScopeFamily::SelectedAspects => {
            forge_foundational::facade::FoundationalMergeScope::selected_aspects(
                proof
                    .requested_aspects()
                    .iter()
                    .map(super::foundational_scope::foundational_denied_aspect_locus)
                    .collect::<Vec<_>>(),
            )
            .expect("retained selected-aspect request should be canonicalizable")
        }
    }
}

fn lower_locators(
    version: CanonicalizationRuleVersion,
    locators: &[forge_foundational::facade::FoundationalTransitionLocator],
) -> Result<Vec<CanonicalBasisReadyArtifact>, CanonicalBasisConstructionDenial> {
    locators
        .iter()
        .cloned()
        .map(|locator| lower_locator(version.clone(), locator))
        .collect()
}

fn lower_locator(
    version: CanonicalizationRuleVersion,
    locator: forge_foundational::facade::FoundationalTransitionLocator,
) -> Result<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
    match prepare_locator_for_canonical_basis(version, CanonicalLocatorInput::Transition(locator)) {
        TransitionOutcome::Success(ready) => Ok(ready),
        TransitionOutcome::Denied(denial) => Err(denial),
        other => unreachable!("canonical locator lowering should not produce {other:?}"),
    }
}
