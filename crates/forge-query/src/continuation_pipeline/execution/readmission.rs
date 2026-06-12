#[cfg(test)]
use crate::application::ForgeQueryContinuationExecutionReadmissionObservation;
use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationBridgeBinding,
    ForgeQueryDeclarationBridgeContinuationRequest, ForgeQueryDeclarationBridgeRouting,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};
use crate::basis_lifecycle::LowerRuntimeEvidenceAuthority;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::memory_workspace::{ForgeQueryCommitIdentity, ForgeQuerySnapshotIdentity};

use crate::continuation_pipeline::readmission::{
    continuation_readmission_basis_identity,
    continuation_readmission_lower_runtime_binding_identity,
    continuation_readmission_source_basis_identity, ForgeQueryPreparedContinuationAuthorityWitness,
    ForgeQueryPreparedContinuationBasisKind, ForgeQueryPreparedContinuationBasisWitness,
    ForgeQueryPreparedContinuationDriftKind, ForgeQueryPreparedContinuationExecutionReadmission,
    ForgeQueryPreparedContinuationFreshnessPosture,
};
use crate::continuation_pipeline::ForgeQueryPreparedContinuation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ForgeQueryPreparedContinuationCurrentReadmissionEvidence {
    authority: LowerRuntimeEvidenceAuthority,
    basis_identity: ForgeQueryEvidenceIdentity,
    lower_runtime_binding_identity: Option<ForgeQueryEvidenceIdentity>,
    freshness_posture: ForgeQueryPreparedContinuationFreshnessPosture,
    drift_kind: Option<ForgeQueryPreparedContinuationDriftKind>,
    evidence_digest: String,
}

impl ForgeQueryPreparedContinuationCurrentReadmissionEvidence {
    fn new(
        authority: LowerRuntimeEvidenceAuthority,
        basis_identity: ForgeQueryEvidenceIdentity,
        lower_runtime_binding_identity: Option<ForgeQueryEvidenceIdentity>,
        freshness_posture: ForgeQueryPreparedContinuationFreshnessPosture,
        drift_kind: Option<ForgeQueryPreparedContinuationDriftKind>,
        evidence_digest: String,
    ) -> Self {
        Self {
            authority,
            basis_identity,
            lower_runtime_binding_identity,
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
        observation.basis_identity().clone(),
        observation.lower_runtime_binding_identity().cloned(),
        observation.freshness_posture(),
        observation.drift_kind(),
        forge_query_evidence_identity(
            ForgeQueryEvidenceScope::ContinuationExecutionReadmissionEvidence,
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("handle"),
            handle.handle_identity_digest(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("support_snapshot"),
            handle.support_snapshot().snapshot_digest(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis"),
            observation.basis_identity(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("lower_runtime_binding"),
            observation.lower_runtime_binding_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("freshness"),
            observation.freshness_posture().as_str(),
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("drift"),
            observation.drift_kind().map(|kind| kind.as_str()),
        )
        .seal()
        .as_str()
        .to_string(),
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
    if &current.basis_identity != witness.basis_identity()
        || current.lower_runtime_binding_identity.as_ref()
            != witness.expected_lower_runtime_binding_identity()
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
            let commit_identity = bridge_commit_evidence_identity(request.commit_identity());
            ForgeQueryPreparedContinuationBasisWitness::new(
                ForgeQueryPreparedContinuationBasisKind::Current,
                commit_identity.clone(),
                Some(commit_identity),
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
            let selector_identity =
                continuation_readmission_basis_identity(kind, request.selector().digest());
            ForgeQueryPreparedContinuationBasisWitness::new(
                kind,
                selector_identity.clone(),
                Some(continuation_readmission_lower_runtime_binding_identity(
                    request.selector().digest(),
                )),
                request
                    .selector()
                    .snapshot_identity()
                    .map(bridge_snapshot_evidence_identity)
                    .or_else(|| {
                        request
                            .selector()
                            .commit_identity()
                            .map(bridge_commit_evidence_identity)
                    }),
            )
        }
        ForgeQueryDeclarationBridgeBinding::PreviewSession(request) => {
            let declaration = request.declaration();
            ForgeQueryPreparedContinuationBasisWitness::new(
                ForgeQueryPreparedContinuationBasisKind::PreviewDerived,
                continuation_readmission_basis_identity(
                    ForgeQueryPreparedContinuationBasisKind::PreviewDerived,
                    declaration.digest(),
                ),
                Some(continuation_readmission_lower_runtime_binding_identity(
                    declaration.truth_view_basis_digest(),
                )),
                Some(continuation_readmission_source_basis_identity(
                    declaration.truth_view_basis_digest(),
                )),
            )
        }
        ForgeQueryDeclarationBridgeBinding::PreviewPromotion(binding) => {
            ForgeQueryPreparedContinuationBasisWitness::new(
                ForgeQueryPreparedContinuationBasisKind::PreviewDerived,
                continuation_readmission_basis_identity(
                    ForgeQueryPreparedContinuationBasisKind::PreviewDerived,
                    binding.promotion_continuation_digest(),
                ),
                Some(continuation_readmission_lower_runtime_binding_identity(
                    binding.preview_basis_digest(),
                )),
                Some(continuation_readmission_source_basis_identity(
                    binding.declaration_digest(),
                )),
            )
        }
        ForgeQueryDeclarationBridgeBinding::SubscriptionPreparation(request) => {
            let basis_kind = basis_kind_for_truth_context(bridge_request.truth_context());
            ForgeQueryPreparedContinuationBasisWitness::new(
                basis_kind,
                continuation_readmission_basis_identity(basis_kind, request.authority_digest()),
                Some(continuation_readmission_lower_runtime_binding_identity(
                    request.authority_digest(),
                )),
                request
                    .child_basis_digests()
                    .first()
                    .map(|digest| continuation_readmission_source_basis_identity(digest.as_ref())),
            )
        }
        ForgeQueryDeclarationBridgeBinding::WritebackPreparation(request) => {
            let basis_kind = basis_kind_for_truth_context(bridge_request.truth_context());
            ForgeQueryPreparedContinuationBasisWitness::new(
                basis_kind,
                continuation_readmission_basis_identity(basis_kind, request.causality().digest()),
                Some(continuation_readmission_lower_runtime_binding_identity(
                    request.declaration().strategy_descriptor_digest(),
                )),
                Some(continuation_readmission_source_basis_identity(
                    request.effect_intent().digest(),
                )),
            )
        }
    }
}

fn bridge_commit_evidence_identity(
    identity: &forge_runtime_bridge::facade::TruthCommitIdentity,
) -> ForgeQueryEvidenceIdentity {
    let commit_id = identity
        .relational_commit_id()
        .expect("continuation readmission commit identity must carry relational payload");
    ForgeQueryCommitIdentity::from_relational_commit_id(commit_id).evidence_identity()
}

fn bridge_snapshot_evidence_identity(
    identity: &forge_runtime_bridge::facade::TruthSnapshotIdentity,
) -> ForgeQueryEvidenceIdentity {
    let parts = identity
        .relational_snapshot_parts()
        .expect("continuation readmission snapshot identity must carry relational payload");
    ForgeQuerySnapshotIdentity::from_relational_snapshot(parts).evidence_identity()
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
        basis_identity_digest
            .map(|identity| continuation_readmission_basis_identity(witness.kind(), identity))
            .unwrap_or_else(|| witness.basis_identity().clone()),
        witness.expected_lower_runtime_binding_identity().cloned(),
        freshness_posture,
        drift_kind,
    )
}
