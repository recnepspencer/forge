use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExactProtocolRefinementCoverageReceipt {
    declared_owner_cases: u64,
    ordinary_executed_cases: u64,
    mapped_model_actions: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolRefinementCoverageDenial {
    EmptyOwnerInventory,
    DuplicateOwnerDeclaration,
    DuplicateModelMapping,
    MissingOrdinaryExecution,
    UnexpectedOrdinaryExecution,
    MissingModelMapping,
    UnexpectedModelMapping,
}

pub fn require_exact_protocol_refinement_coverage<K: Ord>(
    declared: impl IntoIterator<Item = K>,
    executed: impl IntoIterator<Item = K>,
    mapped: impl IntoIterator<Item = K>,
) -> Result<ExactProtocolRefinementCoverageReceipt, ProtocolRefinementCoverageDenial> {
    let (declared, declared_count) = collect(declared);
    let (executed, _) = collect(executed);
    let (mapped, mapped_count) = collect(mapped);
    if declared.is_empty() {
        return Err(ProtocolRefinementCoverageDenial::EmptyOwnerInventory);
    }
    if declared.len() != declared_count {
        return Err(ProtocolRefinementCoverageDenial::DuplicateOwnerDeclaration);
    }
    if mapped.len() != mapped_count {
        return Err(ProtocolRefinementCoverageDenial::DuplicateModelMapping);
    }
    require_equal(
        &declared,
        &executed,
        ProtocolRefinementCoverageDenial::MissingOrdinaryExecution,
        ProtocolRefinementCoverageDenial::UnexpectedOrdinaryExecution,
    )?;
    require_equal(
        &declared,
        &mapped,
        ProtocolRefinementCoverageDenial::MissingModelMapping,
        ProtocolRefinementCoverageDenial::UnexpectedModelMapping,
    )?;
    let count = declared.len() as u64;
    Ok(ExactProtocolRefinementCoverageReceipt {
        declared_owner_cases: count,
        ordinary_executed_cases: count,
        mapped_model_actions: count,
    })
}

fn collect<K: Ord>(values: impl IntoIterator<Item = K>) -> (BTreeSet<K>, usize) {
    let values = values.into_iter().collect::<Vec<_>>();
    let count = values.len();
    (values.into_iter().collect(), count)
}

fn require_equal<K: Ord>(
    expected: &BTreeSet<K>,
    observed: &BTreeSet<K>,
    missing: ProtocolRefinementCoverageDenial,
    unexpected: ProtocolRefinementCoverageDenial,
) -> Result<(), ProtocolRefinementCoverageDenial> {
    if !expected.is_subset(observed) {
        return Err(missing);
    }
    if !observed.is_subset(expected) {
        return Err(unexpected);
    }
    Ok(())
}

impl ExactProtocolRefinementCoverageReceipt {
    pub const fn declared_owner_cases(self) -> u64 {
        self.declared_owner_cases
    }

    pub const fn ordinary_executed_cases(self) -> u64 {
        self.ordinary_executed_cases
    }

    pub const fn mapped_model_actions(self) -> u64 {
        self.mapped_model_actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_coverage_rejects_equal_counts_over_different_cases() {
        assert_eq!(
            require_exact_protocol_refinement_coverage([1, 2], [1, 3], [1, 2]),
            Err(ProtocolRefinementCoverageDenial::MissingOrdinaryExecution)
        );
    }

    #[test]
    fn exact_coverage_rejects_duplicate_mapping_rows() {
        assert_eq!(
            require_exact_protocol_refinement_coverage([1, 2], [1, 2], [1, 2, 2]),
            Err(ProtocolRefinementCoverageDenial::DuplicateModelMapping)
        );
    }
}
