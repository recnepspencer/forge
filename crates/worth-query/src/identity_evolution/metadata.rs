use crate::identity::{BasisDigest, CanonicalQueryDigest, LineageDigest, ResultDigest};

use super::{
    contracts::{IdentityEvolutionComplexityContract, IdentityEvolutionComplexityStatus},
    families::IdentityEvolutionOutcomeFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BranchLocalityClass {
    BranchLocalOnly,
    CrossBranchAuthoritative,
    CrossBranchDenied,
}

impl BranchLocalityClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BranchLocalOnly => "branch_local_only",
            Self::CrossBranchAuthoritative => "cross_branch_authoritative",
            Self::CrossBranchDenied => "cross_branch_denied",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PromotionOrMergeAuthorityState {
    NotRequired,
    RequiredButUnavailable,
    AuthorityWitnessed,
}

impl PromotionOrMergeAuthorityState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::RequiredButUnavailable => "required_but_unavailable",
            Self::AuthorityWitnessed => "authority_witnessed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityEvolutionComplexityReport {
    contract: IdentityEvolutionComplexityContract,
    status: IdentityEvolutionComplexityStatus,
    measured_work_basis: &'static str,
    forbidden_broadening_clause: &'static str,
    complexity_contract_digest: ResultDigest,
}

impl IdentityEvolutionComplexityReport {
    pub fn contract(&self) -> &IdentityEvolutionComplexityContract {
        &self.contract
    }

    pub fn status(&self) -> IdentityEvolutionComplexityStatus {
        self.status
    }

    pub fn measured_work_basis(&self) -> &'static str {
        self.measured_work_basis
    }

    pub fn forbidden_broadening_clause(&self) -> &'static str {
        self.forbidden_broadening_clause
    }

    pub fn complexity_contract_digest(&self) -> &ResultDigest {
        &self.complexity_contract_digest
    }

    pub(crate) fn from_contract(contract: IdentityEvolutionComplexityContract) -> Self {
        let status = contract.verified_or_debt_status();
        let measured_work_basis = contract.measured_work_basis();
        let forbidden_broadening_clause = contract.forbidden_broadening_clause();
        let complexity_contract_digest = contract.digest();
        Self {
            contract,
            status,
            measured_work_basis,
            forbidden_broadening_clause,
            complexity_contract_digest,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityEvolutionMetadata {
    query_digest: CanonicalQueryDigest,
    basis_digest: BasisDigest,
    lineage_digest: LineageDigest,
    outcome_family: IdentityEvolutionOutcomeFamily,
    anchor_branch_basis_digest: BasisDigest,
    lineage_origin_branch_digest: BasisDigest,
    branch_divergence_root_digest: BasisDigest,
    branch_locality_class: BranchLocalityClass,
    promotion_or_merge_authority_state: PromotionOrMergeAuthorityState,
    complexity_report: IdentityEvolutionComplexityReport,
    branch_locality_digest: ResultDigest,
    metadata_digest: ResultDigest,
}

impl IdentityEvolutionMetadata {
    pub fn query_digest(&self) -> &CanonicalQueryDigest {
        &self.query_digest
    }

    pub fn basis_digest(&self) -> &BasisDigest {
        &self.basis_digest
    }

    pub fn lineage_digest(&self) -> &LineageDigest {
        &self.lineage_digest
    }

    pub fn outcome_family(&self) -> IdentityEvolutionOutcomeFamily {
        self.outcome_family
    }

    pub fn anchor_branch_basis_digest(&self) -> &BasisDigest {
        &self.anchor_branch_basis_digest
    }

    pub fn lineage_origin_branch_digest(&self) -> &BasisDigest {
        &self.lineage_origin_branch_digest
    }

    pub fn branch_divergence_root_digest(&self) -> &BasisDigest {
        &self.branch_divergence_root_digest
    }

    pub fn branch_locality_class(&self) -> BranchLocalityClass {
        self.branch_locality_class
    }

    pub fn promotion_or_merge_authority_state(&self) -> PromotionOrMergeAuthorityState {
        self.promotion_or_merge_authority_state
    }

    pub fn complexity_report(&self) -> &IdentityEvolutionComplexityReport {
        &self.complexity_report
    }

    pub fn branch_locality_digest(&self) -> &ResultDigest {
        &self.branch_locality_digest
    }

    pub fn metadata_digest(&self) -> &ResultDigest {
        &self.metadata_digest
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_parts(
        query_digest: CanonicalQueryDigest,
        basis_digest: BasisDigest,
        lineage_digest: LineageDigest,
        outcome_family: IdentityEvolutionOutcomeFamily,
        anchor_branch_basis_digest: BasisDigest,
        lineage_origin_branch_digest: BasisDigest,
        branch_divergence_root_digest: BasisDigest,
        branch_locality_class: BranchLocalityClass,
        promotion_or_merge_authority_state: PromotionOrMergeAuthorityState,
        complexity_report: IdentityEvolutionComplexityReport,
    ) -> Self {
        let branch_locality_digest = ResultDigest::from_parts(&[
            format!(
                "anchor_branch_basis_digest:{}",
                anchor_branch_basis_digest.as_str()
            ),
            format!(
                "lineage_origin_branch_digest:{}",
                lineage_origin_branch_digest.as_str()
            ),
            format!(
                "branch_divergence_root_digest:{}",
                branch_divergence_root_digest.as_str()
            ),
            format!("branch_locality_class:{}", branch_locality_class.as_str()),
            format!(
                "promotion_or_merge_authority_state:{}",
                promotion_or_merge_authority_state.as_str()
            ),
        ]);
        let metadata_digest = ResultDigest::from_parts(&[
            format!("query_digest:{}", query_digest.as_str()),
            format!("basis_digest:{}", basis_digest.as_str()),
            format!("lineage_digest:{}", lineage_digest.as_str()),
            format!("outcome_family:{}", outcome_family.as_str()),
            format!("branch_locality_digest:{}", branch_locality_digest.as_str()),
            format!(
                "complexity_contract_digest:{}",
                complexity_report.complexity_contract_digest().as_str()
            ),
        ]);
        Self {
            query_digest,
            basis_digest,
            lineage_digest,
            outcome_family,
            anchor_branch_basis_digest,
            lineage_origin_branch_digest,
            branch_divergence_root_digest,
            branch_locality_class,
            promotion_or_merge_authority_state,
            complexity_report,
            branch_locality_digest,
            metadata_digest,
        }
    }
}
