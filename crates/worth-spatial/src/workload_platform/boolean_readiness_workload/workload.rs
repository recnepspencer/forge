use forge_query::facade::ForgeQueryDomainOperatingContext;

use crate::bindings::query_native_planar_contract_bundle::{
    PlanarContractBundleValidationContracts, PlanarContractBundleValidationQueryDomain,
};

use super::counters::PlanarBooleanReadinessWorkloadCounters;
use super::denial::PlanarBooleanReadinessWorkloadDenial;
use super::evidence_basis::PlanarBooleanReadinessEvidenceBasis;
use super::receipt::PlanarBooleanReadinessWorkloadReceipt;
use super::required_stage::PlanarBooleanReadinessRequiredStage;
use super::validation::{
    m7_denial_to_workload_denial, query_boundary_denial, readiness_workload_digest,
    validate_readiness_receipt, validate_readiness_workload_basis,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanReadinessWorkload {
    evidence_basis: PlanarBooleanReadinessEvidenceBasis,
    declaration: String,
}

impl PlanarBooleanReadinessWorkload {
    pub fn from_real_workload_evidence(
        evidence_basis: PlanarBooleanReadinessEvidenceBasis,
    ) -> Self {
        Self {
            evidence_basis,
            declaration: String::new(),
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn certify_pre_boolean_readiness<WC>(
        self,
        contracts: &PlanarContractBundleValidationContracts<WC>,
    ) -> Result<PlanarBooleanReadinessWorkloadReceipt, PlanarBooleanReadinessWorkloadDenial>
    where
        WC: ForgeQueryDomainOperatingContext<PlanarContractBundleValidationQueryDomain>,
    {
        validate_readiness_workload_basis(&self.evidence_basis, &self.declaration)?;
        let (evidence_ledger, readiness_bundle, parity_receipt) =
            self.evidence_basis.into_certification_parts();
        let readiness_plan = readiness_bundle
            .compile(contracts)
            .map_err(|denial| m7_denial_to_workload_denial(denial.kind(), denial.reason()))?;
        let closeout_rows = readiness_plan.inspected_closeout_rows();
        let readiness_receipt = readiness_plan
            .certify()
            .map_err(|denial| m7_denial_to_workload_denial(denial.kind(), denial.reason()))?;
        validate_readiness_receipt(&readiness_receipt, &parity_receipt)?;
        let workload_digest = readiness_workload_digest(
            &evidence_ledger,
            &parity_receipt,
            &readiness_receipt,
            &self.declaration,
        );
        if workload_digest.trim().is_empty() {
            return Err(query_boundary_denial(
                "Boolean-readiness workload must produce a non-empty digest.",
            ));
        }
        Ok(PlanarBooleanReadinessWorkloadReceipt::new(
            readiness_receipt,
            workload_digest,
            self.declaration,
            PlanarBooleanReadinessWorkloadCounters::certified(
                PlanarBooleanReadinessRequiredStage::ALL.len(),
                evidence_ledger.rows().len(),
                parity_receipt.counters().lanes_compared(),
                closeout_rows,
                1,
            ),
        ))
    }
}
