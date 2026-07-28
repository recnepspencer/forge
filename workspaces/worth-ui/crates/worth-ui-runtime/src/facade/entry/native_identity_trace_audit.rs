use worth_ui_host_contract::{
    UiMountIncarnation, UiMountedInstanceIdentity, UiMountedNodeReceiptIdentity,
};
use worth_ui_inspection::{UiAuthoredSourceProvenanceRef, UiInspectionDeclarationIdentity};

pub(super) struct RetainedIdentityTraceAudit {
    generation:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    node_receipt: UiMountedNodeReceiptIdentity,
    mounted_instance: UiMountedInstanceIdentity,
    incarnation: UiMountIncarnation,
    graph_node: crate::graph::UiGraphNodeIdentity,
    declaration: UiInspectionDeclarationIdentity,
    authored_provenance: UiAuthoredSourceProvenanceRef,
    evidence: Box<[crate::evidence::UiEvidenceRef]>,
    cost: RetainedIdentityTraceAuditCost,
}

pub(super) struct PreparedIdentityTraceOracle {
    authored_provenance_digest: u64,
    graph_node: crate::graph::UiGraphNodeIdentity,
    declaration: UiInspectionDeclarationIdentity,
    authored_provenance: UiAuthoredSourceProvenanceRef,
    evidence: Box<[crate::evidence::UiEvidenceRef]>,
}

struct AuthoredTraceJoin {
    graph_node: crate::graph::UiGraphNodeIdentity,
    declaration: UiInspectionDeclarationIdentity,
    authored_provenance: UiAuthoredSourceProvenanceRef,
    evidence: Box<[crate::evidence::UiEvidenceRef]>,
    graph_identity_index_lookups: usize,
    declaration_artifact_index_lookups: usize,
    declaration_identity_index_lookups: usize,
    authored_provenance_index_lookups: usize,
}

#[derive(Clone, Copy)]
pub(super) struct RetainedIdentityTraceAuditCost {
    mounted_receipt_index_probes: usize,
    mounted_node_index_probes: usize,
    graph_identity_index_lookups: usize,
    declaration_artifact_index_lookups: usize,
    declaration_identity_index_lookups: usize,
    authored_provenance_index_lookups: usize,
}

pub(super) fn audit_retained_identity_trace(
    basis: &crate::mounting::UiMountedIdentityTraceBasis,
    mounted_instance: UiMountedInstanceIdentity,
    node_receipt: UiMountedNodeReceiptIdentity,
) -> RetainedIdentityTraceAudit {
    let (expected_receipt, mounted_receipt_index_probes) =
        basis.receipt_for_with_probes(mounted_instance);
    assert_eq!(
        expected_receipt,
        Some(node_receipt),
        "retained receipt basis must name the production-minted node receipt"
    );
    let (mounted, mounted_node_index_probes) = basis.node_with_probes(mounted_instance);
    let mounted = mounted.expect("retained semantic projection should index the mounted instance");
    let source = basis.authored_source();
    let authored = join_authored_trace(source, mounted.graph_node());
    RetainedIdentityTraceAudit {
        generation: source.generation().clone(),
        node_receipt,
        mounted_instance,
        incarnation: mounted.incarnation(),
        graph_node: authored.graph_node,
        declaration: authored.declaration,
        authored_provenance: authored.authored_provenance,
        evidence: authored.evidence,
        cost: RetainedIdentityTraceAuditCost {
            mounted_receipt_index_probes,
            mounted_node_index_probes,
            graph_identity_index_lookups: authored.graph_identity_index_lookups,
            declaration_artifact_index_lookups: authored.declaration_artifact_index_lookups,
            declaration_identity_index_lookups: authored.declaration_identity_index_lookups,
            authored_provenance_index_lookups: authored.authored_provenance_index_lookups,
        },
    }
}

fn join_authored_trace(
    source: &crate::facade::prepared_application_authority::WorthUiPreparedVisualTraceSource,
    graph_node: crate::graph::UiGraphNodeIdentity,
) -> AuthoredTraceJoin {
    let graph_lookup = source
        .graph_node_evidence_index()
        .lookup_graph_node_identity(graph_node)
        .expect("exact prepared generation should index the mounted graph node");
    let graph_neighborhood = graph_lookup.neighborhood();
    let artifact = source
        .declaration_artifacts()
        .get(graph_neighborhood.declaration_artifact_index())
        .expect("graph evidence should name an exact declaration artifact");
    let declaration = artifact.identity().inspection_identity();
    let authored_provenance = artifact
        .provenance()
        .inspection_authored_source_provenance_ref();
    let declaration_lookup = source
        .authored_evidence_index()
        .lookup_declaration_identity(declaration)
        .expect("prepared generation should index declaration identity");
    let provenance_lookup = source
        .authored_evidence_index()
        .lookup_authored_provenance(&authored_provenance)
        .expect("prepared generation should index authored provenance");
    assert_eq!(
        graph_neighborhood.declaration_artifact_index(),
        declaration_lookup
            .neighborhood()
            .declaration_artifact_index()
    );
    assert_eq!(
        declaration_lookup
            .neighborhood()
            .declaration_artifact_index(),
        provenance_lookup
            .neighborhood()
            .declaration_artifact_index()
    );
    AuthoredTraceJoin {
        graph_node,
        declaration,
        authored_provenance,
        evidence: trace_evidence(
            graph_neighborhood.refs(),
            declaration_lookup.neighborhood().refs(),
            provenance_lookup.neighborhood().refs(),
        ),
        graph_identity_index_lookups: graph_lookup.cost().index_lookups(),
        declaration_artifact_index_lookups: 1,
        declaration_identity_index_lookups: declaration_lookup.cost().index_lookups(),
        authored_provenance_index_lookups: provenance_lookup.cost().index_lookups(),
    }
}

pub(super) fn prepared_identity_trace_oracles(
    artifacts: &[crate::declaration::UiDeclarationArtifact],
    graph: crate::graph::UiGraphAuthority<'_>,
) -> Vec<PreparedIdentityTraceOracle> {
    let authored = crate::declaration::UiDeclarationAuthoredEvidenceIndex::rebuild(
        artifacts,
        graph.snapshot(),
    );
    let graph_index = crate::graph::UiGraphNodeEvidenceIndex::rebuild(artifacts, graph.snapshot());
    graph
        .node_identities()
        .map(|graph_node| {
            let graph_lookup = graph_index
                .lookup_graph_node_identity(graph_node)
                .expect("prepared graph node should have an evidence neighborhood");
            let artifact = artifacts
                .get(graph_lookup.neighborhood().declaration_artifact_index())
                .expect("prepared graph evidence should name a declaration artifact");
            let declaration = artifact.identity().inspection_identity();
            let authored_provenance = artifact
                .provenance()
                .inspection_authored_source_provenance_ref();
            let declaration_lookup = authored
                .lookup_declaration_identity(declaration)
                .expect("prepared declaration should have an evidence neighborhood");
            let provenance_lookup = authored
                .lookup_authored_provenance(&authored_provenance)
                .expect("prepared provenance should have an evidence neighborhood");
            let source = artifact.provenance().source_provenance();
            PreparedIdentityTraceOracle {
                authored_provenance_digest: crate::declaration::authored_source_provenance_digest(
                    source.module_path(),
                    source.declaration_index(),
                ),
                graph_node,
                declaration,
                authored_provenance,
                evidence: trace_evidence(
                    graph_lookup.neighborhood().refs(),
                    declaration_lookup.neighborhood().refs(),
                    provenance_lookup.neighborhood().refs(),
                ),
            }
        })
        .collect()
}

fn trace_evidence(
    graph: &[crate::evidence::UiEvidenceRef],
    declaration: &[crate::evidence::UiEvidenceRef],
    provenance: &[crate::evidence::UiEvidenceRef],
) -> Box<[crate::evidence::UiEvidenceRef]> {
    let ordered = crate::evidence::order_refs(
        graph
            .iter()
            .chain(declaration)
            .chain(provenance)
            .copied()
            .collect(),
    );
    let mut deduplicated = ordered.into_vec();
    deduplicated.dedup();
    deduplicated.into_boxed_slice()
}

impl RetainedIdentityTraceAudit {
    pub(super) fn generation(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        &self.generation
    }

    pub(super) fn node_receipt(&self) -> UiMountedNodeReceiptIdentity {
        self.node_receipt
    }

    pub(super) fn mounted_instance(&self) -> UiMountedInstanceIdentity {
        self.mounted_instance
    }

    pub(super) fn incarnation(&self) -> UiMountIncarnation {
        self.incarnation
    }

    pub(super) fn graph_node(&self) -> crate::graph::UiGraphNodeIdentity {
        self.graph_node
    }

    pub(super) fn declaration(&self) -> UiInspectionDeclarationIdentity {
        self.declaration
    }

    pub(super) fn authored_provenance(&self) -> &UiAuthoredSourceProvenanceRef {
        &self.authored_provenance
    }

    pub(super) fn evidence(&self) -> &[crate::evidence::UiEvidenceRef] {
        &self.evidence
    }

    pub(super) fn cost(&self) -> RetainedIdentityTraceAuditCost {
        self.cost
    }
}

impl PreparedIdentityTraceOracle {
    pub(super) fn authored_provenance_digest(&self) -> u64 {
        self.authored_provenance_digest
    }

    pub(super) fn graph_node(&self) -> crate::graph::UiGraphNodeIdentity {
        self.graph_node
    }

    pub(super) fn declaration(&self) -> UiInspectionDeclarationIdentity {
        self.declaration
    }

    pub(super) fn authored_provenance(&self) -> &UiAuthoredSourceProvenanceRef {
        &self.authored_provenance
    }

    pub(super) fn evidence(&self) -> &[crate::evidence::UiEvidenceRef] {
        &self.evidence
    }
}

impl RetainedIdentityTraceAuditCost {
    pub(super) fn trace_index_probes(self) -> usize {
        self.mounted_receipt_index_probes
            + self.mounted_node_index_probes
            + self.graph_identity_index_lookups
            + self.declaration_artifact_index_lookups
            + self.declaration_identity_index_lookups
            + self.authored_provenance_index_lookups
    }

    pub(super) fn mounted_receipt_index_probes(self) -> usize {
        self.mounted_receipt_index_probes
    }

    pub(super) fn mounted_node_index_probes(self) -> usize {
        self.mounted_node_index_probes
    }

    pub(super) fn graph_identity_index_lookups(self) -> usize {
        self.graph_identity_index_lookups
    }

    pub(super) fn declaration_artifact_index_lookups(self) -> usize {
        self.declaration_artifact_index_lookups
    }

    pub(super) fn declaration_identity_index_lookups(self) -> usize {
        self.declaration_identity_index_lookups
    }

    pub(super) fn authored_provenance_index_lookups(self) -> usize {
        self.authored_provenance_index_lookups
    }
}
