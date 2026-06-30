use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::facade::planar_projection_consumption::ProjectionConsumedPlanarFactsReceipt;
use crate::facade::planar_retained_facts::RetainedPlanarFactsReceipt;
use crate::workload_platform::compiled_product_admission::denial::{
    SpatialCompiledProductAdmissionError, SpatialCompiledProductAdmissionErrorKind,
};
use crate::workload_platform::evidence_lookup_index_product::{
    selected_query_support_digest, selected_topology_support_digest, EvidenceLookupIndexProduct,
    EvidenceLookupLedgerBasis,
};
use crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;
use crate::workload_platform::retained_cancellation_chain::RetainedCancellationChainReceipt;

pub(crate) struct EvidenceLookupSupportPosture {
    topology_support_digest: String,
    query_support_digest: String,
    evidence_support_digest: String,
}

impl EvidenceLookupSupportPosture {
    pub(crate) fn topology_support_digest(&self) -> &str {
        &self.topology_support_digest
    }

    pub(crate) fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }

    pub(crate) fn evidence_support_digest(&self) -> &str {
        &self.evidence_support_digest
    }
}

pub(crate) struct RetainedReplaySupportPosture {
    projection_consumption_digest: String,
    replay_support_digest: String,
    evidence_support_digest: String,
}

impl RetainedReplaySupportPosture {
    pub(crate) fn projection_consumption_digest(&self) -> &str {
        &self.projection_consumption_digest
    }

    pub(crate) fn replay_support_digest(&self) -> &str {
        &self.replay_support_digest
    }

    pub(crate) fn evidence_support_digest(&self) -> &str {
        &self.evidence_support_digest
    }
}

pub(crate) fn evidence_lookup_from_basis(
    selected_plan: &EvidenceLookupSelectedPlan,
    basis: &EvidenceLookupLedgerBasis,
) -> Result<EvidenceLookupSupportPosture, SpatialCompiledProductAdmissionError> {
    if basis.exceeds_selected_scope() {
        return Err(SpatialCompiledProductAdmissionError::new(
            SpatialCompiledProductAdmissionErrorKind::BroadEvidenceScanDenied,
            "evidence lookup compiled-product admission denies complete-ledger scans as authority basis",
        ));
    }
    let topology_support_digest = selected_topology_support_digest(selected_plan.rows());
    let query_support_digest = selected_query_support_digest(selected_plan.rows());
    if basis.topology_support_digest() != topology_support_digest {
        return Err(SpatialCompiledProductAdmissionError::new(
            SpatialCompiledProductAdmissionErrorKind::WrongSupportPosture,
            "evidence lookup basis topology support digest does not match the selected plan",
        ));
    }
    if basis.query_support_digest() != query_support_digest {
        return Err(SpatialCompiledProductAdmissionError::new(
            SpatialCompiledProductAdmissionErrorKind::WrongSupportPosture,
            "evidence lookup basis query support digest does not match the selected plan",
        ));
    }
    Ok(EvidenceLookupSupportPosture {
        evidence_support_digest: evidence_lookup_support_digest(
            &topology_support_digest,
            &query_support_digest,
        ),
        query_support_digest,
        topology_support_digest,
    })
}

pub(crate) fn evidence_lookup_from_product(
    selected_plan: &EvidenceLookupSelectedPlan,
    product: &EvidenceLookupIndexProduct,
) -> Result<EvidenceLookupSupportPosture, SpatialCompiledProductAdmissionError> {
    let topology_support_digest = selected_topology_support_digest(selected_plan.rows());
    let query_support_digest = selected_query_support_digest(selected_plan.rows());
    if product.topology_support_digest() != topology_support_digest {
        return Err(SpatialCompiledProductAdmissionError::new(
            SpatialCompiledProductAdmissionErrorKind::WrongSupportPosture,
            "evidence lookup product topology support digest does not match the selected plan",
        ));
    }
    if product.query_support_digest() != query_support_digest {
        return Err(SpatialCompiledProductAdmissionError::new(
            SpatialCompiledProductAdmissionErrorKind::WrongSupportPosture,
            "evidence lookup product query support digest does not match the selected plan",
        ));
    }
    Ok(EvidenceLookupSupportPosture {
        evidence_support_digest: evidence_lookup_support_digest(
            &topology_support_digest,
            &query_support_digest,
        ),
        query_support_digest,
        topology_support_digest,
    })
}

pub(crate) fn retained_replay(
    retained: &RetainedPlanarFactsReceipt,
    projection: &ProjectionConsumedPlanarFactsReceipt,
) -> RetainedReplaySupportPosture {
    let projection_consumption_digest = projection.projection_consumption_digest().to_string();
    let replay_support_digest = retained.retained_fact_digest().to_string();
    RetainedReplaySupportPosture {
        evidence_support_digest: retained_replay_support_digest(
            &projection_consumption_digest,
            &replay_support_digest,
        ),
        projection_consumption_digest,
        replay_support_digest,
    }
}

pub(crate) fn retained_cancellation(receipt: &RetainedCancellationChainReceipt) -> String {
    retained_cancellation_support_digest(
        receipt.retained_basis_identity(),
        receipt.projection_consumed_identity(),
    )
}

fn evidence_lookup_support_digest(
    topology_support_digest: &str,
    query_support_digest: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:evidence-lookup-support:v1".to_string(),
            format!("topology-support:{topology_support_digest}"),
            format!("query-support:{query_support_digest}"),
        ],
    )
}

fn retained_replay_support_digest(
    projection_consumption_digest: &str,
    replay_support_digest: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:retained-replay-support:v1".to_string(),
            format!("projection-consumption:{projection_consumption_digest}"),
            format!("replay-support:{replay_support_digest}"),
        ],
    )
}

fn retained_cancellation_support_digest(
    retained_basis_identity: &str,
    projection_consumed_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:retained-cancellation-support:v1".to_string(),
            format!("retained-basis:{retained_basis_identity}"),
            format!("projection-consumed:{projection_consumed_identity}"),
        ],
    )
}
