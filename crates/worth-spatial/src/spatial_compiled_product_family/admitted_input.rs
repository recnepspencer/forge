use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[cfg(test)]
use crate::facade::planar_projection_consumption::ProjectionConsumedPlanarFactsReceipt;
#[cfg(test)]
use crate::facade::planar_retained_facts::RetainedPlanarFactsReceipt;
#[cfg(test)]
use crate::planar_contracts::retained_planar_facts::RetainedPlanarHistoricalInspection;
#[cfg(test)]
use crate::workload_platform::evidence_lookup_index_product::EvidenceLookupIndexProduct;
#[cfg(test)]
use crate::workload_platform::evidence_lookup_index_product::EvidenceLookupLedgerBasis;
#[cfg(test)]
use crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;
#[cfg(test)]
use crate::workload_platform::retained_cancellation_chain::RetainedCancellationChainReceipt;

use super::catalog::SpatialCompiledProductFamilyCatalog;
use super::consumer::SpatialCompiledProductConsumer;
use super::error::{SpatialCompiledProductFamilyError, SpatialCompiledProductFamilyErrorKind};
use super::family_identity::SpatialCompiledProductFamilyIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SpatialCompiledProductSupportBasis {
    EvidenceLookupIndexProduct {
        evidence_ledger_basis_digest: String,
        locality_footprint_digest: String,
        prior_proof_digest: String,
        query_support_digest: String,
        stage_receipt_digest: String,
        topology_support_digest: String,
    },
    RetainedCancellationChain {
        evidence_support_digest: String,
        locality_footprint_digest: String,
        prior_proof_digest: String,
        source_authority_digest: String,
    },
    RetainedReplayParity {
        locality_footprint_digest: String,
        projection_consumption_digest: String,
        retained_planar_historical_digest: String,
        replay_support_digest: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialCompiledProductFamilyAdmittedInput {
    consumer: SpatialCompiledProductConsumer,
    family_identity: SpatialCompiledProductFamilyIdentity,
    source_authority_digest: String,
    locality_footprint_digest: String,
    prior_proof_digest: Option<String>,
    evidence_support_digest: String,
    stage_receipt_digest: Option<String>,
    grouped_support_digest: Option<String>,
}

impl SpatialCompiledProductFamilyAdmittedInput {
    pub const fn consumer(&self) -> SpatialCompiledProductConsumer {
        self.consumer
    }

    pub const fn family_identity(&self) -> SpatialCompiledProductFamilyIdentity {
        self.family_identity
    }

    pub fn source_authority_digest(&self) -> &str {
        &self.source_authority_digest
    }

    pub fn locality_footprint_digest(&self) -> &str {
        &self.locality_footprint_digest
    }

    pub fn prior_proof_digest(&self) -> Option<&str> {
        self.prior_proof_digest.as_deref()
    }

    pub fn evidence_support_digest(&self) -> &str {
        &self.evidence_support_digest
    }

    pub fn stage_receipt_digest(&self) -> Option<&str> {
        self.stage_receipt_digest.as_deref()
    }

    pub fn grouped_support_digest(&self) -> Option<&str> {
        self.grouped_support_digest.as_deref()
    }
}

#[cfg(test)]
pub(crate) fn admit_evidence_lookup_spatial_compiled_product_family_input(
    catalog: &SpatialCompiledProductFamilyCatalog,
    consumer: SpatialCompiledProductConsumer,
    selected_plan: &EvidenceLookupSelectedPlan,
    product: &EvidenceLookupIndexProduct,
) -> Result<SpatialCompiledProductFamilyAdmittedInput, SpatialCompiledProductFamilyError> {
    admit_spatial_compiled_product_family_input(
        catalog,
        consumer,
        SpatialCompiledProductSupportBasis::EvidenceLookupIndexProduct {
            evidence_ledger_basis_digest: product.evidence_ledger_basis_digest().to_string(),
            locality_footprint_digest: selected_plan.spatial_touch_digest().to_string(),
            prior_proof_digest: evidence_lookup_index_prior_proof_digest(
                selected_plan.selected_plan_digest(),
                product.topology_support_digest(),
                product.query_support_digest(),
            ),
            query_support_digest: product.query_support_digest().to_string(),
            stage_receipt_digest: selected_plan.stage_receipt_digest().to_string(),
            topology_support_digest: product.topology_support_digest().to_string(),
        },
    )
}

#[cfg(test)]
pub(crate) fn admit_evidence_lookup_spatial_compiled_product_family_input_from_basis(
    catalog: &SpatialCompiledProductFamilyCatalog,
    selected_plan: &EvidenceLookupSelectedPlan,
    basis: &EvidenceLookupLedgerBasis,
) -> Result<SpatialCompiledProductFamilyAdmittedInput, SpatialCompiledProductFamilyError> {
    admit_spatial_compiled_product_family_input(
        catalog,
        SpatialCompiledProductConsumer::EvidenceLookupIndexProduct,
        SpatialCompiledProductSupportBasis::EvidenceLookupIndexProduct {
            evidence_ledger_basis_digest: basis.basis_digest().to_string(),
            locality_footprint_digest: selected_plan.spatial_touch_digest().to_string(),
            prior_proof_digest: evidence_lookup_index_prior_proof_digest(
                selected_plan.selected_plan_digest(),
                basis.topology_support_digest(),
                basis.query_support_digest(),
            ),
            query_support_digest: basis.query_support_digest().to_string(),
            stage_receipt_digest: selected_plan.stage_receipt_digest().to_string(),
            topology_support_digest: basis.topology_support_digest().to_string(),
        },
    )
}

#[cfg(test)]
pub(crate) fn admit_retained_replay_spatial_compiled_product_family_input(
    catalog: &SpatialCompiledProductFamilyCatalog,
    historical: &RetainedPlanarHistoricalInspection,
    retained: &RetainedPlanarFactsReceipt,
    projection: &ProjectionConsumedPlanarFactsReceipt,
) -> Result<SpatialCompiledProductFamilyAdmittedInput, SpatialCompiledProductFamilyError> {
    admit_spatial_compiled_product_family_input(
        catalog,
        SpatialCompiledProductConsumer::RetainedReplayParity,
        SpatialCompiledProductSupportBasis::RetainedReplayParity {
            locality_footprint_digest: projection.projection_consumption_digest().to_string(),
            projection_consumption_digest: projection.projection_consumption_digest().to_string(),
            retained_planar_historical_digest: historical.historical_digest().to_string(),
            replay_support_digest: retained.retained_fact_digest().to_string(),
        },
    )
}

#[cfg(test)]
pub(crate) fn admit_retained_cancellation_spatial_compiled_product_family_input(
    catalog: &SpatialCompiledProductFamilyCatalog,
    receipt: &RetainedCancellationChainReceipt,
) -> Result<SpatialCompiledProductFamilyAdmittedInput, SpatialCompiledProductFamilyError> {
    admit_retained_cancellation_spatial_compiled_product_family_input_from_parts(
        catalog,
        receipt.workload_identity(),
        receipt.retained_basis_identity(),
        receipt.projection_consumed_identity(),
        receipt
            .checkpoints()
            .iter()
            .map(|checkpoint| checkpoint.checkpoint_identity()),
    )
}

#[cfg(test)]
pub(crate) fn admit_retained_cancellation_spatial_compiled_product_family_input_from_parts<'a>(
    catalog: &SpatialCompiledProductFamilyCatalog,
    workload_identity: &str,
    retained_basis_identity: &str,
    projection_consumed_identity: &str,
    checkpoint_identities: impl IntoIterator<Item = &'a str>,
) -> Result<SpatialCompiledProductFamilyAdmittedInput, SpatialCompiledProductFamilyError> {
    admit_spatial_compiled_product_family_input(
        catalog,
        SpatialCompiledProductConsumer::RetainedCancellationChain,
        SpatialCompiledProductSupportBasis::RetainedCancellationChain {
            evidence_support_digest: retained_cancellation_support_digest(
                retained_basis_identity,
                projection_consumed_identity,
            ),
            locality_footprint_digest: projection_consumed_identity.to_string(),
            prior_proof_digest: retained_cancellation_checkpoint_history_digest(
                checkpoint_identities,
            ),
            source_authority_digest: retained_cancellation_source_authority_digest(
                workload_identity,
                retained_basis_identity,
            ),
        },
    )
}

pub(crate) fn admit_spatial_compiled_product_family_input(
    catalog: &SpatialCompiledProductFamilyCatalog,
    consumer: SpatialCompiledProductConsumer,
    basis: SpatialCompiledProductSupportBasis,
) -> Result<SpatialCompiledProductFamilyAdmittedInput, SpatialCompiledProductFamilyError> {
    let family_identity = catalog
        .family_for_consumer(consumer)
        .map(|family| family.identity())
        .ok_or_else(|| {
            SpatialCompiledProductFamilyError::new(
                SpatialCompiledProductFamilyErrorKind::NoDeclaredFamilyForConsumer,
                "spatial compiled-product family catalog has no declaration for the requested consumer",
            )
        })?;

    match (consumer, basis) {
        (
            SpatialCompiledProductConsumer::EvidenceLookupIndexProduct
            | SpatialCompiledProductConsumer::EvidenceLookupPublicCloseout,
            SpatialCompiledProductSupportBasis::EvidenceLookupIndexProduct {
                evidence_ledger_basis_digest,
                locality_footprint_digest,
                prior_proof_digest,
                query_support_digest,
                stage_receipt_digest,
                topology_support_digest,
            },
        ) => Ok(SpatialCompiledProductFamilyAdmittedInput {
            consumer,
            family_identity,
            source_authority_digest: evidence_ledger_basis_digest,
            locality_footprint_digest,
            prior_proof_digest: Some(prior_proof_digest),
            evidence_support_digest: evidence_lookup_support_digest(
                &topology_support_digest,
                &query_support_digest,
            ),
            stage_receipt_digest: Some(stage_receipt_digest),
            grouped_support_digest: None,
        }),
        (
            SpatialCompiledProductConsumer::RetainedReplayParity,
            SpatialCompiledProductSupportBasis::RetainedReplayParity {
                locality_footprint_digest,
                projection_consumption_digest,
                retained_planar_historical_digest,
                replay_support_digest,
            },
        ) => Ok(SpatialCompiledProductFamilyAdmittedInput {
            consumer,
            family_identity,
            source_authority_digest: retained_planar_historical_digest,
            locality_footprint_digest,
            prior_proof_digest: None,
            evidence_support_digest: retained_replay_support_digest(
                &projection_consumption_digest,
                &replay_support_digest,
            ),
            stage_receipt_digest: None,
            grouped_support_digest: None,
        }),
        (
            SpatialCompiledProductConsumer::RetainedCancellationChain,
            SpatialCompiledProductSupportBasis::RetainedCancellationChain {
                evidence_support_digest,
                locality_footprint_digest,
                prior_proof_digest,
                source_authority_digest,
            },
        ) => Ok(SpatialCompiledProductFamilyAdmittedInput {
            consumer,
            family_identity,
            source_authority_digest,
            locality_footprint_digest,
            prior_proof_digest: Some(prior_proof_digest),
            evidence_support_digest,
            stage_receipt_digest: None,
            grouped_support_digest: None,
        }),
        _ => Err(SpatialCompiledProductFamilyError::new(
            SpatialCompiledProductFamilyErrorKind::UnsupportedConsumerBasis,
            format!("spatial compiled-product consumer `{}` received an unsupported admitted-input basis", consumer.as_str()),
        )),
    }
}

#[cfg(test)]
fn evidence_lookup_index_prior_proof_digest(
    selected_plan_digest: &str,
    topology_support_digest: &str,
    query_support_digest: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:evidence-lookup-index-prior-proof:v2".to_string(),
            format!("selected-plan:{selected_plan_digest}"),
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

#[cfg(test)]
fn retained_cancellation_source_authority_digest(
    workload_identity: &str,
    retained_basis_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:retained-cancellation-source-authority:v1".to_string(),
            format!("workload:{workload_identity}"),
            format!("retained-basis:{retained_basis_identity}"),
        ],
    )
}

#[cfg(test)]
fn retained_cancellation_checkpoint_history_digest<'a>(
    checkpoint_identities: impl IntoIterator<Item = &'a str>,
) -> String {
    let checkpoint_list = checkpoint_identities
        .into_iter()
        .collect::<Vec<_>>()
        .join("|");
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:retained-cancellation-prior-proof:v1".to_string(),
            format!("checkpoint-history:{checkpoint_list}"),
        ],
    )
}

#[cfg(test)]
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
