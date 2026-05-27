use std::marker::PhantomData;

use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryBridgeContinuationAuthority, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDeclarationBridgeContinuationMode,
    ForgeQueryDeclarationBridgeContinuationRequest, ForgeQueryDeclarationBridgeTruthContext,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDeclarationSignalCompatibilityContract,
    ForgeQueryDeclarationSignalCompatibilityInput, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQuerySignalCompatiblePosture,
};

use crate::signal_compatibility_orchestration::{
    ForgeQuerySignalCompatibilityOrchestration, ForgeQuerySignalCompatibilityOrchestrationClass,
    ForgeQuerySignalCompatibilityOrchestrationInput,
    ForgeQuerySignalCompatibilityOrchestrationOutcome,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct SignalDomain;

impl ForgeQueryDomainEntryMarker for SignalDomain {
    fn domain_key(&self) -> &'static str {
        "test.signal.orchestration.domain"
    }

    fn display_name(&self) -> &'static str {
        "SignalOrchestrationDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct SignalWorld(pub(super) &'static str);

impl ForgeQueryDomainOperatingContext<SignalDomain> for SignalWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
            ForgeQueryCapabilityFamily::WorkflowOrchestration,
            ForgeQueryCapabilityFamily::PreviewSession,
            ForgeQueryCapabilityFamily::QueryComposition,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
            ForgeQueryConfigSectionFamily::Signal,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("signal-world-{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct SignalFamily;

impl ForgeQueryDeclarationFamilyMarker<SignalDomain> for SignalFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "SignalFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &[
                "selection.face",
                "selection.active_face",
                "signal.dependency.runtime_inputs",
            ],
            &["selection.neighborhood.local_topology"],
            &["signal.preview.surface"],
            &["signal.private_authority"],
            &["signal.conflicting_dependency"],
        )
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_slices(
            &[
                "selection.face",
                "selection.active_face",
                "signal.dependency.runtime_inputs",
                "selection.neighborhood.local_topology",
                "signal.preview.surface",
            ],
            &[],
            &[],
        )
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn bridge_continuation_contract(
    ) -> Option<crate::application::ForgeQueryDeclarationBridgeContinuationContract> {
        Some(
            crate::application::ForgeQueryDeclarationBridgeContinuationContract::runtime_route_current(),
        )
    }

    fn signal_compatibility_contract() -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
        Some(
            ForgeQueryDeclarationSignalCompatibilityContract::runtime_derived_execution()
                .with_aspects(
                    ForgeQueryDeclarationAspectContract::from_slices(
                        &["selection.active_face", "signal.dependency.runtime_inputs"],
                        &["selection.neighborhood.local_topology"],
                        &[],
                        &["signal.private_authority"],
                        &["signal.conflicting_dependency"],
                    ),
                    ForgeQueryDeclarationAspectContract::from_slices(
                        &["signal.produced.derived_face_preview"],
                        &["signal.produced.material_projection"],
                        &["signal.produced.preview.surface"],
                        &[],
                        &[],
                    ),
                ),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct SignalInput {
    id: &'static str,
    _marker: PhantomData<SignalFamily>,
}

impl SignalInput {
    pub(super) fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl ForgeQueryDeclarationInput<SignalDomain> for SignalInput {
    type Family = SignalFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

pub(super) fn admitted_handle(
    world: &'static str,
) -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<SignalDomain, SignalWorld> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(SignalDomain)
        .with_operating_context(SignalWorld(world))
        .validate()
        .unwrap()
        .admit()
        .unwrap()
}

pub(super) fn bridge_request() -> ForgeQueryDeclarationBridgeContinuationRequest {
    ForgeQueryDeclarationBridgeContinuationRequest::new(
        ForgeQueryDeclarationBridgeContinuationMode::RuntimeRoute,
        ForgeQueryDeclarationBridgeTruthContext::Current,
    )
}

pub(super) fn envelope(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
        SignalDomain,
        SignalWorld,
    >,
    id: &'static str,
) -> crate::application::ForgeQueryDeclarationEnvelope<SignalDomain, SignalInput> {
    let progressed = handle
        .declare_review_and_progress(SignalInput::new(id))
        .unwrap_or_else(|_| panic!("expected progressed signal declaration"));
    handle
        .envelope_routes_from_progressed(progressed)
        .unwrap_or_else(|_| panic!("expected signal envelope"))
}

pub(super) fn orchestration_input(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
        SignalDomain,
        SignalWorld,
    >,
    id: &'static str,
) -> ForgeQuerySignalCompatibilityOrchestrationInput<SignalDomain, SignalInput> {
    ForgeQuerySignalCompatibilityOrchestrationInput::new(
        ForgeQueryDeclarationSignalCompatibilityInput::enveloped(envelope(handle, id)),
    )
}

pub(super) fn progressed_input(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
        SignalDomain,
        SignalWorld,
    >,
    id: &'static str,
) -> ForgeQuerySignalCompatibilityOrchestrationInput<SignalDomain, SignalInput> {
    ForgeQuerySignalCompatibilityOrchestrationInput::from_progressed(
        handle
            .declare_review_and_progress(SignalInput::new(id))
            .unwrap_or_else(|_| panic!("expected progressed signal declaration")),
    )
}

#[derive(Debug, Eq, PartialEq)]
pub(super) enum OutcomeDigestToken {
    Compatible(String),
    Prepared {
        prepared_digest: String,
        signal_compatibility_digest: Option<String>,
    },
    Status(&'static str),
}

pub(super) fn orchestration_outcome_token(
    outcome: &ForgeQuerySignalCompatibilityOrchestrationOutcome<SignalDomain, SignalInput>,
) -> OutcomeDigestToken {
    match outcome {
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Bound(value) => match value.class() {
            ForgeQuerySignalCompatibilityOrchestrationClass::Compatible => {
                OutcomeDigestToken::Compatible(
                    value
                        .signal_execution_family()
                        .map(|family| format!("{family:?}:{:?}", value.envelope_digest()))
                        .unwrap_or_else(|| format!("compatible:{:?}", value.envelope_digest())),
                )
            }
            ForgeQuerySignalCompatibilityOrchestrationClass::Prepared => match value {
                ForgeQuerySignalCompatibilityOrchestration::Prepared(prepared) => {
                    OutcomeDigestToken::Prepared {
                        prepared_digest: prepared.prepared_digest().to_string(),
                        signal_compatibility_digest: prepared
                            .signal_compatibility_digest()
                            .map(str::to_string),
                    }
                }
                _ => unreachable!(),
            },
        },
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Ambiguous(_) => {
            OutcomeDigestToken::Status("ambiguous")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Unavailable(_) => {
            OutcomeDigestToken::Status("unavailable")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::WrongWorld(_) => {
            OutcomeDigestToken::Status("wrong_world")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::WrongHandle(_) => {
            OutcomeDigestToken::Status("wrong_handle")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Stale(_) => {
            OutcomeDigestToken::Status("stale")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::RebindRequired(_) => {
            OutcomeDigestToken::Status("rebind_required")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::MissingRequiredAspect(_) => {
            OutcomeDigestToken::Status("missing_required_aspect")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::AspectConflict(_) => {
            OutcomeDigestToken::Status("aspect_conflict")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::AuthorityMismatch(_) => {
            OutcomeDigestToken::Status("authority_mismatch")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::BasisMismatch(_) => {
            OutcomeDigestToken::Status("basis_mismatch")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Deferred(_) => {
            OutcomeDigestToken::Status("deferred")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Denied(_) => {
            OutcomeDigestToken::Status("denied")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Unsupported(_) => {
            OutcomeDigestToken::Status("unsupported")
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Failed(_) => {
            OutcomeDigestToken::Status("failed")
        }
    }
}

pub(super) fn continuation_outcome_token(
    outcome: &crate::continuation_pipeline::ForgeQueryPreparedContinuationOutcome<
        SignalDomain,
        SignalInput,
    >,
) -> OutcomeDigestToken {
    match outcome {
        crate::continuation_pipeline::ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => {
            OutcomeDigestToken::Prepared {
                prepared_digest: prepared.prepared_digest().to_string(),
                signal_compatibility_digest: prepared
                    .signal_compatibility_digest()
                    .map(str::to_string),
            }
        }
        crate::continuation_pipeline::ForgeQueryPreparedContinuationOutcome::Ambiguous(_) => {
            OutcomeDigestToken::Status("ambiguous")
        }
        crate::continuation_pipeline::ForgeQueryPreparedContinuationOutcome::Unavailable(_) => {
            OutcomeDigestToken::Status("unavailable")
        }
        crate::continuation_pipeline::ForgeQueryPreparedContinuationOutcome::WrongWorld(_) => {
            OutcomeDigestToken::Status("wrong_world")
        }
        crate::continuation_pipeline::ForgeQueryPreparedContinuationOutcome::WrongHandle(_) => {
            OutcomeDigestToken::Status("wrong_handle")
        }
        crate::continuation_pipeline::ForgeQueryPreparedContinuationOutcome::Stale(_) => {
            OutcomeDigestToken::Status("stale")
        }
        crate::continuation_pipeline::ForgeQueryPreparedContinuationOutcome::RebindRequired(_) => {
            OutcomeDigestToken::Status("rebind_required")
        }
        crate::continuation_pipeline::ForgeQueryPreparedContinuationOutcome::AuthorityMismatch(
            _,
        ) => OutcomeDigestToken::Status("authority_mismatch"),
        crate::continuation_pipeline::ForgeQueryPreparedContinuationOutcome::BasisMismatch(_) => {
            OutcomeDigestToken::Status("basis_mismatch")
        }
        crate::continuation_pipeline::ForgeQueryPreparedContinuationOutcome::Unsupported(_) => {
            OutcomeDigestToken::Status("unsupported")
        }
        crate::continuation_pipeline::ForgeQueryPreparedContinuationOutcome::Deferred(_) => {
            OutcomeDigestToken::Status("deferred")
        }
        crate::continuation_pipeline::ForgeQueryPreparedContinuationOutcome::Denied(_) => {
            OutcomeDigestToken::Status("denied")
        }
        crate::continuation_pipeline::ForgeQueryPreparedContinuationOutcome::Failed(_) => {
            OutcomeDigestToken::Status("failed")
        }
    }
}
