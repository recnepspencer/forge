use super::{LayoutOwnerCaseDeclarations, LayoutOwnerFamily, LayoutOwnerObservationLedger};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutOwnerCoverageIssue {
    Missing {
        family: LayoutOwnerFamily,
        case: &'static str,
    },
    Unexpected {
        family: LayoutOwnerFamily,
        case: &'static str,
    },
    Duplicate {
        family: LayoutOwnerFamily,
        case: &'static str,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct LayoutOwnerCoverageDenial {
    issues: Vec<LayoutOwnerCoverageIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutOwnerCoverageReceipt {
    observed_cases: BTreeMap<LayoutOwnerFamily, BTreeSet<&'static str>>,
    owner_case_count: usize,
    executed_evidence: crate::courtroom::layout::executed_evidence::LayoutExecutedEvidenceReceipt,
}

impl LayoutOwnerCoverageReceipt {
    pub fn covers(&self, family: LayoutOwnerFamily) -> bool {
        self.observed_cases.contains_key(&family)
    }

    pub fn observed_case(&self, family: LayoutOwnerFamily, case: &'static str) -> bool {
        self.observed_cases
            .get(&family)
            .is_some_and(|cases| cases.contains(case))
    }

    pub fn owner_family_count(&self) -> usize {
        self.observed_cases.len()
    }

    pub const fn owner_case_count(&self) -> usize {
        self.owner_case_count
    }

    pub(crate) fn families(&self) -> impl Iterator<Item = LayoutOwnerFamily> + '_ {
        self.observed_cases.keys().copied()
    }

    pub(crate) fn cases(&self, family: LayoutOwnerFamily) -> &BTreeSet<&'static str> {
        self.observed_cases
            .get(&family)
            .expect("certified owner family must retain its observed cases")
    }

    pub(crate) const fn executed_evidence(
        &self,
    ) -> &crate::courtroom::layout::executed_evidence::LayoutExecutedEvidenceReceipt {
        &self.executed_evidence
    }
}

impl LayoutOwnerCoverageDenial {
    pub fn issues(&self) -> &[LayoutOwnerCoverageIssue] {
        &self.issues
    }
}

pub fn require_exact_owner_case_coverage(
    declarations: &LayoutOwnerCaseDeclarations,
    observations: &LayoutOwnerObservationLedger,
) -> Result<(), LayoutOwnerCoverageDenial> {
    require_exact_owner_family_coverage(declarations, observations, declarations.families())
}

pub fn certify_exact_owner_case_coverage(
    declarations: &LayoutOwnerCaseDeclarations,
    observations: &LayoutOwnerObservationLedger,
) -> Result<LayoutOwnerCoverageReceipt, LayoutOwnerCoverageDenial> {
    require_exact_owner_case_coverage(declarations, observations)?;
    let observed_cases = declarations
        .families()
        .map(|family| (family, observations.observed(family)))
        .collect::<BTreeMap<_, _>>();
    let owner_case_count = observed_cases.values().map(BTreeSet::len).sum();
    Ok(LayoutOwnerCoverageReceipt {
        observed_cases,
        owner_case_count,
        executed_evidence: observations.executed_evidence().clone(),
    })
}

pub fn require_exact_owner_family_coverage(
    declarations: &LayoutOwnerCaseDeclarations,
    observations: &LayoutOwnerObservationLedger,
    families: impl IntoIterator<Item = LayoutOwnerFamily>,
) -> Result<(), LayoutOwnerCoverageDenial> {
    let mut issues = Vec::new();

    for family in families {
        let declared = declarations.cases(family);
        let observed = observations.observed(family);

        issues.extend(
            declared
                .difference(&observed)
                .copied()
                .map(|case| LayoutOwnerCoverageIssue::Missing { family, case }),
        );
        issues.extend(
            observed
                .difference(declared)
                .copied()
                .map(|case| LayoutOwnerCoverageIssue::Unexpected { family, case }),
        );
    }

    issues.extend(
        observations
            .duplicates()
            .map(|(family, case)| LayoutOwnerCoverageIssue::Duplicate { family, case }),
    );

    if issues.is_empty() {
        Ok(())
    } else {
        Err(LayoutOwnerCoverageDenial { issues })
    }
}
