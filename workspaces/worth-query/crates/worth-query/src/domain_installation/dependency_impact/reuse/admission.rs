use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::compatibility::WorthQueryCompatibilityUseDenial;
use crate::domain_installation::operation_authority_chain::operation_phase_basis;
use crate::domain_installation::{
    WorthQueryBoundDomainOperation, WorthQueryExecutionSharingWitness,
};

use super::super::compiled::WorthQueryCompiledSemanticAspectDependencyClosure;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDependencyClosureReuseDenial {
    WrongCapabilityPair,
    StaleAuthority,
    StaleConditionalLowering,
    SubjectClosureMismatch,
    CandidateClosureMismatch,
    DependencyDivergence,
}

pub struct WorthQueryDependencyClosureReuseWitness {
    _sharing: WorthQueryExecutionSharingWitness,
    #[allow(dead_code)] // Readmitted by the Phase 19 invalidation consumer.
    subject_affinity:
        crate::domain_installation::operation_authority_chain::WorthQueryOperationAuthorityBasis,
    #[allow(dead_code)] // Readmitted by the Phase 19 invalidation consumer.
    candidate_affinity:
        crate::domain_installation::operation_authority_chain::WorthQueryOperationAuthorityBasis,
    dependency_count: usize,
}

impl WorthQueryDependencyClosureReuseWitness {
    pub const fn dependency_count(&self) -> usize {
        self.dependency_count
    }

    #[allow(dead_code)] // Readmitted by the Phase 19 invalidation consumer.
    pub(crate) fn readmit_for_pair<D, O, F, L: BasisOperationLane>(
        self,
        subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
        candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
        subject_closure: &WorthQueryCompiledSemanticAspectDependencyClosure,
        candidate_closure: &WorthQueryCompiledSemanticAspectDependencyClosure,
    ) -> Result<Self, WorthQueryDependencyClosureReuseDenial> {
        let sharing = self
            ._sharing
            .readmit_for_pair(subject, candidate)
            .map_err(map_basis_denial)?;
        if self.subject_affinity != *operation_phase_basis(subject.authority_proof())
            || self.subject_affinity != subject_closure.affinity
        {
            return Err(WorthQueryDependencyClosureReuseDenial::SubjectClosureMismatch);
        }
        if self.candidate_affinity != *operation_phase_basis(candidate.authority_proof())
            || self.candidate_affinity != candidate_closure.affinity
        {
            return Err(WorthQueryDependencyClosureReuseDenial::CandidateClosureMismatch);
        }
        if self.dependency_count != subject_closure.dependencies().len()
            || !subject_closure.converges_with(candidate_closure)
        {
            return Err(WorthQueryDependencyClosureReuseDenial::DependencyDivergence);
        }
        Ok(Self {
            _sharing: sharing,
            subject_affinity: self.subject_affinity,
            candidate_affinity: self.candidate_affinity,
            dependency_count: self.dependency_count,
        })
    }
}

impl WorthQueryExecutionSharingWitness {
    pub fn admit_dependency_closure_reuse<D, O, F, L: BasisOperationLane>(
        self,
        subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
        candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
        subject_closure: &WorthQueryCompiledSemanticAspectDependencyClosure,
        candidate_closure: &WorthQueryCompiledSemanticAspectDependencyClosure,
    ) -> Result<WorthQueryDependencyClosureReuseWitness, WorthQueryDependencyClosureReuseDenial>
    {
        let witness = self
            .readmit_for_pair(subject, candidate)
            .map_err(map_basis_denial)?;
        if &subject_closure.affinity != operation_phase_basis(subject.authority_proof()) {
            return Err(WorthQueryDependencyClosureReuseDenial::SubjectClosureMismatch);
        }
        if &candidate_closure.affinity != operation_phase_basis(candidate.authority_proof()) {
            return Err(WorthQueryDependencyClosureReuseDenial::CandidateClosureMismatch);
        }
        if !subject_closure.converges_with(candidate_closure) {
            return Err(WorthQueryDependencyClosureReuseDenial::DependencyDivergence);
        }
        Ok(WorthQueryDependencyClosureReuseWitness {
            _sharing: witness,
            subject_affinity: subject_closure.affinity.clone(),
            candidate_affinity: candidate_closure.affinity.clone(),
            dependency_count: subject_closure.dependencies().len(),
        })
    }
}

fn map_basis_denial(
    denial: WorthQueryCompatibilityUseDenial,
) -> WorthQueryDependencyClosureReuseDenial {
    match denial {
        WorthQueryCompatibilityUseDenial::WrongCapabilityPair => {
            WorthQueryDependencyClosureReuseDenial::WrongCapabilityPair
        }
        WorthQueryCompatibilityUseDenial::StaleAuthority => {
            WorthQueryDependencyClosureReuseDenial::StaleAuthority
        }
        WorthQueryCompatibilityUseDenial::StaleConditionalLowering => {
            WorthQueryDependencyClosureReuseDenial::StaleConditionalLowering
        }
    }
}
