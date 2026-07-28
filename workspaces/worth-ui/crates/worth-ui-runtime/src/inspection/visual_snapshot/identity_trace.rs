#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiVisualIdentityTraceDenial {
    ReceiptNotRetained,
    GraphNodeNotIndexed,
    DeclarationArtifactMissing,
    DeclarationNotIndexed,
    ProvenanceNotIndexed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiVisualIdentityTraceCost {
    index_probes: usize,
}

pub(crate) struct UiResolvedVisualIdentityTrace {
    trace: worth_ui_inspection::UiVisualIdentityTrace,
    cost: UiVisualIdentityTraceCost,
}

pub(crate) fn resolve_identity_trace(
    basis: &crate::mounting::UiMountedIdentityTraceBasis,
    node_receipt: worth_ui_host_contract::UiMountedNodeReceiptIdentity,
) -> Result<UiResolvedVisualIdentityTrace, UiVisualIdentityTraceDenial> {
    let (mounted, mounted_probes) = basis.node_for_receipt_with_probes(node_receipt);
    let mounted = mounted.ok_or(UiVisualIdentityTraceDenial::ReceiptNotRetained)?;
    let source = basis.authored_source();
    let graph_lookup = source
        .graph_node_evidence_index()
        .lookup_graph_node_identity(mounted.graph_node())
        .ok_or(UiVisualIdentityTraceDenial::GraphNodeNotIndexed)?;
    let artifact = source
        .declaration_artifacts()
        .get(graph_lookup.neighborhood().declaration_artifact_index())
        .ok_or(UiVisualIdentityTraceDenial::DeclarationArtifactMissing)?;
    let declaration = artifact.identity().inspection_identity();
    let provenance = artifact
        .provenance()
        .inspection_authored_source_provenance_ref();
    let declaration_lookup = source
        .authored_evidence_index()
        .lookup_declaration_identity(declaration)
        .ok_or(UiVisualIdentityTraceDenial::DeclarationNotIndexed)?;
    let provenance_lookup = source
        .authored_evidence_index()
        .lookup_authored_provenance(&provenance)
        .ok_or(UiVisualIdentityTraceDenial::ProvenanceNotIndexed)?;
    let evidence = projected_evidence(
        graph_lookup.neighborhood().refs(),
        declaration_lookup.neighborhood().refs(),
        provenance_lookup.neighborhood().refs(),
    );
    let trace = worth_ui_inspection::UiVisualIdentityTrace::from_runtime_projection(
        worth_ui_inspection::UiVisualIdentityTraceInput {
            node_receipt: node_receipt.diagnostic_value(),
            mounted_instance: mounted.mounted_instance().diagnostic_value(),
            incarnation: mounted.incarnation().diagnostic_value(),
            graph_node: mounted.graph_node().digest(),
            declaration: declaration.digest(),
            authored_semantic_name: artifact.identity().authored_semantic_name().into(),
            source_artifact: provenance.source_artifact().clone(),
            source_generation: provenance.source_generation(),
            declaration_index: provenance.declaration_index(),
            evidence,
        },
    );
    Ok(UiResolvedVisualIdentityTrace {
        trace,
        cost: UiVisualIdentityTraceCost {
            index_probes: mounted_probes
                .saturating_add(graph_lookup.cost().index_lookups())
                .saturating_add(1)
                .saturating_add(declaration_lookup.cost().index_lookups())
                .saturating_add(provenance_lookup.cost().index_lookups()),
        },
    })
}

fn projected_evidence(
    graph: &[crate::evidence::UiEvidenceRef],
    declaration: &[crate::evidence::UiEvidenceRef],
    provenance: &[crate::evidence::UiEvidenceRef],
) -> Vec<worth_ui_inspection::UiVisualEvidenceRef> {
    let ordered = crate::evidence::order_refs(
        graph
            .iter()
            .chain(declaration)
            .chain(provenance)
            .copied()
            .collect(),
    );
    let mut projected = ordered
        .into_vec()
        .into_iter()
        .map(|evidence| {
            worth_ui_inspection::UiVisualEvidenceRef::from_runtime_projection(
                evidence.family(),
                evidence.authority_generation().as_u64(),
                evidence.identity().digest(),
                evidence.handle().handle_digest(),
            )
        })
        .collect::<Vec<_>>();
    projected.dedup();
    projected
}

impl UiResolvedVisualIdentityTrace {
    pub(crate) fn into_parts(
        self,
    ) -> (
        worth_ui_inspection::UiVisualIdentityTrace,
        UiVisualIdentityTraceCost,
    ) {
        (self.trace, self.cost)
    }
}

impl UiVisualIdentityTraceCost {
    pub(crate) const fn index_probes(self) -> usize {
        self.index_probes
    }
}
