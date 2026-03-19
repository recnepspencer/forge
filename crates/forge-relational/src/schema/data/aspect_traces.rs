use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use crate::identity::data::KindId;
use crate::publication::patch::data::AspectKey;

use super::{
    AspectBinding, AspectComparator, AspectPlanRevision, AspectPrecision, KindAspectDeclarations,
    LoweredAspectComparator, LoweredAspectExtractor, LoweredAspectPlan,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectDeclarationTrace {
    pub kind_id: KindId,
    pub plan_revision: AspectPlanRevision,
    pub declarations: Vec<AspectDeclarationTraceRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectDeclarationTraceRow {
    pub aspect_key: AspectKey,
    pub binding: AspectBinding,
    pub comparator: AspectComparator,
    pub precision: AspectPrecision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
                    extractor: binding.extractor.clone(),
                    comparator: binding.comparator,
                    precision: binding.precision,
                })
                .collect(),
        }
    }
}

impl AspectDeclarationTrace {
    pub fn diagnostic_artifact(&self) -> RelationalDiagnosticArtifact {
        RelationalDiagnosticArtifact {
            scope: DiagnosticsScope::Schema,
            kind: DiagnosticsArtifactKind::DetailedTrace,
            determinism: DeterminismExpectation::Required,
            entries: vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::AspectDeclarationTraced,
                message: "aspect declaration trace derived from canonical schema declarations"
                    .to_string(),
                fields: json!({
                    "kind_id": self.kind_id.0,
                    "plan_revision": self.plan_revision.0.to_string(),
                    "declarations": self.declarations,
                }),
            }],
        }
    }
}

impl AspectLoweringTrace {
    pub fn diagnostic_artifact(&self) -> RelationalDiagnosticArtifact {
        RelationalDiagnosticArtifact {
            scope: DiagnosticsScope::Schema,
            kind: DiagnosticsArtifactKind::DetailedTrace,
            determinism: DeterminismExpectation::Required,
            entries: vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::AspectLoweringTraced,
                message: "aspect lowering trace derived from canonical lowered aspect plan"
                    .to_string(),
                fields: json!({
                    "kind_id": self.kind_id.0,
                    "plan_revision": self.plan_revision.0.to_string(),
                    "bindings": self.bindings,
                }),
            }],
        }
    }
}
