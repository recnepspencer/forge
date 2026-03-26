use serde::{Deserialize, Serialize};
use serde_json::{json, to_value, Value};

use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use crate::identity::data::KindId;
use crate::merge::data::{AspectMergePolicyDeclaration, IdentityBasisDeclaration};
use crate::publication::patch::data::AspectKey;

use super::{
    AspectBinding, AspectComparator, AspectPlanRevision, AspectPrecision, KindAspectDeclarations,
    LoweredAspectComparator, LoweredAspectExtractor, LoweredAspectPlan,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct AspectDeclarationTrace {
    pub kind_id: KindId,
    pub plan_revision: AspectPlanRevision,
    pub declarations: Vec<AspectDeclarationTraceRow>,
    pub identity_declarations: Vec<IdentityBasisDeclaration>,
    pub merge_policy_declarations: Vec<AspectMergePolicyDeclaration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectDeclarationTraceRow {
    pub aspect_key: AspectKey,
    pub binding: AspectBinding,
    pub comparator: AspectComparator,
    pub precision: AspectPrecision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct AspectLoweringTrace {
    pub kind_id: KindId,
    pub plan_revision: AspectPlanRevision,
    pub bindings: Vec<AspectLoweringTraceRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectLoweringTraceRow {
    pub aspect_key: AspectKey,
    pub extractor: LoweredAspectExtractor,
    pub comparator: LoweredAspectComparator,
    pub precision: AspectPrecision,
}

impl KindAspectDeclarations {
    pub fn declaration_trace(&self, kind_id: KindId) -> AspectDeclarationTrace {
        AspectDeclarationTrace {
            kind_id,
            plan_revision: self.plan_revision,
            declarations: self
                .aspects
                .iter()
                .map(|aspect| AspectDeclarationTraceRow {
                    aspect_key: aspect.key.clone(),
                    binding: aspect.binding.clone(),
                    comparator: aspect.comparator,
                    precision: aspect.precision,
                })
                .collect(),
            identity_declarations: self.identity_declarations.clone(),
            merge_policy_declarations: self.merge_policy_declarations.clone(),
        }
    }
}

impl LoweredAspectPlan {
    pub fn lowering_trace(&self) -> AspectLoweringTrace {
        AspectLoweringTrace {
            kind_id: self.kind_id,
            plan_revision: self.plan_revision,
            bindings: self
                .executable_bindings
                .iter()
                .map(|binding| AspectLoweringTraceRow {
                    aspect_key: binding.aspect_key.clone(),
                    extractor: binding.extractor(),
                    comparator: binding.comparator(),
                    precision: binding.precision,
                })
                .collect(),
        }
    }
}

impl AspectDeclarationTrace {
    pub fn diagnostic_artifact(&self) -> RelationalDiagnosticArtifact {
        let fields = AspectDeclarationTraceFields::from_trace(self);
        RelationalDiagnosticArtifact {
            scope: DiagnosticsScope::Schema,
            kind: DiagnosticsArtifactKind::DetailedTrace,
            determinism: DeterminismExpectation::Required,
            entries: vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::AspectDeclarationTraced,
                message: "aspect declaration trace derived from canonical schema declarations"
                    .to_string(),
                fields: trace_fields_value(&fields, "aspect declaration trace"),
            }],
        }
    }
}

impl AspectLoweringTrace {
    pub fn diagnostic_artifact(&self) -> RelationalDiagnosticArtifact {
        let fields = AspectLoweringTraceFields::from_trace(self);
        RelationalDiagnosticArtifact {
            scope: DiagnosticsScope::Schema,
            kind: DiagnosticsArtifactKind::DetailedTrace,
            determinism: DeterminismExpectation::Required,
            entries: vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::AspectLoweringTraced,
                message: "aspect lowering trace derived from canonical lowered aspect plan"
                    .to_string(),
                fields: trace_fields_value(&fields, "aspect lowering trace"),
            }],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct AspectDeclarationTraceFields {
    kind_id: u64,
    plan_revision: String,
    declarations: Vec<AspectDeclarationTraceRow>,
    identity_declarations: Vec<IdentityBasisDeclaration>,
    merge_policy_declarations: Vec<AspectMergePolicyDeclaration>,
}

impl AspectDeclarationTraceFields {
    fn from_trace(trace: &AspectDeclarationTrace) -> Self {
        Self {
            kind_id: trace.kind_id.0 as u64,
            plan_revision: trace.plan_revision.0.to_string(),
            declarations: trace.declarations.clone(),
            identity_declarations: trace.identity_declarations.clone(),
            merge_policy_declarations: trace.merge_policy_declarations.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct AspectLoweringTraceFields {
    kind_id: u64,
    plan_revision: String,
    bindings: Vec<AspectLoweringTraceRow>,
}

impl AspectLoweringTraceFields {
    fn from_trace(trace: &AspectLoweringTrace) -> Self {
        Self {
            kind_id: trace.kind_id.0 as u64,
            plan_revision: trace.plan_revision.0.to_string(),
            bindings: trace.bindings.clone(),
        }
    }
}

fn trace_fields_value<T>(fields: &T, trace_kind: &str) -> Value
where
    T: Serialize,
{
    match to_value(fields) {
        Ok(value) => value,
        Err(error) => json!({
            "trace_kind": trace_kind,
            "serialization_failure": error.to_string(),
        }),
    }
}
