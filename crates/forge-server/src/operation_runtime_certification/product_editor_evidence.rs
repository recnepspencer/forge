use crate::{
    ForgeServerCompletedProductOperation, ForgeServerExecutedProductReadBatch,
    ForgeServerProductOperationSurfaceDenial, ForgeServerProductOperationSurfaceDenialCode,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerProductSharedReadCertificationProof {
    canonical_digest: String,
}

impl ForgeServerProductSharedReadCertificationProof {
    pub fn from_batch(batch: &ForgeServerExecutedProductReadBatch) -> Result<Self, &'static str> {
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
pub struct ForgeServerProductMutationCertificationProof {
    canonical_digest: String,
}

impl ForgeServerProductMutationCertificationProof {
    pub fn new(
        apply: &ForgeServerCompletedProductOperation,
        finalize: &ForgeServerCompletedProductOperation,
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
pub struct ForgeServerProductRouteParityCertificationProof {
    canonical_digest: String,
}

impl ForgeServerProductRouteParityCertificationProof {
    pub fn new(entries: &[ForgeServerProductRouteParityEntry]) -> Result<Self, &'static str> {
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
                    .map(ForgeServerProductRouteParityEntry::canonical_digest)
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
pub struct ForgeServerProductRouteParityEntry {
    canonical_digest: String,
    parity_exact: bool,
}

impl ForgeServerProductRouteParityEntry {
    pub fn new(
        operation_name: &str,
        direct: &ForgeServerCompletedProductOperation,
        compat: &ForgeServerCompletedProductOperation,
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
pub struct ForgeServerProductPressureShapeCertificationProof {
    canonical_digest: String,
}

impl ForgeServerProductPressureShapeCertificationProof {
    pub fn new(
        render: &ForgeServerCompletedProductOperation,
        select: &ForgeServerCompletedProductOperation,
        actions_before: &ForgeServerCompletedProductOperation,
        actions_after: &ForgeServerCompletedProductOperation,
        applied: &ForgeServerCompletedProductOperation,
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
pub struct ForgeServerProductStaleApplyDenialCertificationProof {
    canonical_digest: String,
}

impl ForgeServerProductStaleApplyDenialCertificationProof {
    pub fn from_denial(
        denial: &ForgeServerProductOperationSurfaceDenial,
    ) -> Result<Self, &'static str> {
        if denial.code() != ForgeServerProductOperationSurfaceDenialCode::PreconditionDenied {
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
pub struct ForgeServerProductIdempotentReplayCertificationProof {
    canonical_digest: String,
}

impl ForgeServerProductIdempotentReplayCertificationProof {
    pub fn new(
        authoritative: &ForgeServerCompletedProductOperation,
        replayed: &ForgeServerCompletedProductOperation,
    ) -> Result<Self, &'static str> {
        if !authoritative.replay_diagnostics().is_authoritative()
            || !replayed.replay_diagnostics().is_replayed()
        {
            return Err(
                "idempotent replay certification requires authoritative and replayed proofs",
            );
        }
        Ok(Self {
            canonical_digest: format!(
                "product-idempotent-replay-certification-v1|authoritative={}|replayed={}|linked={}",
                authoritative.envelope().canonical_digest(),
                replayed.envelope().canonical_digest(),
                replayed
                    .replay_receipt()
                    .and_then(|receipt| receipt.authoritative_operation_digest())
                    .unwrap_or("missing"),
            ),
        })
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
