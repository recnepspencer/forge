use std::collections::BTreeSet;

use crate::protocols::compaction_visibility::CompactionVisibilityAction;

use super::{
    CompactionVisibilityMappedOwnerCase, CompactionVisibilityOwnerCase,
    CompactionVisibilityOwnerCaseFamily,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompactionVisibilityRefinementCoverageIssue {
    DuplicateOwnerDeclaration(CompactionVisibilityOwnerCase),
    MissingOrdinaryExecution(CompactionVisibilityOwnerCase),
    UnexpectedOrdinaryExecution(CompactionVisibilityOwnerCase),
    DuplicateOrdinaryExecution(CompactionVisibilityOwnerCase),
    MissingModelMapping(CompactionVisibilityOwnerCase),
    UnexpectedModelMapping(CompactionVisibilityOwnerCase),
    DuplicateModelMapping(CompactionVisibilityOwnerCase),
    IncorrectModelMapping {
        owner_case: CompactionVisibilityOwnerCase,
        expected: CompactionVisibilityAction,
        observed: CompactionVisibilityAction,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactionVisibilityRefinementCoverageDenial {
    issues: Vec<CompactionVisibilityRefinementCoverageIssue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactionVisibilityRefinementCoverageReceipt {
    family_coverage: [CompactionVisibilityFamilyCoverage; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactionVisibilityFamilyCoverage {
    declared_owner_cases: usize,
    ordinary_executed_cases: usize,
    mapped_model_actions: usize,
}

pub fn require_compaction_visibility_refinement_coverage(
    declared: impl IntoIterator<Item = CompactionVisibilityOwnerCase>,
    executed: impl IntoIterator<Item = CompactionVisibilityOwnerCase>,
    mapped: impl IntoIterator<Item = CompactionVisibilityMappedOwnerCase>,
) -> Result<
    CompactionVisibilityRefinementCoverageReceipt,
    CompactionVisibilityRefinementCoverageDenial,
> {
    let (declared, declared_duplicates) = collect_cases(declared);
    let (executed, executed_duplicates) = collect_cases(executed);
    let (mapped, mapped_duplicates, incorrect_mappings) = collect_mappings(mapped);
    let issues = refinement_coverage_issues(
        &declared,
        declared_duplicates,
        &executed,
        executed_duplicates,
        &mapped,
        mapped_duplicates,
        incorrect_mappings,
    );

    if issues.is_empty() {
        Ok(CompactionVisibilityRefinementCoverageReceipt {
            family_coverage: CompactionVisibilityOwnerCaseFamily::all().map(|family| {
                CompactionVisibilityFamilyCoverage {
                    declared_owner_cases: count_family(&declared, family),
                    ordinary_executed_cases: count_family(&executed, family),
                    mapped_model_actions: count_family(&mapped, family),
                }
            }),
        })
    } else {
        Err(CompactionVisibilityRefinementCoverageDenial { issues })
    }
}

fn refinement_coverage_issues(
    declared: &BTreeSet<CompactionVisibilityOwnerCase>,
    declared_duplicates: BTreeSet<CompactionVisibilityOwnerCase>,
    executed: &BTreeSet<CompactionVisibilityOwnerCase>,
    executed_duplicates: BTreeSet<CompactionVisibilityOwnerCase>,
    mapped: &BTreeSet<CompactionVisibilityOwnerCase>,
    mapped_duplicates: BTreeSet<CompactionVisibilityOwnerCase>,
    incorrect_mappings: Vec<CompactionVisibilityRefinementCoverageIssue>,
) -> Vec<CompactionVisibilityRefinementCoverageIssue> {
    let mut issues = Vec::new();
    issues.extend(
        declared_duplicates
            .into_iter()
            .map(CompactionVisibilityRefinementCoverageIssue::DuplicateOwnerDeclaration),
    );
    issues.extend(
        declared
            .difference(executed)
            .copied()
            .map(CompactionVisibilityRefinementCoverageIssue::MissingOrdinaryExecution),
    );
    issues.extend(
        executed
            .difference(declared)
            .copied()
            .map(CompactionVisibilityRefinementCoverageIssue::UnexpectedOrdinaryExecution),
    );
    issues.extend(
        executed_duplicates
            .into_iter()
            .map(CompactionVisibilityRefinementCoverageIssue::DuplicateOrdinaryExecution),
    );
    issues.extend(
        declared
            .difference(mapped)
            .copied()
            .map(CompactionVisibilityRefinementCoverageIssue::MissingModelMapping),
    );
    issues.extend(
        mapped
            .difference(declared)
            .copied()
            .map(CompactionVisibilityRefinementCoverageIssue::UnexpectedModelMapping),
    );
    issues.extend(
        mapped_duplicates
            .into_iter()
            .map(CompactionVisibilityRefinementCoverageIssue::DuplicateModelMapping),
    );
    issues.extend(incorrect_mappings);
    issues
}

fn collect_mappings(
    mappings: impl IntoIterator<Item = CompactionVisibilityMappedOwnerCase>,
) -> (
    BTreeSet<CompactionVisibilityOwnerCase>,
    BTreeSet<CompactionVisibilityOwnerCase>,
    Vec<CompactionVisibilityRefinementCoverageIssue>,
) {
    let mappings = mappings.into_iter().collect::<Vec<_>>();
    let (owner_cases, duplicates) = collect_cases(
        mappings
            .iter()
            .copied()
            .map(CompactionVisibilityMappedOwnerCase::owner_case),
    );
    let incorrect = mappings
        .into_iter()
        .filter_map(|mapping| {
            let owner_case = mapping.owner_case();
            let expected = super::correspondence::expected_action_for_owner_case(owner_case);
            let observed = mapping.action();
            (expected != observed).then_some(
                CompactionVisibilityRefinementCoverageIssue::IncorrectModelMapping {
                    owner_case,
                    expected,
                    observed,
                },
            )
        })
        .collect();
    (owner_cases, duplicates, incorrect)
}

fn collect_cases(
    cases: impl IntoIterator<Item = CompactionVisibilityOwnerCase>,
) -> (
    BTreeSet<CompactionVisibilityOwnerCase>,
    BTreeSet<CompactionVisibilityOwnerCase>,
) {
    let mut unique = BTreeSet::new();
    let mut duplicates = BTreeSet::new();
    for case in cases {
        if !unique.insert(case) {
            duplicates.insert(case);
        }
    }
    (unique, duplicates)
}

fn count_family(
    cases: &BTreeSet<CompactionVisibilityOwnerCase>,
    family: CompactionVisibilityOwnerCaseFamily,
) -> usize {
    cases.iter().filter(|case| case.family() == family).count()
}

impl CompactionVisibilityRefinementCoverageDenial {
    pub fn issues(&self) -> &[CompactionVisibilityRefinementCoverageIssue] {
        &self.issues
    }
}

impl CompactionVisibilityRefinementCoverageReceipt {
    pub fn declared_owner_cases(self) -> usize {
        self.family_coverage
            .iter()
            .map(|coverage| coverage.declared_owner_cases)
            .sum()
    }

    pub fn ordinary_executed_cases(self) -> usize {
        self.family_coverage
            .iter()
            .map(|coverage| coverage.ordinary_executed_cases)
            .sum()
    }

    pub fn mapped_model_actions(self) -> usize {
        self.family_coverage
            .iter()
            .map(|coverage| coverage.mapped_model_actions)
            .sum()
    }

    pub const fn family_coverage(
        self,
        family: CompactionVisibilityOwnerCaseFamily,
    ) -> CompactionVisibilityFamilyCoverage {
        self.family_coverage[family.index()]
    }
}

impl CompactionVisibilityFamilyCoverage {
    pub const fn declared_owner_cases(self) -> usize {
        self.declared_owner_cases
    }

    pub const fn ordinary_executed_cases(self) -> usize {
        self.ordinary_executed_cases
    }

    pub const fn mapped_model_actions(self) -> usize {
        self.mapped_model_actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocols::compaction_visibility::CompactionVisibilityAction;
    use worth_store_physical_isolation::CompactionOwnerCaseId;

    #[test]
    fn changed_mapping_edge_fails_the_independent_correspondence_oracle() {
        let owner_case =
            CompactionVisibilityOwnerCase::PhysicalCompaction(CompactionOwnerCaseId::LowerRewrite);
        let mutant = CompactionVisibilityMappedOwnerCase::new(
            owner_case,
            CompactionVisibilityAction::DenyInPlaceOverwrite,
        );

        let denial =
            require_compaction_visibility_refinement_coverage([owner_case], [owner_case], [mutant])
                .expect_err("a changed mapping edge must fail correspondence");

        assert!(denial.issues().contains(
            &CompactionVisibilityRefinementCoverageIssue::IncorrectModelMapping {
                owner_case,
                expected: CompactionVisibilityAction::LowerRewrite,
                observed: CompactionVisibilityAction::DenyInPlaceOverwrite,
            }
        ));
    }

    #[test]
    fn every_side_of_the_three_set_contract_rejects_duplicates() {
        let owner_case =
            CompactionVisibilityOwnerCase::PhysicalCompaction(CompactionOwnerCaseId::LowerRewrite);
        let mapping = crate::protocols::compaction_visibility::map_compaction_case(
            CompactionOwnerCaseId::LowerRewrite,
        );

        let duplicate_declaration = require_compaction_visibility_refinement_coverage(
            [owner_case, owner_case],
            [owner_case],
            [mapping],
        )
        .expect_err("duplicate owner declarations must fail");
        assert!(duplicate_declaration.issues().contains(
            &CompactionVisibilityRefinementCoverageIssue::DuplicateOwnerDeclaration(owner_case)
        ));

        let duplicate_execution = require_compaction_visibility_refinement_coverage(
            [owner_case],
            [owner_case, owner_case],
            [mapping],
        )
        .expect_err("duplicate canonical owner executions must fail");
        assert!(duplicate_execution.issues().contains(
            &CompactionVisibilityRefinementCoverageIssue::DuplicateOrdinaryExecution(owner_case)
        ));

        let duplicate_mapping = require_compaction_visibility_refinement_coverage(
            [owner_case],
            [owner_case],
            [mapping, mapping],
        )
        .expect_err("duplicate model mappings must fail");
        assert!(duplicate_mapping.issues().contains(
            &CompactionVisibilityRefinementCoverageIssue::DuplicateModelMapping(owner_case)
        ));
    }
}
