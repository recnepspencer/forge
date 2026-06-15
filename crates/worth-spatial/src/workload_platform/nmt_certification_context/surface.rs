use topology::facade::NmtTopologyScopeReceipt;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::{
    NmtCertificationDenial, NmtCertificationDenialInput, NmtCertificationDenialKind,
    NmtScopeAttackCounters,
};
use crate::workload_platform::surface_support::{CertifiedSurfaceSupport, SurfaceFamily};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NmtScopeSurfaceCounters {
    scope_face_carriers: usize,
    scope_edge_carriers: usize,
    scope_loop_carriers: usize,
    parent_carriers_read: usize,
}

impl NmtScopeSurfaceCounters {
    fn new(
        scope_face_carriers: usize,
        scope_edge_carriers: usize,
        scope_loop_carriers: usize,
        parent_carriers_read: usize,
    ) -> Self {
        Self {
            scope_face_carriers,
            scope_edge_carriers,
            scope_loop_carriers,
            parent_carriers_read,
        }
    }

    pub fn scope_face_carriers(self) -> usize {
        self.scope_face_carriers
    }

    pub fn scope_edge_carriers(self) -> usize {
        self.scope_edge_carriers
    }

    pub fn scope_loop_carriers(self) -> usize {
        self.scope_loop_carriers
    }

    pub fn parent_carriers_read(self) -> usize {
        self.parent_carriers_read
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NmtScopeSurfaceSupportReceipt {
    parent_surface_support_identity: String,
    scope_identity: String,
    scope_surface_support_identity: String,
    surface_family: SurfaceFamily,
    consumed_geometry_carriers: Vec<String>,
    counters: NmtScopeSurfaceCounters,
}

impl NmtScopeSurfaceSupportReceipt {
    pub(crate) fn from_certified_surface_scope(
        support: &CertifiedSurfaceSupport,
        scope: &NmtTopologyScopeReceipt,
        surface_family: SurfaceFamily,
    ) -> Result<Self, NmtCertificationDenial> {
        let snapshot = support.geometry_snapshot();
        let parent_surface_support_identity =
            support.receipts().stage_identity().receipt_identity();
        let faces = snapshot
            .face_rows()
            .iter()
            .filter(|row| scope.contains_topology_entity(row.topology_entity_identity()))
            .map(|row| row.geometry_carrier_identity().to_string())
            .collect::<Vec<_>>();
        let edges = snapshot
            .edge_rows()
            .iter()
            .filter(|row| scope.contains_topology_entity(row.topology_entity_identity()))
            .map(|row| row.geometry_carrier_identity().to_string())
            .collect::<Vec<_>>();
        let loops = snapshot
            .loop_rows()
            .iter()
            .filter(|row| scope.contains_topology_entity(row.topology_entity_identity()))
            .map(|row| row.geometry_carrier_identity().to_string())
            .collect::<Vec<_>>();
        if faces.len() != scope.face_identities().len()
            || edges.len() != scope.edge_identities().len()
            || loops.len() != scope.loop_identities().len()
        {
            return Err(NmtCertificationDenial::new(NmtCertificationDenialInput {
                kind: NmtCertificationDenialKind::MissingScopeGeometry,
                target_scope_identity: Some(scope.scope_identity().to_string()),
                source_scope_identity: None,
                target_scope_kind: Some(scope.kind()),
                consumed_evidence_digest: parent_surface_support_identity,
                human_reason: format!(
                    "{} surface support did not consume every bound geometry carrier in the topology scope.",
                    scope.kind().human_name()
                ),
                counters: NmtScopeAttackCounters::new(
                    1,
                    scope.counters().scope_entity_count(),
                    0,
                    0,
                    0,
                    1,
                ),
            }));
        }
        let mut consumed_geometry_carriers = faces;
        consumed_geometry_carriers.extend(edges);
        consumed_geometry_carriers.extend(loops);
        let parent_carriers_read = support.receipts().counters().upstream_geometry_carriers();
        let counters = NmtScopeSurfaceCounters::new(
            scope.face_identities().len(),
            scope.edge_identities().len(),
            scope.loop_identities().len(),
            parent_carriers_read,
        );
        let scope_surface_support_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "nmt-scope-surface-support".to_string(),
                parent_surface_support_identity.clone(),
                scope.scope_identity().to_string(),
                format!("family:{surface_family:?}"),
                consumed_geometry_carriers.join("|"),
            ],
        );
        Ok(Self {
            parent_surface_support_identity,
            scope_identity: scope.scope_identity().to_string(),
            scope_surface_support_identity,
            surface_family,
            consumed_geometry_carriers,
            counters,
        })
    }

    pub fn parent_surface_support_identity(&self) -> &str {
        &self.parent_surface_support_identity
    }

    pub fn scope_identity(&self) -> &str {
        &self.scope_identity
    }

    pub fn scope_surface_support_identity(&self) -> &str {
        &self.scope_surface_support_identity
    }

    pub fn surface_family(&self) -> SurfaceFamily {
        self.surface_family
    }

    pub fn consumed_geometry_carriers(&self) -> &[String] {
        &self.consumed_geometry_carriers
    }

    pub fn counters(&self) -> NmtScopeSurfaceCounters {
        self.counters
    }
}
