use crate::runtime::WorthQueryReadResult;
use crate::{basis_lifecycle::BasisFamily, ordinary::read::WorthQueryReadContextKind};

use super::super::projection::WorthQueryReadProjectionBinding;
use super::super::{WorthQueryProjectionDeclaration, WorthQueryProjectionOutcome};
use super::super::{WorthQueryReadContextReceipt, WorthQueryReadJourneyCounters};

#[derive(Debug)]
pub struct WorthQueryReadCompletion {
    result: WorthQueryReadResult,
    context_receipt: WorthQueryReadContextReceipt,
    journey_counters: WorthQueryReadJourneyCounters,
    projection_binding: WorthQueryReadProjectionBinding,
    runtime_authority: crate::runtime::WorthQueryRuntimeAuthorityIdentity,
}

impl WorthQueryReadCompletion {
    pub fn result(&self) -> &WorthQueryReadResult {
        &self.result
    }

    pub fn context_receipt(&self) -> &WorthQueryReadContextReceipt {
        &self.context_receipt
    }

    pub fn journey_counters(&self) -> &WorthQueryReadJourneyCounters {
        &self.journey_counters
    }

    pub(crate) fn runtime_authority(&self) -> crate::runtime::WorthQueryRuntimeAuthorityIdentity {
        self.runtime_authority
    }

    /// Extract typed projection facts through the authority sealed into this
    /// completed read. The operational receipt alone cannot invoke this lane.
    pub fn consume_projection(
        &self,
        declaration: WorthQueryProjectionDeclaration,
    ) -> WorthQueryProjectionOutcome {
        self.projection_binding.consume(&self.result, declaration)
    }

    pub(crate) fn consume_projection_contract_for_certification(
        &self,
        contract: crate::projection_consumption::ProjectionAuthorityContract,
    ) -> WorthQueryProjectionOutcome {
        self.projection_binding
            .consume_contract(&self.result, contract)
    }

    pub(crate) fn validates_installed_publication(
        &self,
        canonical: &crate::canonicalization::CanonicalQueryBundle,
        expected_basis: BasisFamily,
        expected_snapshot: &crate::memory_workspace::WorthQuerySnapshotIdentity,
        expected_runtime: crate::runtime::WorthQueryRuntimeAuthorityIdentity,
    ) -> bool {
        self.runtime_authority == expected_runtime
            && expected_basis == BasisFamily::CurrentHead
            && self.context_receipt.context_kind() == WorthQueryReadContextKind::Current
            && self.context_receipt.canonical_query_digest() == canonical.query().digest().as_str()
            && self.result.receipt().canonical_query_digest() == canonical.query().digest().as_str()
            && self.result.receipt().snapshot_identity() == expected_snapshot
            && self
                .projection_binding
                .validates_installed_publication(canonical)
    }

    pub fn into_result(self) -> WorthQueryReadResult {
        self.result
    }

    pub(crate) fn new(
        result: WorthQueryReadResult,
        context_receipt: WorthQueryReadContextReceipt,
        journey_counters: WorthQueryReadJourneyCounters,
        projection_binding: WorthQueryReadProjectionBinding,
        runtime_authority: crate::runtime::WorthQueryRuntimeAuthorityIdentity,
    ) -> Self {
        Self {
            result,
            context_receipt,
            journey_counters,
            projection_binding,
            runtime_authority,
        }
    }
}
