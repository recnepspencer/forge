#[cfg(test)]
use crate::application::ForgeQueryContinuationExecutionReadmissionObservation;
use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationBridgeBinding,
    ForgeQueryDeclarationBridgeContinuationRequest, ForgeQueryDeclarationBridgeRouting,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};
use crate::basis_lifecycle::LowerRuntimeEvidenceAuthority;
use crate::identity::hash_parts;

use crate::continuation_pipeline::readmission::{
    ForgeQueryPreparedContinuationAuthorityWitness, ForgeQueryPreparedContinuationBasisKind,
    ForgeQueryPreparedContinuationBasisWitness, ForgeQueryPreparedContinuationDriftKind,
    ForgeQueryPreparedContinuationExecutionReadmission,
    ForgeQueryPreparedContinuationFreshnessPosture,
};
use crate::continuation_pipeline::ForgeQueryPreparedContinuation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ForgeQueryPreparedContinuationCurrentReadmissionEvidence {
    authority: LowerRuntimeEvidenceAuthority,
    basis_identity_digest: String,
    lower_runtime_binding_digest: Option<String>,
    freshness_posture: ForgeQueryPreparedContinuationFreshnessPosture,
    drift_kind: Option<ForgeQueryPreparedContinuationDriftKind>,
    evidence_digest: String,
}

impl ForgeQueryPreparedContinuationCurrentReadmissionEvidence {
    fn new(
        authority: LowerRuntimeEvidenceAuthority,
        basis_identity_digest: String,
        lower_runtime_binding_digest: Option<String>,
        freshness_posture: ForgeQueryPreparedContinuationFreshnessPosture,
        drift_kind: Option<ForgeQueryPreparedContinuationDriftKind>,
        evidence_digest: String,
    ) -> Self {
        Self {
            authority,
            basis_identity_digest,
            lower_runtime_binding_digest,
            freshness_posture,
            drift_kind,
            evidence_digest,
        }
    }
}

pub(crate) enum ForgeQueryPreparedContinuationExecutionReadmissionDenial {
    Stale(String),
    BasisMismatch(String),
    AuthorityMismatch(String),
    AsyncRequestDrift(String),
    ReplayDrift(String),
    RemaskDrift(String),
    PreviewCrossedResidue(String),
    StaleCompletion(String),
}

pub(crate) fn prepared_execution_readmission_from_routing<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    bridge_request: &ForgeQueryDeclarationBridgeContinuationRequest,
    routing: &ForgeQueryDeclarationBridgeRouting<D, I>,
    required_capability_families: Vec<crate::application::ForgeQueryCapabilityFamily>,
) -> ForgeQueryPreparedContinuationExecutionReadmission {
    let basis_witness = basis_witness_from_binding(bridge_request, routing.binding());
    let authority_witness = authority_witness_from_binding(routing.binding());
    ForgeQueryPreparedContinuationExecutionReadmission::new(
        basis_witness,
        authority_witness,
        ForgeQueryPreparedContinuationFreshnessPosture::Stable,
        None,
        required_capability_families,
    )
}

pub(crate) fn current_readmission_evidence_from_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    prepared: &ForgeQueryPreparedContinuation<D, I>,
) -> ForgeQueryPreparedContinuationCurrentReadmissionEvidence {
    let retained = prepared.execution_readmission();
    let observation = handle
        .operating_context()
        .continuation_execution_readmission_observation(retained, handle.support_snapshot());
    ForgeQueryPreparedContinuationCurrentReadmissionEvidence::new(
        observation.authority(),
        observation.basis_identity_digest().to_string(),
        observation
            .lower_runtime_binding_digest()
            .map(str::to_string),
        observation.freshness_posture(),
        observation.drift_kind(),
        hash_parts(&[
            "forge_query_continuation_execution_readmission_v1".to_string(),
            handle.handle_identity_digest().to_string(),
            handle.support_snapshot().snapshot_digest().to_string(),
            format!("basis:{}", observation.basis_identity_digest()),
            format!(
                "drift:{}",
                observation
                    .drift_kind()
                    .map(|kind| kind.as_str())
                    .unwrap_or("none")
            ),
        ]),
    )
}

pub(crate) fn validate_execution_readmission<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    prepared: &ForgeQueryPreparedContinuation<D, I>,
    current: &ForgeQueryPreparedContinuationCurrentReadmissionEvidence,
) -> Result<(), ForgeQueryPreparedContinuationExecutionReadmissionDenial> {
    if current.freshness_posture == ForgeQueryPreparedContinuationFreshnessPosture::Stale {
        return Err(
            ForgeQueryPreparedContinuationExecutionReadmissionDenial::Stale(
                "the retained continuation basis evidence is stale at execution time".to_string(),
            ),
        );
    }

    if let Some(drift_kind) = current.drift_kind {
        return Err(match drift_kind {
            ForgeQueryPreparedContinuationDriftKind::AsyncRequest => {
                ForgeQueryPreparedContinuationExecutionReadmissionDenial::AsyncRequestDrift(
                    "the retained continuation async request identity drifted before execution"
                        .to_string(),
                )
            }
            ForgeQueryPreparedContinuationDriftKind::Replay => {
                ForgeQueryPreparedContinuationExecutionReadmissionDenial::ReplayDrift(
                    "the retained continuation replay identity drifted before execution"
                        .to_string(),
                )
            }
            ForgeQueryPreparedContinuationDriftKind::Remask => {
                ForgeQueryPreparedContinuationExecutionReadmissionDenial::RemaskDrift(
                    "the retained continuation was remasked before execution".to_string(),
                )
            }
            ForgeQueryPreparedContinuationDriftKind::PreviewCrossedResidue => {
                ForgeQueryPreparedContinuationExecutionReadmissionDenial::PreviewCrossedResidue(
                    "the retained continuation crossed preview residue before execution"
                        .to_string(),
                )
            }
            ForgeQueryPreparedContinuationDriftKind::StaleCompletion => {
                ForgeQueryPreparedContinuationExecutionReadmissionDenial::StaleCompletion(
                    "the retained continuation completion state is stale at execution time"
                        .to_string(),
                )
            }
        });
    }

    let retained = prepared.execution_readmission();
    let witness = retained.basis_witness();
    if current.basis_identity_digest != witness.basis_identity_digest()
        || current.lower_runtime_binding_digest.as_deref()
            != witness.expected_lower_runtime_binding_digest()
    {
        return Err(
            ForgeQueryPreparedContinuationExecutionReadmissionDenial::BasisMismatch(
                format!(
                    "the current lower-runtime basis evidence no longer matches retained continuation basis {}",
                    witness.basis_identity_digest()
                ),
            ),
        );
    }

    let retained_authority = lower_runtime_authority_from_witness(retained.authority_witness());
    if current.authority != retained_authority {
        return Err(
            ForgeQueryPreparedContinuationExecutionReadmissionDenial::AuthorityMismatch(
                format!(
                    "the current lower-runtime authority {} no longer matches retained continuation authority {}",
                    current.authority.as_str(),
                    retained_authority.as_str()
                ),
            ),
        );
    }

    Ok(())
}

fn basis_witness_from_binding(
    bridge_request: &ForgeQueryDeclarationBridgeContinuationRequest,
    binding: &ForgeQueryDeclarationBridgeBinding,
) -> ForgeQueryPreparedContinuationBasisWitness {
    match binding {
        ForgeQueryDeclarationBridgeBinding::RuntimeRoute(request) => {
            ForgeQueryPreparedContinuationBasisWitness::new(
                ForgeQueryPreparedContinuationBasisKind::Current,
                request.commit_identity().to_string(),
                Some(request.commit_identity().to_string()),
                None,
            )
        }
        ForgeQueryDeclarationBridgeBinding::TruthView(request) => {
            let kind = match bridge_request.truth_context() {
                crate::application::ForgeQueryDeclarationBridgeTruthContext::Current => {
                    ForgeQueryPreparedContinuationBasisKind::Current
                }
                crate::application::ForgeQueryDeclarationBridgeTruthContext::Historical => {
                    ForgeQueryPreparedContinuationBasisKind::Historical
                }
                crate::application::ForgeQueryDeclarationBridgeTruthContext::Preview => {
                    ForgeQueryPreparedContinuationBasisKind::PreviewDerived
                }
            };
            ForgeQueryPreparedContinuationBasisWitness::new(
                kind,
                request.selector().digest().to_string(),
                Some(request.selector().digest().to_string()),
                request
                    .selector()
                    .snapshot_identity()
                    .map(
                        |identity: &forge_runtime_bridge::facade::TruthSnapshotIdentity| {
                            identity.as_str().to_string()
                        },
                    )
                    .or_else(|| {
                        request.selector().commit_identity().map(
                            |identity: &forge_runtime_bridge::facade::TruthCommitIdentity| {
                                identity.as_str().to_string()
                            },
                        )
                    }),
            )
        }
        ForgeQueryDeclarationBridgeBinding::PreviewSession(request) => {
            let declaration = request.declaration();
            ForgeQueryPreparedContinuationBasisWitness::new(
                ForgeQueryPreparedContinuationBasisKind::PreviewDerived,
                declaration.digest().to_string(),
                Some(declaration.truth_view_basis_digest().to_string()),
                Some(declaration.truth_view_basis_digest().to_string()),
            )
        }
        ForgeQueryDeclarationBridgeBinding::PreviewPromotion(binding) => {
            ForgeQueryPreparedContinuationBasisWitness::new(
                ForgeQueryPreparedContinuationBasisKind::PreviewDerived,
                binding.promotion_continuation_digest().to_string(),
                Some(binding.preview_basis_digest().to_string()),
                Some(binding.declaration_digest().to_string()),
            )
        }
        ForgeQueryDeclarationBridgeBinding::SubscriptionPreparation(request) => {
            ForgeQueryPreparedContinuationBasisWitness::new(
                basis_kind_for_truth_context(bridge_request.truth_context()),
                request.authority_digest().to_string(),
                Some(request.authority_digest().to_string()),
                request
                    .child_basis_digests()
                    .first()
                    .map(|digest| digest.as_ref().to_string()),
            )
        }
        ForgeQueryDeclarationBridgeBinding::WritebackPreparation(request) => {
            ForgeQueryPreparedContinuationBasisWitness::new(
                basis_kind_for_truth_context(bridge_request.truth_context()),
                request.causality().digest().to_string(),
                Some(
                    request
                        .declaration()
                        .strategy_descriptor_digest()
                        .to_string(),
                ),
                Some(request.effect_intent().digest().to_string()),
            )
        }
    }
}

fn authority_witness_from_binding(
    binding: &ForgeQueryDeclarationBridgeBinding,
) -> ForgeQueryPreparedContinuationAuthorityWitness {
    match binding {
        ForgeQueryDeclarationBridgeBinding::RuntimeRoute(_) => {
            ForgeQueryPreparedContinuationAuthorityWitness::Runtime
        }
        ForgeQueryDeclarationBridgeBinding::TruthView(_) => {
            ForgeQueryPreparedContinuationAuthorityWitness::RelationalFacade
        }
        ForgeQueryDeclarationBridgeBinding::PreviewSession(_)
        | ForgeQueryDeclarationBridgeBinding::PreviewPromotion(_)
        | ForgeQueryDeclarationBridgeBinding::SubscriptionPreparation(_)
        | ForgeQueryDeclarationBridgeBinding::WritebackPreparation(_) => {
            ForgeQueryPreparedContinuationAuthorityWitness::RuntimeBridgeFacade
        }
    }
}

fn basis_kind_for_truth_context(
    truth_context: crate::application::ForgeQueryDeclarationBridgeTruthContext,
) -> ForgeQueryPreparedContinuationBasisKind {
    match truth_context {
        crate::application::ForgeQueryDeclarationBridgeTruthContext::Current => {
            ForgeQueryPreparedContinuationBasisKind::Current
        }
        crate::application::ForgeQueryDeclarationBridgeTruthContext::Historical => {
            ForgeQueryPreparedContinuationBasisKind::Historical
        }
        crate::application::ForgeQueryDeclarationBridgeTruthContext::Preview => {
            ForgeQueryPreparedContinuationBasisKind::PreviewDerived
        }
    }
}

fn lower_runtime_authority_from_witness(
    witness: ForgeQueryPreparedContinuationAuthorityWitness,
) -> LowerRuntimeEvidenceAuthority {
    match witness {
        ForgeQueryPreparedContinuationAuthorityWitness::Runtime => {
            LowerRuntimeEvidenceAuthority::Runtime
        }
        ForgeQueryPreparedContinuationAuthorityWitness::RuntimeBridgeFacade => {
            LowerRuntimeEvidenceAuthority::RuntimeBridgeFacade
        }
        ForgeQueryPreparedContinuationAuthorityWitness::RelationalFacade => {
            LowerRuntimeEvidenceAuthority::RelationalFacade
        }
        ForgeQueryPreparedContinuationAuthorityWitness::SignalFacade => {
            LowerRuntimeEvidenceAuthority::SignalFacade
        }
    }
}

#[cfg(test)]
pub(crate) fn drifted_observation_from_retained(
    retained: &ForgeQueryPreparedContinuationExecutionReadmission,
    freshness_posture: ForgeQueryPreparedContinuationFreshnessPosture,
    basis_identity_digest: Option<String>,
    authority: Option<LowerRuntimeEvidenceAuthority>,
    drift_kind: Option<ForgeQueryPreparedContinuationDriftKind>,
) -> ForgeQueryContinuationExecutionReadmissionObservation {
    let witness = retained.basis_witness();
    ForgeQueryContinuationExecutionReadmissionObservation::new(
        authority
            .unwrap_or_else(|| lower_runtime_authority_from_witness(retained.authority_witness())),
        basis_identity_digest.unwrap_or_else(|| witness.basis_identity_digest().to_string()),
        witness
            .expected_lower_runtime_binding_digest()
            .map(str::to_string),
        freshness_posture,
        drift_kind,
    )
}
