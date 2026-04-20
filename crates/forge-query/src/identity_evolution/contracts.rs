use crate::identity::ResultDigest;

use super::{
    families::LineageTraversalFamily,
    request::IdentityEvolutionComparisonBasisFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum IdentityEvolutionComplexityStatus {
    Verified,
    Debt,
}

impl IdentityEvolutionComplexityStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Debt => "debt",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityEvolutionComplexityContract {
    contract_name: &'static str,
    declared_big_o: &'static str,
    measured_work_basis: &'static str,
    verified_or_debt_status: IdentityEvolutionComplexityStatus,
    forbidden_broadening_clause: &'static str,
}

impl IdentityEvolutionComplexityContract {
    pub fn contract_name(&self) -> &'static str {
        self.contract_name
    }

    pub fn declared_big_o(&self) -> &'static str {
        self.declared_big_o
    }

    pub fn measured_work_basis(&self) -> &'static str {
        self.measured_work_basis
    }

    pub fn verified_or_debt_status(&self) -> IdentityEvolutionComplexityStatus {
        self.verified_or_debt_status
    }

    pub fn forbidden_broadening_clause(&self) -> &'static str {
        self.forbidden_broadening_clause
    }

    pub fn digest(&self) -> ResultDigest {
        ResultDigest::from_parts(&[
            format!("contract_name:{}", self.contract_name),
            format!("declared_big_o:{}", self.declared_big_o),
            format!("measured_work_basis:{}", self.measured_work_basis),
            format!("status:{}", self.verified_or_debt_status.as_str()),
            format!(
                "forbidden_broadening_clause:{}",
                self.forbidden_broadening_clause
            ),
        ])
    }

    pub(crate) fn direct_lineage(family: LineageTraversalFamily) -> Self {
        Self {
            contract_name: family.as_str(),
            declared_big_o: "O(1) adjacency lookup",
            measured_work_basis: "single-anchor direct lineage edge inspection",
            verified_or_debt_status: IdentityEvolutionComplexityStatus::Verified,
            forbidden_broadening_clause:
                "must not widen into recursive traversal, arbitrary breadth expansion, or advisory fallback scanning",
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn correspondence_identity_comparison(
        family: IdentityEvolutionComparisonBasisFamily,
    ) -> Self {
        let contract_name = match family {
            IdentityEvolutionComparisonBasisFamily::BranchToBranch => {
                "branch_to_branch_identity_comparison"
            }
            IdentityEvolutionComparisonBasisFamily::CurrentToHistorical => {
                "current_to_historical_identity_comparison"
            }
            IdentityEvolutionComparisonBasisFamily::HistoricalToHistorical => {
                "historical_to_historical_identity_comparison"
            }
            IdentityEvolutionComparisonBasisFamily::PreviewToAuthoritative => {
                "preview_to_authoritative_identity_comparison"
            }
        };
        Self {
            contract_name,
            declared_big_o: "O(1) basis-paired comparison",
            measured_work_basis: "bounded correspondence metadata comparison between explicit admitted bases",
            verified_or_debt_status: IdentityEvolutionComplexityStatus::Debt,
            forbidden_broadening_clause:
                "must not widen into candidate discovery, collection scans, raw diff payload inspection, or recursive lineage expansion",
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn denied_or_deferred(reason: &'static str) -> Self {
        Self {
            contract_name: reason,
            declared_big_o: "O(1) denial shaping",
            measured_work_basis: "surface-only denial and digest shaping",
            verified_or_debt_status: IdentityEvolutionComplexityStatus::Verified,
            forbidden_broadening_clause:
                "must not auto-broaden into hidden work to rescue an unsupported request",
        }
    }
}
