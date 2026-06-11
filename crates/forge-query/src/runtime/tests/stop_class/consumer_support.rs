use super::super::support::*;
use crate::schema_view::{QuerySchemaView, SchemaRelationView};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum ConsumerStopRoute {
    MissingRuntimeComponent(ForgeQueryRuntimeMissingComponent),
    ExistingTruthAssertionDenied(ForgeQueryExistingTruthAssertionDenialKind),
    ExistingTruthProbeDenied(ForgeQueryExistingTruthProbeDenialKind),
    MutationBindingDenied(ForgeQueryExistingTruthBindingDenialKind),
    MutationContinuityDenied(ForgeQueryContinuityMutationDenialKind),
    GraphCompositionDenied(ForgeQueryGraphCompositionDenialKind),
    GraphCompositionDomainInvariantDenied,
    MutationNamingDenied(ForgeQueryNamingMutationDenialKind),
    MutationTargetReferenceDenied(ForgeQuerySymbolicTargetReferenceDenialKind),
    ReadCompositionDenied(ForgeQueryReadDenialKind),
    ReadCompositionDomainInvariantDenied(&'static str),
    WorkspaceDenied,
    ProgramDenied,
    RuntimeLookupDenied(ForgeQueryRuntimeLookupFailureKind),
    MissingRuntimeArtifact(ForgeQueryRuntimeMissingArtifactKind),
    RuntimeDeclarationDenied(ForgeQueryRuntimeDeclarationFailureKind),
    UnsupportedAuthority,
    IntentCommitDenied,
    IntentExecutionRoutingFailed(ForgeQueryRuntimeDeclarationFailureKind),
    EffectPolicyDenied,
    PreviewPromotionDenied(ForgeQueryPreviewPromotionDenialKind),
    FamilyAdmissionDenied {
        family: ForgeQueryRuntimeFacadeFamily,
        status: ForgeQueryRuntimeFamilySupportStatus,
        teaching_posture: Option<ForgeQueryRuntimeFamilyTeachingPosture>,
    },
    SessionLabelCollision(ForgeQueryAuthorityLane),
}

pub(super) fn route_consumer_stop_class(error: &ForgeQueryRuntimeError) -> ConsumerStopRoute {
    match error.stop_class() {
        ForgeQueryStopClass::MissingRuntimeComponent { component } => {
            ConsumerStopRoute::MissingRuntimeComponent(component)
        }
        ForgeQueryStopClass::ExistingTruthAssertionDenied { denial } => {
            ConsumerStopRoute::ExistingTruthAssertionDenied(denial.kind())
        }
        ForgeQueryStopClass::ExistingTruthProbeDenied { denial } => {
            ConsumerStopRoute::ExistingTruthProbeDenied(denial.kind())
        }
        ForgeQueryStopClass::MutationBindingDenied { denial } => {
            ConsumerStopRoute::MutationBindingDenied(denial.kind())
        }
        ForgeQueryStopClass::MutationContinuityDenied { denial } => {
            ConsumerStopRoute::MutationContinuityDenied(denial.kind())
        }
        ForgeQueryStopClass::GraphCompositionDenied { denial } => {
            ConsumerStopRoute::GraphCompositionDenied(denial.kind())
        }
        ForgeQueryStopClass::GraphCompositionDomainInvariantDenied { .. } => {
            ConsumerStopRoute::GraphCompositionDomainInvariantDenied
        }
        ForgeQueryStopClass::MutationNamingDenied { denial } => {
            ConsumerStopRoute::MutationNamingDenied(denial.kind())
        }
        ForgeQueryStopClass::MutationTargetReferenceDenied { denial } => {
            ConsumerStopRoute::MutationTargetReferenceDenied(denial.kind())
        }
        ForgeQueryStopClass::ReadCompositionDenied { denial } => {
            ConsumerStopRoute::ReadCompositionDenied(denial.kind().clone())
        }
        ForgeQueryStopClass::ReadCompositionDomainInvariantDenied { .. } => {
            ConsumerStopRoute::ReadCompositionDomainInvariantDenied("domain_invariant_pack_hook")
        }
        ForgeQueryStopClass::Workspace { .. } => ConsumerStopRoute::WorkspaceDenied,
        ForgeQueryStopClass::Program { .. } => ConsumerStopRoute::ProgramDenied,
        ForgeQueryStopClass::RuntimeLookupFailed { kind, .. } => {
            ConsumerStopRoute::RuntimeLookupDenied(kind)
        }
        ForgeQueryStopClass::MissingRuntimeArtifact { kind, .. } => {
            ConsumerStopRoute::MissingRuntimeArtifact(kind)
        }
        ForgeQueryStopClass::RuntimeDeclarationFailed { kind, .. } => {
            ConsumerStopRoute::RuntimeDeclarationDenied(kind)
        }
        ForgeQueryStopClass::UnsupportedAuthority { .. } => ConsumerStopRoute::UnsupportedAuthority,
        ForgeQueryStopClass::IntentCommitDenied { .. } => ConsumerStopRoute::IntentCommitDenied,
        ForgeQueryStopClass::IntentExecutionRoutingFailed { source, .. } => {
            match source.stop_class() {
                ForgeQueryStopClass::RuntimeDeclarationFailed { kind, .. } => {
                    ConsumerStopRoute::IntentExecutionRoutingFailed(kind)
                }
                other => panic!("consumer source route expected declaration stop, got {other:?}"),
            }
        }
        ForgeQueryStopClass::EffectPolicyDenied { .. } => ConsumerStopRoute::EffectPolicyDenied,
        ForgeQueryStopClass::PreviewPromotionDenied { kind, .. } => {
            ConsumerStopRoute::PreviewPromotionDenied(kind)
        }
        ForgeQueryStopClass::FamilyAdmissionDenied {
            family,
            status,
            teaching_posture,
            ..
        } => ConsumerStopRoute::FamilyAdmissionDenied {
            family,
            status,
            teaching_posture,
        },
        ForgeQueryStopClass::SessionLabelCollision { authority_lane, .. } => {
            ConsumerStopRoute::SessionLabelCollision(authority_lane)
        }
    }
}

pub(super) fn existing_binding() -> ForgeQueryExistingTruthTargetBinding {
    ForgeQueryExistingTruthTargetBinding::direct_entity("authority:task-1", "Task:1")
        .expect("binding should build")
        .in_target_collection("Task")
        .expect("collection should build")
}

pub(super) fn expanded_manager_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "runtime-read-composition-expanded",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("profile", "display_name", SchemaFieldKind::String),
        ],
        [SchemaRelationView::new("manager", 2)],
    )
}
