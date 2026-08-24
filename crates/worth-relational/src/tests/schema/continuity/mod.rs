mod commit_admission_denials;
mod commit_artifact_publication;
mod descriptor_identity;
mod publication_bundle;
mod registry_authority_basis;
mod transition_admission;

use std::sync::Arc;

use worth_foundational::facade::CanonicalFieldPath;

use crate::authority::commit::phases::schema_continuity::{
    validate_schema_continuity_publication, SchemaContinuityPlan,
};
use crate::diagnostics::data::{DiagnosticsArtifactKind, RelationalDiagnosticValue};
use crate::facade::history::BranchId;
use crate::facade::identity::KindId;
use crate::facade::schema::{
    DescriptorCanonicalBasisVersion, DescriptorSemanticsVersion, FreeFormSchemaDiffIntent,
    HistoricalInterpretationSensitivity, LoweredSchemaTransitionPlan, ProposedSchemaTransition,
    SchemaBoundaryFingerprint, SchemaBridgeDescriptor, SchemaBridgeabilityClassification,
    SchemaContinuationAdmissionObservation, SchemaContinuationClassification,
    SchemaContinuationDescriptor, SchemaDiffAtom, SchemaDiffDetail, SchemaElementKind,
    SchemaElementRef, SchemaId, SchemaLineageArtifact, SchemaLineageOrderingSemantics,
    SchemaPublicationImpact, SchemaReconciliationClassification, SchemaReconciliationDescriptor,
    SchemaReconciliationOrderingMode, SchemaReconciliationPolicy, SchemaStratum,
    SchemaSubscriberImpact, SchemaTransitionArtifact, SchemaTransitionSummary, SchemaVersionId,
    ValidatedSchemaTransition,
};
use crate::schema::{
    classify_schema_transition, lower_schema_transition, validate_schema_continuity_bundle,
    validate_schema_transition, SchemaContinuityBundleIssue,
};
use crate::tests::support::*;
use crate::transactions::data::ConflictClass;

fn diagnostic_object_field<'a>(
    value: &'a RelationalDiagnosticValue,
    field: &str,
) -> &'a RelationalDiagnosticValue {
    let RelationalDiagnosticValue::Object(fields) = value else {
        panic!("diagnostic value is not an object: {value:?}");
    };
    fields
        .get(field)
        .unwrap_or_else(|| panic!("diagnostic object field '{field}' missing from {value:?}"))
}
