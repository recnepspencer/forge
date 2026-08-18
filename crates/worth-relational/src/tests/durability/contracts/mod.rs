mod adjacency_checkpoint;
mod branch_and_aspect_history;
mod checkpoint_recovery;
mod descriptor_continuity;
mod merge_replay_continuity;
mod rejected_and_parent_closure;
mod retention_inspection_store;
mod schema_transition_continuity;
mod tail_checkpoint_validation;

use crate::diagnostics::data::RelationalDiagnosticValue;
use crate::facade::diagnostics::{DiagnosticCode, DiagnosticsScope};
use crate::facade::durability::{
    DurabilityMode, DurableStore, DurableStoreLayout, RecoveryAuthorityContinuityCheck,
    RecoveryAuthorityContinuityMismatch, RecoveryAuthorityParity, RecoveryCursor,
    RecoveryFailureClass, RecoveryIntegrityReport, RecoveryPlan, RecoveryVerificationMode,
    RecoveryVerificationOutcome, RelationIntegrityContractFamily,
};
use crate::facade::history::BranchId;
use crate::facade::identity::KindId;
use crate::facade::indexes::{
    DerivedIndexBuildRequest, DerivedIndexDefinition, DerivedIndexId, DerivedIndexKind,
};
use crate::facade::lineage::{
    CorrespondencePromotionRejectionClass, LineageDecisionKind, LineageEventKind,
};
use crate::facade::merge::{MergeExecutionRequest, MergeIntent};
use crate::facade::replay::ReplayVerificationLayer;
use crate::facade::runtime::RelationalRuntimeApi;
use crate::facade::schema::{DescriptorCanonicalBasisVersion, DescriptorSemanticsVersion};
use crate::facade::schema::{
    EntityKindRegistration, HistoricalInterpretationSensitivity, KindAspectContractDeclarations,
    ProposedSchemaTransition, RelationalSchemaRegistry, SchemaDiffAtom, SchemaDiffDetail,
    SchemaElementKind, SchemaElementRef, SchemaId, SchemaPublicationImpact,
    SchemaReconciliationPolicy, SchemaStratum, SchemaSubscriberImpact, SchemaVersionId,
};
use crate::facade::transactions::TransactionCommitError;
use crate::tests::support::*;

// CONTRACT: durability
// LANES: success, failure, recovery

fn schema_transition_for_subscriber_impact(
    target_schema_version_id: SchemaVersionId,
    subscriber_impact: SchemaSubscriberImpact,
) -> ProposedSchemaTransition {
    ProposedSchemaTransition {
        source_schema_id: SchemaId("test".to_string()),
        source_schema_version_id: SchemaVersionId(target_schema_version_id.0 - 1),
        target_schema_id: SchemaId("test".to_string()),
        target_schema_version_id,
        diff_atoms: vec![SchemaDiffAtom::new(
            SchemaElementRef::new(
                SchemaElementKind::Field,
                SchemaId("test".to_string()),
                target_schema_version_id,
                Some(KindId(1)),
                "tag",
            ),
            vec![
                SchemaStratum::StructuralShape,
                SchemaStratum::PublicationContract,
            ],
            SchemaPublicationImpact::ObservableSurfaceChanged,
            subscriber_impact,
            HistoricalInterpretationSensitivity::NotSensitive,
            SchemaDiffDetail::AddedField {
                field: field_key("tag"),
                required: false,
                default_expression: Some("null".into()),
            },
        )
        .with_boundary_visibility_proof(match subscriber_impact {
            SchemaSubscriberImpact::ConsumableSurfaceChanged => {
                crate::schema::data::SubscriberBoundaryVisibility::VisibleSemanticallyIgnorable
            }
            SchemaSubscriberImpact::ContractUpgradeRequired => {
                crate::schema::data::SubscriberBoundaryVisibility::VisibleRequiresContractUptake
            }
            _ => crate::schema::data::SubscriberBoundaryVisibility::NotVisible,
        })],
    }
}

fn verified_at_digest_parity_value() -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::object([
        (
            "parity",
            RelationalDiagnosticValue::string("VerifiedAtLayer"),
        ),
        (
            "verification_layer",
            RelationalDiagnosticValue::string("DigestParity"),
        ),
    ])
}
