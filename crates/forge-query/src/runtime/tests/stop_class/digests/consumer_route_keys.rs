use super::super::super::support::*;
use super::super::consumer_support::routing::ConsumerStopRoute;

pub(super) fn consumer_stop_route_key(route: &ConsumerStopRoute) -> String {
    match route {
        ConsumerStopRoute::MissingRuntimeComponent(component) => {
            format!(
                "missing_runtime_component:{}",
                missing_component_key(*component)
            )
        }
        ConsumerStopRoute::ExistingTruthAssertionDenied(kind) => {
            format!("existing_truth_assertion_denied:{kind:?}")
        }
        ConsumerStopRoute::ExistingTruthProbeDenied(kind) => {
            format!("existing_truth_probe_denied:{kind:?}")
        }
        ConsumerStopRoute::MutationBindingDenied(kind) => {
            format!("mutation_binding_denied:{kind:?}")
        }
        ConsumerStopRoute::MutationContinuityDenied(kind) => {
            format!("mutation_continuity_denied:{kind:?}")
        }
        ConsumerStopRoute::GraphCompositionDenied(kind) => {
            format!("graph_composition_denied:{kind:?}")
        }
        ConsumerStopRoute::GraphCompositionDomainInvariantDenied {
            hook_family,
            invariant_family,
        } => format!("graph_composition_domain_invariant_denied:{hook_family}:{invariant_family}"),
        ConsumerStopRoute::MutationNamingDenied(kind) => {
            format!("mutation_naming_denied:{kind:?}")
        }
        ConsumerStopRoute::MutationTargetReferenceDenied(kind) => {
            format!("mutation_target_reference_denied:{kind:?}")
        }
        ConsumerStopRoute::ReadCompositionDenied(kind) => {
            format!("read_composition_denied:{kind:?}")
        }
        ConsumerStopRoute::ReadCompositionDomainInvariantDenied {
            hook_family,
            invariant_family,
        } => format!("read_composition_domain_invariant_denied:{hook_family}:{invariant_family}"),
        ConsumerStopRoute::WorkspaceDenied => "workspace_denied".to_string(),
        ConsumerStopRoute::ProgramDenied => "program_denied".to_string(),
        ConsumerStopRoute::RuntimeLookupDenied(kind) => {
            format!("runtime_lookup_denied:{kind:?}")
        }
        ConsumerStopRoute::MissingRuntimeArtifact(kind) => {
            format!("missing_runtime_artifact:{kind:?}")
        }
        ConsumerStopRoute::RuntimeDeclarationDenied(kind) => {
            format!("runtime_declaration_denied:{kind:?}")
        }
        ConsumerStopRoute::PreviewOperationEffectDenied(label_identity) => {
            format!(
                "preview_operation_effect_denied:{}",
                label_identity.as_str()
            )
        }
        ConsumerStopRoute::UnsupportedAuthorityRequirement(requirement) => {
            format!("unsupported_authority_requirement:{}", requirement.as_str())
        }
        ConsumerStopRoute::ExistingTruthAssertionRequiresAuthorityLane(required_lane) => {
            format!(
                "existing_truth_assertion_requires_authority_lane:{}",
                required_lane.as_str()
            )
        }
        ConsumerStopRoute::IntentCommitDenied => "intent_commit_denied".to_string(),
        ConsumerStopRoute::IntentExecutionRoutingFailed(kind) => {
            format!("intent_execution_routing_failed:{kind:?}")
        }
        ConsumerStopRoute::EffectPolicyDenied => "effect_policy_denied".to_string(),
        ConsumerStopRoute::SharedReadStaleBasis => "shared_read_stale_basis".to_string(),
        ConsumerStopRoute::JournalReplayDenied(kind) => {
            format!("journal_replay_denied:{}", kind.as_str())
        }
        ConsumerStopRoute::PreviewPromotionDenied(kind) => {
            format!("preview_promotion_denied:{kind:?}")
        }
        ConsumerStopRoute::FamilyAdmissionDenied {
            family,
            status,
            teaching_posture,
        } => format!(
            "family_admission_denied:{}:{}:{}",
            family.as_str(),
            status.as_str(),
            teaching_posture
                .map(ForgeQueryRuntimeFamilyTeachingPosture::as_str)
                .unwrap_or("none")
        ),
        ConsumerStopRoute::SessionLabelCollision(authority_lane) => {
            format!("session_label_collision:{}", authority_lane.as_str())
        }
    }
}

fn missing_component_key(component: ForgeQueryRuntimeMissingComponent) -> &'static str {
    match component {
        ForgeQueryRuntimeMissingComponent::Backend => "backend",
        ForgeQueryRuntimeMissingComponent::RuntimeBridge => "runtime_bridge",
        ForgeQueryRuntimeMissingComponent::SchemaAdapter => "schema_adapter",
        ForgeQueryRuntimeMissingComponent::SnapshotIdentityAdapter => "snapshot_identity_adapter",
        ForgeQueryRuntimeMissingComponent::SourceAdapter => "source_adapter",
        ForgeQueryRuntimeMissingComponent::WriteAuthority => "write_authority",
        ForgeQueryRuntimeMissingComponent::SignalSink => "signal_sink",
        ForgeQueryRuntimeMissingComponent::SubscriptionActivation => "subscription_activation",
        ForgeQueryRuntimeMissingComponent::PreviewBasis => "preview_basis",
        ForgeQueryRuntimeMissingComponent::InspectorEvidence => "inspector_evidence",
        ForgeQueryRuntimeMissingComponent::IntentAuthority => "intent_authority",
    }
}
