use crate::{
    WorthServerCompletedProductOperation, WorthServerExecutedProductReadBatch,
    WorthServerProductOperationSurfaceDenial, WorthServerProductOperationSurfaceDenialCode,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductSharedReadCertificationProof {
    canonical_digest: String,
}

impl WorthServerProductSharedReadCertificationProof {
    pub fn from_batch(batch: &WorthServerExecutedProductReadBatch) -> Result<Self, &'static str> {
        let operation_count = batch.operations().len();
        if operation_count == 0 {
            return Err("shared-read certification requires at least one completed operation");
        }
        if batch.counters().planned_batch_width() != operation_count
            || batch.counters().admitted_read_slot_count() != operation_count
            || batch.counters().queued_read_slot_count() != operation_count
            || batch.counters().completed_read_slot_count() != operation_count
        {
            return Err("shared-read certification requires exact scheduler counters");
        }
        if batch.operations().iter().any(|operation| {
            operation
                .scheduler_admission()
                .map(|admission| admission.scheduler_lane() != "shared-read")
                .unwrap_or(true)
        }) {
            return Err("shared-read certification requires shared-read scheduler lanes");
        }
        Ok(Self {
            canonical_digest: format!(
                "product-shared-read-certification-v1|batch={}",
                batch.canonical_digest()
            ),
        })
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductMutationCertificationProof {
    canonical_digest: String,
}

impl WorthServerProductMutationCertificationProof {
    pub fn new(
        apply: &WorthServerCompletedProductOperation,
        finalize: &WorthServerCompletedProductOperation,
    ) -> Result<Self, &'static str> {
        if !apply.adapter_execution_attempted() || !finalize.adapter_execution_attempted() {
            return Err("mutation certification requires adapter-executed apply/finalize proofs");
        }
        Ok(Self {
            canonical_digest: format!(
                "product-mutation-certification-v1|apply={}|finalize={}",
                apply.envelope().canonical_digest(),
                finalize.envelope().canonical_digest()
            ),
        })
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductRouteParityCertificationProof {
    canonical_digest: String,
}

impl WorthServerProductRouteParityCertificationProof {
    pub fn new(entries: &[WorthServerProductRouteParityEntry]) -> Result<Self, &'static str> {
        if entries.len() != 4 {
            return Err("route parity certification requires render/select/apply/finalize entries");
        }
        if entries.iter().any(|entry| !entry.is_parity_exact()) {
            return Err("route parity certification requires exact direct/compat/route parity");
        }
        Ok(Self {
            canonical_digest: format!(
                "product-route-parity-certification-v1|{}",
                entries
                    .iter()
                    .map(WorthServerProductRouteParityEntry::canonical_digest)
                    .collect::<Vec<_>>()
                    .join("|")
            ),
        })
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductRouteParityEntry {
    canonical_digest: String,
    parity_exact: bool,
}

impl WorthServerProductRouteParityEntry {
    pub fn new(
        operation_name: &str,
        direct: &WorthServerCompletedProductOperation,
        compat: &WorthServerCompletedProductOperation,
        route_plan_digest: &str,
        route_envelope_digest: &str,
        require_envelope_parity: bool,
    ) -> Self {
        let direct_plan_digest = direct
            .plan()
            .map(|plan| plan.canonical_digest())
            .unwrap_or("missing");
        let compat_plan_digest = compat
            .plan()
            .map(|plan| plan.canonical_digest())
            .unwrap_or("missing");
        let parity_exact = direct_plan_digest == compat_plan_digest
            && direct_plan_digest == route_plan_digest
            && (!require_envelope_parity
                || (direct.envelope().canonical_digest() == compat.envelope().canonical_digest()
                    && direct.envelope().canonical_digest() == route_envelope_digest));
        Self {
            canonical_digest: format!(
                "operation={operation_name}|direct_plan={direct_plan_digest}|compat_plan={compat_plan_digest}|route_plan={route_plan_digest}|direct_envelope={}|compat_envelope={}|route_envelope={route_envelope_digest}|require_envelope_parity={require_envelope_parity}|parity={parity_exact}",
                direct.envelope().canonical_digest(),
                compat.envelope().canonical_digest(),
            ),
            parity_exact,
        }
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub fn is_parity_exact(&self) -> bool {
        self.parity_exact
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductPressureShapeCertificationProof {
    canonical_digest: String,
}

impl WorthServerProductPressureShapeCertificationProof {
    pub fn new(
        render: &WorthServerCompletedProductOperation,
        select: &WorthServerCompletedProductOperation,
        actions_before: &WorthServerCompletedProductOperation,
        actions_after: &WorthServerCompletedProductOperation,
        applied: &WorthServerCompletedProductOperation,
        finalize_denial_reason_key: &str,
    ) -> Self {
        Self {
            canonical_digest: format!(
                "product-pressure-shape-certification-v1|render={}|select={}|actions_before={}|actions_after={}|apply={}|finalize_denial={finalize_denial_reason_key}",
                render.envelope().canonical_digest(),
                select.envelope().canonical_digest(),
                actions_before.envelope().canonical_digest(),
                actions_after.envelope().canonical_digest(),
                applied.envelope().canonical_digest(),
            ),
        }
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductStaleApplyDenialCertificationProof {
    canonical_digest: String,
}

impl WorthServerProductStaleApplyDenialCertificationProof {
    pub fn from_denial(
        denial: &WorthServerProductOperationSurfaceDenial,
    ) -> Result<Self, &'static str> {
        if denial.code() != WorthServerProductOperationSurfaceDenialCode::PreconditionDenied {
            return Err("stale apply certification requires precondition denial");
        }
        let facts = denial
            .facts()
            .ok_or("stale apply certification requires denial facts")?;
        Ok(Self {
            canonical_digest: format!(
                "product-stale-apply-certification-v1|expected={}|observed={}|boundary={:?}",
                facts.expected_basis_digest().unwrap_or("missing"),
                facts.observed_basis_digest().unwrap_or("missing"),
                facts.execution_boundary()
            ),
        })
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductIdempotentRetryCertificationProof {
    canonical_digest: String,
}

impl WorthServerProductIdempotentRetryCertificationProof {
    pub fn new(
        executed: &WorthServerCompletedProductOperation,
        previously_committed: &WorthServerCompletedProductOperation,
    ) -> Result<Self, &'static str> {
        if !executed.retry_diagnostics().is_executed()
            || !previously_committed
                .retry_diagnostics()
                .is_previously_committed()
        {
            return Err(
                "idempotent retry certification requires executed and previously committed proofs",
            );
        }
        Ok(Self {
            canonical_digest: format!(
                "product-idempotent-retry-certification-v1|executed={}|previously_committed={}|linked={}",
                executed.envelope().canonical_digest(),
                previously_committed.envelope().canonical_digest(),
                previously_committed
                    .retry_receipt()
                    .and_then(|receipt| receipt.original_operation_digest())
                    .unwrap_or("missing"),
            ),
        })
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
