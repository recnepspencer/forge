use crate::spatial_compiled_product_family::{
    admit_spatial_compiled_product_family_input, SpatialCompiledProductFamilyAdmittedInput,
    SpatialCompiledProductFamilyCatalog, SpatialCompiledProductSupportBasis,
};
use crate::workload_platform::compiled_product_admission::denial::{
    SpatialCompiledProductAdmissionError, SpatialCompiledProductAdmissionErrorKind,
};
use crate::workload_platform::compiled_product_admission::request::SpatialCompiledProductAdmissionRequest;
use crate::workload_platform::evidence_lookup_index_product::EvidenceLookupLedgerBasis;
use std::sync::OnceLock;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EvidenceLookupAdmissionMaterialization {
    evidence_ledger_basis_digest: String,
    topology_support_digest: String,
    query_support_digest: String,
}

impl EvidenceLookupAdmissionMaterialization {
    pub(crate) fn evidence_ledger_basis_digest(&self) -> &str {
        &self.evidence_ledger_basis_digest
    }

    pub(crate) fn topology_support_digest(&self) -> &str {
        &self.topology_support_digest
    }

    pub(crate) fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SpatialCompiledProductAdmittedInput {
    witness: SpatialCompiledProductAdmissionWitness,
    family_admitted_input: SpatialCompiledProductFamilyAdmittedInput,
    evidence_lookup: Option<EvidenceLookupAdmissionMaterialization>,
}

impl SpatialCompiledProductAdmittedInput {
    pub(crate) const fn witness(&self) -> &SpatialCompiledProductAdmissionWitness {
        &self.witness
    }

    pub(crate) fn family_admitted_input(&self) -> SpatialCompiledProductFamilyAdmittedInput {
        self.family_admitted_input.clone()
    }

    pub(crate) fn evidence_lookup(&self) -> Option<&EvidenceLookupAdmissionMaterialization> {
        self.evidence_lookup.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SpatialCompiledProductAdmissionWitness {
    consumer: crate::spatial_compiled_product_family::SpatialCompiledProductConsumer,
    family_identity: crate::spatial_compiled_product_family::SpatialCompiledProductFamilyIdentity,
    admission_token: String,
}

impl SpatialCompiledProductAdmissionWitness {
    pub(crate) const fn consumer(
        &self,
    ) -> crate::spatial_compiled_product_family::SpatialCompiledProductConsumer {
        self.consumer
    }

    pub(crate) const fn family_identity(
        &self,
    ) -> crate::spatial_compiled_product_family::SpatialCompiledProductFamilyIdentity {
        self.family_identity
    }

    pub(crate) fn admission_token(&self) -> &str {
        &self.admission_token
    }

    fn from_admission_request(
        request_key: &str,
        family_admitted_input: &SpatialCompiledProductFamilyAdmittedInput,
    ) -> Self {
        Self {
            consumer: family_admitted_input.consumer(),
            family_identity: family_admitted_input.family_identity(),
            admission_token: admission_token_for_key(request_key),
        }
    }
}

fn admission_request_key(request: &SpatialCompiledProductAdmissionRequest<'_>) -> String {
    let mut parts = vec![
        "worth-spatial:compiled-product-admission-request-key:v1".to_string(),
        format!("consumer:{}", request.consumer().as_str()),
    ];

    match request {
        SpatialCompiledProductAdmissionRequest::EvidenceLookupLedger {
            selected_plan,
            ledger,
            ..
        } => {
            parts.push("request-kind:evidence-lookup-ledger".to_string());
            parts.push(format!(
                "selected-plan:{}",
                selected_plan.selected_plan_digest()
            ));
            parts.push(format!(
                "stage-receipt:{}",
                selected_plan.stage_receipt_digest()
            ));
            parts.push(format!(
                "ledger-rows:{}",
                row_identity_digest(ledger.complete_ledger().rows())
            ));
        }
        SpatialCompiledProductAdmissionRequest::EvidenceLookupProduct {
            selected_plan,
            product,
            ..
        } => {
            parts.push("request-kind:evidence-lookup-product".to_string());
            parts.push(format!(
                "selected-plan:{}",
                selected_plan.selected_plan_digest()
            ));
            parts.push(format!(
                "stage-receipt:{}",
                selected_plan.stage_receipt_digest()
            ));
            parts.push(format!("index-product:{}", product.index_product_digest()));
        }
        SpatialCompiledProductAdmissionRequest::RetainedReplay {
            historical,
            retained,
            projection,
        } => {
            parts.push("request-kind:retained-replay".to_string());
            parts.push(format!("historical:{}", historical.historical_digest()));
            parts.push(format!(
                "retained-facts:{}",
                retained.retained_fact_digest()
            ));
            parts.push(format!(
                "retained-declaration:{}",
                retained.declaration_digest()
            ));
            parts.push(format!(
                "retained-route-plan:{}",
                retained.route_plan_digest()
            ));
            parts.push(format!(
                "retained-query-receipt:{}",
                retained.query_receipt_digest()
            ));
            parts.push(format!("retained-envelope:{}", retained.envelope_digest()));
            parts.push(format!(
                "projection-consumption:{}",
                projection.projection_consumption_digest()
            ));
            parts.push(format!(
                "projection-retained-facts:{}",
                projection.retained_planar_fact_digest()
            ));
        }
        SpatialCompiledProductAdmissionRequest::RetainedCancellation { receipt } => {
            parts.push("request-kind:retained-cancellation".to_string());
            parts.push(format!("chain-digest:{}", receipt.chain_digest()));
            parts.push(format!("workload:{}", receipt.workload_identity()));
            parts.push(format!(
                "retained-basis:{}",
                receipt.retained_basis_identity()
            ));
            parts.push(format!(
                "projection-consumption:{}",
                receipt.projection_consumed_identity()
            ));
        }
    }

    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

fn row_identity_digest(
    rows: &[crate::workload_platform::evidence_ledger::WorkloadEvidenceRow],
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &rows
            .iter()
            .map(|row| format!("{}:{}", row.stage().human_name(), row.evidence_identity()))
            .collect::<Vec<_>>(),
    )
}

fn admission_boundary_secret() -> &'static str {
    static SECRET: OnceLock<String> = OnceLock::new();
    SECRET
        .get_or_init(|| {
            truth_digest_parts(
                TruthDigestScope::ArtifactIdentity,
                &[
                    "worth-spatial:compiled-product-admission-boundary-secret:v1".to_string(),
                    format!("process-id:{}", std::process::id()),
                    format!("secret-cell:{:p}", &SECRET),
                ],
            )
        })
        .as_str()
}

fn admission_token_for_key(key: &str) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:compiled-product-admission-token:v2".to_string(),
            format!("boundary-secret:{}", admission_boundary_secret()),
            format!("key:{key}"),
        ],
    )
}

pub(crate) fn admit_spatial_compiled_product_input(
    catalog: &SpatialCompiledProductFamilyCatalog,
    request: SpatialCompiledProductAdmissionRequest<'_>,
) -> Result<SpatialCompiledProductAdmittedInput, SpatialCompiledProductAdmissionError> {
    use crate::workload_platform::compiled_product_admission::{
        locality_basis, prior_proof_basis, source_authority_basis, support_posture,
    };

    let consumer = request.consumer();
    let request_key = admission_request_key(&request);
    let mut evidence_lookup = None;
    let basis = match request {
        SpatialCompiledProductAdmissionRequest::EvidenceLookupLedger {
            selected_plan,
            ledger,
            ..
        } => {
            let basis = EvidenceLookupLedgerBasis::from_selected_plan(
                selected_plan,
                ledger.complete_ledger(),
            );
            let source_authority_digest =
                source_authority_basis::evidence_lookup_from_basis(selected_plan, &basis)?;
            let locality_footprint_digest =
                locality_basis::evidence_lookup_from_basis(selected_plan, &basis)?;
            let support = support_posture::evidence_lookup_from_basis(selected_plan, &basis)?;
            let prior_proof_digest = prior_proof_basis::evidence_lookup(selected_plan, &support);
            evidence_lookup = Some(EvidenceLookupAdmissionMaterialization {
                evidence_ledger_basis_digest: source_authority_digest.clone(),
                query_support_digest: support.query_support_digest().to_string(),
                topology_support_digest: support.topology_support_digest().to_string(),
            });
            SpatialCompiledProductSupportBasis::EvidenceLookupIndexProduct {
                evidence_ledger_basis_digest: source_authority_digest,
                locality_footprint_digest,
                prior_proof_digest,
                query_support_digest: support.query_support_digest().to_string(),
                stage_receipt_digest: selected_plan.stage_receipt_digest().to_string(),
                topology_support_digest: support.topology_support_digest().to_string(),
            }
        }
        SpatialCompiledProductAdmissionRequest::EvidenceLookupProduct {
            selected_plan,
            product,
            ..
        } => {
            let source_authority_digest =
                source_authority_basis::evidence_lookup_from_product(selected_plan, product)?;
            let locality_footprint_digest =
                locality_basis::evidence_lookup_from_product(selected_plan, product)?;
            let support = support_posture::evidence_lookup_from_product(selected_plan, product)?;
            let prior_proof_digest = prior_proof_basis::evidence_lookup(selected_plan, &support);
            evidence_lookup = Some(EvidenceLookupAdmissionMaterialization {
                evidence_ledger_basis_digest: source_authority_digest.clone(),
                query_support_digest: support.query_support_digest().to_string(),
                topology_support_digest: support.topology_support_digest().to_string(),
            });
            SpatialCompiledProductSupportBasis::EvidenceLookupIndexProduct {
                evidence_ledger_basis_digest: source_authority_digest,
                locality_footprint_digest,
                prior_proof_digest,
                query_support_digest: support.query_support_digest().to_string(),
                stage_receipt_digest: selected_plan.stage_receipt_digest().to_string(),
                topology_support_digest: support.topology_support_digest().to_string(),
            }
        }
        SpatialCompiledProductAdmissionRequest::RetainedReplay {
            historical,
            retained,
            projection,
        } => {
            let source_authority_digest =
                source_authority_basis::retained_replay(historical, retained, projection)?;
            let locality_footprint_digest =
                locality_basis::retained_replay(projection.projection_consumption_digest());
            let support = support_posture::retained_replay(retained, projection);
            SpatialCompiledProductSupportBasis::RetainedReplayParity {
                locality_footprint_digest,
                projection_consumption_digest: support.projection_consumption_digest().to_string(),
                retained_planar_historical_digest: source_authority_digest,
                replay_support_digest: support.replay_support_digest().to_string(),
            }
        }
        SpatialCompiledProductAdmissionRequest::RetainedCancellation { receipt } => {
            let source_authority_digest = source_authority_basis::retained_cancellation(receipt);
            let locality_footprint_digest = locality_basis::retained_cancellation(receipt);
            let prior_proof_digest = prior_proof_basis::retained_cancellation(receipt);
            let evidence_support_digest = support_posture::retained_cancellation(receipt);
            SpatialCompiledProductSupportBasis::RetainedCancellationChain {
                evidence_support_digest,
                locality_footprint_digest,
                prior_proof_digest,
                source_authority_digest,
            }
        }
    };
    let family_admitted_input =
        admit_spatial_compiled_product_family_input(catalog, consumer, basis).map_err(|error| {
            SpatialCompiledProductAdmissionError::new(
                SpatialCompiledProductAdmissionErrorKind::FamilyCatalogDenied,
                format!(
                    "spatial compiled-product family admission failed: {:?}",
                    error.kind()
                ),
            )
        })?;

    let witness = SpatialCompiledProductAdmissionWitness::from_admission_request(
        &request_key,
        &family_admitted_input,
    );

    Ok(SpatialCompiledProductAdmittedInput {
        witness,
        family_admitted_input,
        evidence_lookup,
    })
}
