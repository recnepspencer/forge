use crate::runtime::{
    WorthUiGraphFactRegistry, WorthUiProjectionDependencySet, WorthUiQueryGraphExecutionReceipt,
    WorthUiQueryGraphExecutionRow, WorthUiQueryGraphObligationSemantic, WorthUiRuntimeFactId,
    WorthUiValidatedProjectionDependencyContract,
};

use super::super::primitive_authored_prop_schemas;
use super::{
    WorthUiPrimitiveConstructionObligationKind, WorthUiPrimitiveConstructionObligationPosture,
    WorthUiPrimitiveConstructionObligationRow, WorthUiPrimitiveGraphCounters,
    WorthUiPrimitiveQueryPosture,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveConstructionGraphProof {
    surface_id: String,
    component_id: String,
    dependency_contract: WorthUiValidatedProjectionDependencyContract,
    published_facts: Vec<WorthUiRuntimeFactId>,
    obligation_rows: Vec<WorthUiPrimitiveConstructionObligationRow>,
    query_posture: WorthUiPrimitiveQueryPosture,
    query_graph_execution: WorthUiQueryGraphExecutionReceipt,
    counters: WorthUiPrimitiveGraphCounters,
    graph_proof_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiPrimitiveFamilyAdmissionDigests {
    pub primitive: u64,
    pub flow: u64,
    pub content: u64,
    pub appearance_state: u64,
    pub interaction: u64,
    pub event_geometry: u64,
}

impl WorthUiPrimitiveConstructionGraphProof {
    pub(super) fn prove(
        surface_id: &str,
        component_id: &str,
        dependency_contract: WorthUiValidatedProjectionDependencyContract,
        query_graph_execution: WorthUiQueryGraphExecutionReceipt,
        digests: WorthUiPrimitiveFamilyAdmissionDigests,
    ) -> Self {
        let published_facts = WorthUiGraphFactRegistry::for_primitive_surface(surface_id)
            .published_facts()
            .facts()
            .cloned()
            .collect::<Vec<_>>();
        let query_posture = WorthUiPrimitiveQueryPosture::ProjectionFactsRequired;
        let obligation_rows = query_graph_execution
            .rows()
            .iter()
            .map(primitive_obligation_row)
            .collect::<Vec<_>>();
        let counters = WorthUiPrimitiveGraphCounters::new(
            primitive_authored_prop_schemas().len(),
            6,
            published_facts.len(),
            dependency_contract.dependencies().facts().count(),
            0,
            obligation_rows
                .iter()
                .filter(|row| {
                    row.posture() == WorthUiPrimitiveConstructionObligationPosture::Selected
                })
                .count(),
            obligation_rows
                .iter()
                .filter(|row| {
                    row.posture() == WorthUiPrimitiveConstructionObligationPosture::NotApplicable
                })
                .count(),
        );
        let graph_proof_digest = graph_proof_digest(
            surface_id,
            component_id,
            dependency_contract.dependencies(),
            &published_facts,
            &obligation_rows,
            &query_posture,
            &query_graph_execution,
            digests,
        );
        Self {
            surface_id: surface_id.to_owned(),
            component_id: component_id.to_owned(),
            dependency_contract,
            published_facts,
            obligation_rows,
            query_posture,
            query_graph_execution,
            counters,
            graph_proof_digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn component_id(&self) -> &str {
        &self.component_id
    }

    pub fn dependency_contract(&self) -> &WorthUiValidatedProjectionDependencyContract {
        &self.dependency_contract
    }

    pub fn published_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.published_facts
    }

    pub fn obligation_rows(&self) -> &[WorthUiPrimitiveConstructionObligationRow] {
        &self.obligation_rows
    }

    pub fn query_posture(&self) -> &WorthUiPrimitiveQueryPosture {
        &self.query_posture
    }

    pub fn query_graph_execution(&self) -> &WorthUiQueryGraphExecutionReceipt {
        &self.query_graph_execution
    }

    pub fn counters(&self) -> WorthUiPrimitiveGraphCounters {
        self.counters
    }

    pub fn graph_proof_digest(&self) -> u64 {
        self.graph_proof_digest
    }
}

fn primitive_obligation_row(
    row: &WorthUiQueryGraphExecutionRow,
) -> WorthUiPrimitiveConstructionObligationRow {
    WorthUiPrimitiveConstructionObligationRow::new(
        primitive_obligation_kind(row.semantic()),
        primitive_obligation_posture(row),
        format!(
            "query graph execution selected {} support={} execution={}",
            row.semantic().as_str(),
            row.support_status(),
            row.execution_status()
        ),
    )
}

fn primitive_obligation_kind(
    kind: WorthUiQueryGraphObligationSemantic,
) -> WorthUiPrimitiveConstructionObligationKind {
    match kind {
        WorthUiQueryGraphObligationSemantic::SchemaAdmission => {
            WorthUiPrimitiveConstructionObligationKind::SchemaContract
        }
        WorthUiQueryGraphObligationSemantic::CapabilitySupport => {
            WorthUiPrimitiveConstructionObligationKind::CapabilitySupport
        }
        WorthUiQueryGraphObligationSemantic::OperatingContext => {
            WorthUiPrimitiveConstructionObligationKind::OperatingContext
        }
        WorthUiQueryGraphObligationSemantic::DependencyContract => {
            WorthUiPrimitiveConstructionObligationKind::DependencyContract
        }
        _ => WorthUiPrimitiveConstructionObligationKind::QuerySupport,
    }
}

fn primitive_obligation_posture(
    row: &WorthUiQueryGraphExecutionRow,
) -> WorthUiPrimitiveConstructionObligationPosture {
    match (row.support_status(), row.execution_status()) {
        ("not-applicable", _) | (_, "not-applicable-after-state-load") => {
            WorthUiPrimitiveConstructionObligationPosture::NotApplicable
        }
        ("unsupported", _) | (_, "unsupported") => {
            WorthUiPrimitiveConstructionObligationPosture::Unsupported
        }
        (_, "executor-error")
        | (_, "budget-exceeded")
        | (_, "suppressed-by-policy")
        | (_, "blocked-by-prerequisite") => WorthUiPrimitiveConstructionObligationPosture::Denied,
        _ => WorthUiPrimitiveConstructionObligationPosture::Selected,
    }
}

fn graph_proof_digest(
    surface_id: &str,
    component_id: &str,
    dependencies: &WorthUiProjectionDependencySet,
    published_facts: &[WorthUiRuntimeFactId],
    obligations: &[WorthUiPrimitiveConstructionObligationRow],
    query_posture: &WorthUiPrimitiveQueryPosture,
    query_graph_execution: &WorthUiQueryGraphExecutionReceipt,
    digests: WorthUiPrimitiveFamilyAdmissionDigests,
) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325;
    for value in [surface_id, component_id] {
        digest = fold(digest, value.as_bytes());
    }
    for value in [
        digests.primitive,
        digests.flow,
        digests.content,
        digests.appearance_state,
        digests.interaction,
        digests.event_geometry,
        dependencies.digest().value(),
        query_posture.digest(),
        query_graph_execution.execution_digest(),
    ] {
        digest = fold(digest, &value.to_le_bytes());
    }
    for fact in published_facts {
        digest = fold(digest, fact.family().token().as_bytes());
        digest = fold(digest, fact.identity().as_bytes());
    }
    for row in obligations {
        digest = fold(digest, format!("{:?}", row.kind()).as_bytes());
        digest = fold(digest, format!("{:?}", row.posture()).as_bytes());
        digest = fold(digest, row.evidence().as_bytes());
    }
    digest
}

fn fold(mut digest: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
    }
    digest
}
