use super::{
    NmtTopologyScopeCounters, NmtTopologyScopeDenial, NmtTopologyScopeKind,
    NmtTopologyScopeReceipt, NmtTopologyScopeReceiptInput,
};
use crate::workload_platform::nmt_topology_construction::{
    NmtTopologyConstructionReceipt, NmtTopologyPattern, NmtTopologyPosture,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NmtTopologyScopeSet {
    parent_construction_identity: String,
    scopes: Vec<NmtTopologyScopeReceipt>,
}

impl NmtTopologyScopeSet {
    pub fn from_construction(
        construction: &NmtTopologyConstructionReceipt,
    ) -> Result<Self, NmtTopologyScopeDenial> {
        let parent_construction_identity = construction
            .pattern_identity()
            .identity_digest()
            .to_string();
        let scopes = scopes_from_construction(construction)?;
        if scopes.is_empty() {
            return Err(NmtTopologyScopeDenial::missing(
                "NMT topology construction did not expose any certifiable topology scopes.",
            ));
        }
        Ok(Self {
            parent_construction_identity,
            scopes,
        })
    }

    pub fn parent_construction_identity(&self) -> &str {
        &self.parent_construction_identity
    }

    pub fn scopes(&self) -> &[NmtTopologyScopeReceipt] {
        &self.scopes
    }

    pub fn single_scope(
        &self,
        kind: NmtTopologyScopeKind,
    ) -> Result<&NmtTopologyScopeReceipt, NmtTopologyScopeDenial> {
        let mut matches = self
            .scopes
            .iter()
            .filter(|scope| scope.kind() == kind)
            .collect::<Vec<_>>();
        if matches.len() == 1 {
            Ok(matches.remove(0))
        } else {
            Err(NmtTopologyScopeDenial::MissingScopeKind { kind })
        }
    }

    pub fn layer(&self, layer_index: usize) -> Option<&NmtTopologyScopeReceipt> {
        self.scopes
            .iter()
            .find(|scope| scope.layer_index() == Some(layer_index))
    }
}

fn scopes_from_construction(
    construction: &NmtTopologyConstructionReceipt,
) -> Result<Vec<NmtTopologyScopeReceipt>, NmtTopologyScopeDenial> {
    match construction.pattern() {
        NmtTopologyPattern::OpenWireChain(_) => Ok(vec![single_scope(
            construction,
            NmtTopologyScopeKind::OpenWire,
            None,
        )]),
        NmtTopologyPattern::OpenSheetPatch(_) => Ok(vec![single_scope(
            construction,
            NmtTopologyScopeKind::OpenSheet,
            None,
        )]),
        NmtTopologyPattern::OpenRadialFan(_) => Ok(vec![single_scope(
            construction,
            NmtTopologyScopeKind::OpenRadialFan,
            None,
        )]),
        NmtTopologyPattern::OpenLayerStack(_) => layer_scopes(construction),
    }
}

fn single_scope(
    construction: &NmtTopologyConstructionReceipt,
    kind: NmtTopologyScopeKind,
    layer_index: Option<usize>,
) -> NmtTopologyScopeReceipt {
    let identities = construction.topology_seed_receipt().entity_identities();
    build_scope(
        construction,
        kind,
        layer_index,
        identities.face_identity_tokens(),
        identities.edge_identity_tokens(),
        identities.loop_identity_tokens(),
        construction.counters().boundary_half_edge_count(),
        construction.counters().non_manifold_edge_count(),
    )
}

fn layer_scopes(
    construction: &NmtTopologyConstructionReceipt,
) -> Result<Vec<NmtTopologyScopeReceipt>, NmtTopologyScopeDenial> {
    let layer_count = construction.counters().layer_count();
    if layer_count == 0 {
        return Err(NmtTopologyScopeDenial::unsupported(
            "Open layer topology construction cannot produce scopes with zero layers.",
        ));
    }
    let identities = construction.topology_seed_receipt().entity_identities();
    let faces = identities.face_identity_tokens();
    let edges = identities.edge_identity_tokens();
    let loops = identities.loop_identity_tokens();
    if faces.len() % layer_count != 0
        || edges.len() % layer_count != 0
        || loops.len() % layer_count != 0
    {
        return Err(NmtTopologyScopeDenial::unsupported(
            "Open layer topology construction entity identities are not evenly partitioned by layer.",
        ));
    }
    let faces_per_layer = faces.len() / layer_count;
    let edges_per_layer = edges.len() / layer_count;
    let loops_per_layer = loops.len() / layer_count;
    let boundary_per_layer = construction
        .counters()
        .boundary_half_edge_count()
        .checked_div(layer_count)
        .unwrap_or(0);
    let radial_per_layer = construction
        .counters()
        .non_manifold_edge_count()
        .checked_div(layer_count)
        .unwrap_or(0);
    Ok((0..layer_count)
        .map(|layer| {
            build_scope(
                construction,
                NmtTopologyScopeKind::OpenLayer,
                Some(layer),
                slice(&faces, layer, faces_per_layer),
                slice(&edges, layer, edges_per_layer),
                slice(&loops, layer, loops_per_layer),
                boundary_per_layer,
                radial_per_layer,
            )
        })
        .collect())
}

fn slice(values: &[String], index: usize, width: usize) -> Vec<String> {
    values[index * width..(index + 1) * width].to_vec()
}

fn build_scope(
    construction: &NmtTopologyConstructionReceipt,
    kind: NmtTopologyScopeKind,
    layer_index: Option<usize>,
    face_identities: Vec<String>,
    edge_identities: Vec<String>,
    loop_identities: Vec<String>,
    boundary_half_edge_count: usize,
    non_manifold_edge_count: usize,
) -> NmtTopologyScopeReceipt {
    let counters = NmtTopologyScopeCounters::new(
        face_identities.len(),
        edge_identities.len(),
        loop_identities.len(),
        boundary_half_edge_count,
        non_manifold_edge_count,
    );
    let parent_boundary = construction.open_boundary().boundary_digest();
    let parent_radial = construction.radial_adjacency().radial_digest();
    let scope_anchor = format!("{kind:?}:{layer_index:?}:{}", face_identities.join("|"));
    let open_boundary_identity = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "nmt-scope-open-boundary".to_string(),
            parent_boundary.to_string(),
            scope_anchor.clone(),
        ],
    );
    let radial_adjacency_identity = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "nmt-scope-radial-adjacency".to_string(),
            parent_radial.to_string(),
            scope_anchor,
        ],
    );
    NmtTopologyScopeReceipt::new(NmtTopologyScopeReceiptInput {
        parent_construction_identity: construction
            .pattern_identity()
            .identity_digest()
            .to_string(),
        pattern_identity: construction.pattern_identity().pattern_name().to_string(),
        kind,
        layer_index,
        face_identities,
        edge_identities,
        loop_identities,
        topology_posture: scope_posture(construction, kind),
        open_boundary_identity,
        radial_adjacency_identity,
        counters,
    })
}

fn scope_posture(
    construction: &NmtTopologyConstructionReceipt,
    kind: NmtTopologyScopeKind,
) -> NmtTopologyPosture {
    match kind {
        NmtTopologyScopeKind::OpenLayer => NmtTopologyPosture::OpenSheet,
        _ => construction.topology_posture().posture(),
    }
}
