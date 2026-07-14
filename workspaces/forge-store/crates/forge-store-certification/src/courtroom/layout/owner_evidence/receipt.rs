use crate::courtroom::layout::owner_coverage::{
    certify_exact_owner_case_coverage, LayoutOwnerCaseDeclarations, LayoutOwnerCoverageReceipt,
};
use crate::courtroom::layout::owner_scenarios::{
    durable_observation::LayoutDurableObservationSource, LayoutOwnerScenarioTranscript,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutOwnerExecutionEvidenceDenial {
    OwnerCoverageIncomplete,
}

/// Courtroom evidence that every declared owner case was reached through its
/// ordinary production operation.
///
/// This receipt carries observations forward; it is not production authority
/// and cannot issue or reconstruct an owner outcome.
#[derive(Debug, PartialEq, Eq)]
pub struct LayoutOwnerExecutionEvidence {
    coverage: LayoutOwnerCoverageReceipt,
    performance: forge_store_layout_indexes::LayoutAccessPerformanceReceipt,
    durable: LayoutDurableObservationSource,
}

pub fn certify_layout_owner_execution_evidence(
    execution: LayoutOwnerScenarioTranscript,
) -> Result<LayoutOwnerExecutionEvidence, LayoutOwnerExecutionEvidenceDenial> {
    let (observations, performance, durable) = execution.into_evidence_parts();
    let coverage = certify_exact_owner_case_coverage(
        &LayoutOwnerCaseDeclarations::from_owner_inventories(),
        &observations,
    )
    .map_err(|_| LayoutOwnerExecutionEvidenceDenial::OwnerCoverageIncomplete)?;
    Ok(LayoutOwnerExecutionEvidence {
        coverage,
        performance,
        durable,
    })
}

impl LayoutOwnerExecutionEvidence {
    pub const fn coverage(&self) -> &LayoutOwnerCoverageReceipt {
        &self.coverage
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        LayoutOwnerCoverageReceipt,
        forge_store_layout_indexes::LayoutAccessPerformanceReceipt,
        LayoutDurableObservationSource,
    ) {
        (self.coverage, self.performance, self.durable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::courtroom::layout::owner_coverage::{
        require_exact_owner_family_coverage, LayoutOwnerFamily,
    };
    use crate::courtroom::layout::owner_scenarios::execute_declaration_owner_scenarios;

    #[test]
    fn independent_executions_converge_on_exact_owner_and_performance_evidence() {
        let first =
            certify_layout_owner_execution_evidence(execute_declaration_owner_scenarios().unwrap())
                .unwrap();
        let second =
            certify_layout_owner_execution_evidence(execute_declaration_owner_scenarios().unwrap())
                .unwrap();

        assert_eq!(first.coverage, second.coverage);
        assert_ne!(
            first.performance.plan_binding(),
            second.performance.plan_binding()
        );
        assert_eq!(
            first.performance.counter_backed(),
            second.performance.counter_backed()
        );
    }

    #[test]
    fn exact_coverage_is_independent_of_registry_iteration_order() {
        let execution = execute_declaration_owner_scenarios().unwrap();
        let declarations = LayoutOwnerCaseDeclarations::from_owner_inventories();
        let mut reversed = LayoutOwnerFamily::all().to_vec();
        reversed.reverse();

        require_exact_owner_family_coverage(&declarations, execution.observations(), reversed)
            .unwrap();
    }
}
