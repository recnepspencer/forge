mod origin_receipts;

use crate::graph::{
    UiGraphMountedReceiptTransition, UiGraphNodeIdentity, UiGraphSnapshot, UiGraphTopologyRecord,
    UiGraphWorldProfile,
};
use crate::obligations::touch::{
    normalize_aspects, UiGraphTouchAspects, UiGraphTouchDenial, UiGraphTouchDescriptor,
    UiGraphTouchOriginAuthority, UiGraphTouchOriginWitness, UiGraphTouchTarget, UiGraphTouchTiming,
    UiGraphTouchWorld,
};

#[derive(Clone, Copy)]
pub struct UiGraphTouchAuthority<'a> {
    snapshot: &'a UiGraphSnapshot,
}

impl<'a> UiGraphTouchAuthority<'a> {
    pub(crate) const fn new(snapshot: &'a UiGraphSnapshot) -> Self {
        Self { snapshot }
    }

    pub fn from_node(
        self,
        origin: UiGraphTouchOriginWitness,
        timing: UiGraphTouchTiming,
        graph_node_identity: UiGraphNodeIdentity,
        aspects: UiGraphTouchAspects,
    ) -> Result<UiGraphTouchDescriptor, UiGraphTouchDenial> {
        self.require_origin_graph_node(&origin, graph_node_identity, false)?;
        self.require_node(graph_node_identity)?;
        self.descriptor_from_target(
            UiGraphTouchTarget::node(graph_node_identity),
            origin,
            timing,
            aspects,
        )
    }

    pub fn from_slot_occupancy(
        self,
        origin: UiGraphTouchOriginWitness,
        timing: UiGraphTouchTiming,
        graph_node_identity: UiGraphNodeIdentity,
        aspects: UiGraphTouchAspects,
    ) -> Result<UiGraphTouchDescriptor, UiGraphTouchDenial> {
        self.require_origin_graph_node(&origin, graph_node_identity, false)?;
        let topology = self.require_topology(graph_node_identity)?;
        let Some(parent_node_identity) = topology.parent_node_identity() else {
            return Err(UiGraphTouchDenial::SlotOccupancyUnavailable {
                graph_node_identity,
            });
        };
        let Some(slot_topology) = topology.slot_topology() else {
            return Err(UiGraphTouchDenial::SlotOccupancyUnavailable {
                graph_node_identity,
            });
        };

        self.descriptor_from_target(
            UiGraphTouchTarget::slot_occupancy(
                graph_node_identity,
                parent_node_identity,
                slot_topology.slot_name().into(),
            ),
            origin,
            timing,
            aspects,
        )
    }

    pub fn from_page_membership(
        self,
        origin: UiGraphTouchOriginWitness,
        timing: UiGraphTouchTiming,
        graph_node_identity: UiGraphNodeIdentity,
        aspects: UiGraphTouchAspects,
    ) -> Result<UiGraphTouchDescriptor, UiGraphTouchDenial> {
        self.require_origin_graph_node(&origin, graph_node_identity, false)?;
        let topology = self.require_topology(graph_node_identity)?;
        let Some(page_membership) = topology.page_membership() else {
            return Err(UiGraphTouchDenial::PageMembershipUnavailable {
                graph_node_identity,
            });
        };

        self.descriptor_from_target(
            UiGraphTouchTarget::page_membership(
                graph_node_identity,
                page_membership.page_node_identity(),
            ),
            origin,
            timing,
            aspects,
        )
    }

    pub fn from_region_membership(
        self,
        origin: UiGraphTouchOriginWitness,
        timing: UiGraphTouchTiming,
        graph_node_identity: UiGraphNodeIdentity,
        aspects: UiGraphTouchAspects,
    ) -> Result<UiGraphTouchDescriptor, UiGraphTouchDenial> {
        self.require_origin_graph_node(&origin, graph_node_identity, false)?;
        let topology = self.require_topology(graph_node_identity)?;
        let Some(region_membership) = topology.region_membership() else {
            return Err(UiGraphTouchDenial::RegionMembershipUnavailable {
                graph_node_identity,
            });
        };

        self.descriptor_from_target(
            UiGraphTouchTarget::region_membership(
                graph_node_identity,
                region_membership.region_name().into(),
            ),
            origin,
            timing,
            aspects,
        )
    }

    pub fn from_mosaic_membership(
        self,
        origin: UiGraphTouchOriginWitness,
        timing: UiGraphTouchTiming,
        graph_node_identity: UiGraphNodeIdentity,
        aspects: UiGraphTouchAspects,
    ) -> Result<UiGraphTouchDescriptor, UiGraphTouchDenial> {
        self.require_origin_graph_node(&origin, graph_node_identity, false)?;
        let topology = self.require_topology(graph_node_identity)?;
        let Some(mosaic_membership) = topology.mosaic_membership() else {
            return Err(UiGraphTouchDenial::MosaicMembershipUnavailable {
                graph_node_identity,
            });
        };

        self.descriptor_from_target(
            UiGraphTouchTarget::mosaic_membership(
                graph_node_identity,
                mosaic_membership.mosaic_name().into(),
            ),
            origin,
            timing,
            aspects,
        )
    }

    pub fn from_mounted_receipt_transition(
        self,
        origin: UiGraphTouchOriginWitness,
        timing: UiGraphTouchTiming,
        transition: UiGraphMountedReceiptTransition,
        aspects: UiGraphTouchAspects,
    ) -> Result<UiGraphTouchDescriptor, UiGraphTouchDenial> {
        self.require_origin_graph_node(
            &origin,
            transition.authority_record().graph_node_identity(),
            true,
        )?;
        self.descriptor_from_target(
            UiGraphTouchTarget::mounted_receipt_slot(
                transition.authority_record().graph_node_identity(),
                transition.authority_record().mounted_receipt_identity(),
            ),
            origin,
            timing,
            aspects,
        )
    }

    fn descriptor_from_target(
        self,
        target: UiGraphTouchTarget,
        origin: UiGraphTouchOriginWitness,
        timing: UiGraphTouchTiming,
        aspects: UiGraphTouchAspects,
    ) -> Result<UiGraphTouchDescriptor, UiGraphTouchDenial> {
        let normalized_aspects = normalize_aspects(&aspects)?;

        Ok(UiGraphTouchDescriptor::new(
            target,
            origin.into(),
            UiGraphTouchWorld::from_profile(self.snapshot.world_profile()),
            timing,
            normalized_aspects,
        ))
    }

    fn require_node(
        self,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Result<(), UiGraphTouchDenial> {
        if self
            .snapshot
            .lookup()
            .graph_node(graph_node_identity)
            .is_none()
        {
            return Err(UiGraphTouchDenial::UnknownGraphNode {
                graph_node_identity,
            });
        }
        Ok(())
    }

    fn require_topology(
        self,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Result<UiGraphTopologyRecord, UiGraphTouchDenial> {
        self.snapshot
            .lookup()
            .topology_node(graph_node_identity)
            .map(|row| row.value().clone())
            .ok_or(UiGraphTouchDenial::UnknownGraphNode {
                graph_node_identity,
            })
    }

    fn require_origin_graph_node(
        self,
        origin: &UiGraphTouchOriginWitness,
        graph_node_identity: UiGraphNodeIdentity,
        allow_mounted_receipt_transition_only: bool,
    ) -> Result<(), UiGraphTouchDenial> {
        match origin.authority() {
            UiGraphTouchOriginAuthority::DeclarationInstances {
                declaration_identity,
            } => {
                let declaration_instances = self
                    .snapshot
                    .lookup()
                    .declaration_instances(declaration_identity)
                    .value();
                if declaration_instances.contains(&graph_node_identity) {
                    Ok(())
                } else {
                    Err(UiGraphTouchDenial::OriginDoesNotAuthorizeGraphNode {
                        origin_class: origin.receipt().class(),
                        graph_node_identity,
                    })
                }
            }
            UiGraphTouchOriginAuthority::QueryBasis { prerequisites } => {
                let canonical = prerequisites.canonical_basis_digest();
                let expected = canonical
                    .value()
                    .bytes()
                    .iter()
                    .take(8)
                    .enumerate()
                    .fold(0u64, |digest, (index, byte)| {
                        digest | (u64::from(*byte) << (index * 8))
                    });
                if allow_mounted_receipt_transition_only
                    && origin.receipt().authority_digest() == expected
                {
                    Ok(())
                } else {
                    Err(UiGraphTouchDenial::OriginDoesNotAuthorizeGraphNode {
                        origin_class: origin.receipt().class(),
                        graph_node_identity,
                    })
                }
            }
            UiGraphTouchOriginAuthority::InstalledQueryBasis { authority } => {
                let aligned = matches!(
                    self.snapshot.world_profile(),
                    UiGraphWorldProfile::InstalledQueryBasis { authority: current }
                        if current.shares_authority_with(authority)
                );
                if allow_mounted_receipt_transition_only
                    && aligned
                    && origin.receipt().authority_digest() == authority.identity().as_u64()
                {
                    Ok(())
                } else {
                    Err(UiGraphTouchDenial::OriginDoesNotAuthorizeGraphNode {
                        origin_class: origin.receipt().class(),
                        graph_node_identity,
                    })
                }
            }
            UiGraphTouchOriginAuthority::AuthoredProvenanceDigests { digests } => self
                .snapshot
                .core_indexes()
                .declaration_correspondence()
                .authored_provenance_digest_for(graph_node_identity)
                .filter(|digest| digests.binary_search(digest).is_ok())
                .map(|_| ())
                .ok_or(UiGraphTouchDenial::OriginDoesNotAuthorizeGraphNode {
                    origin_class: origin.receipt().class(),
                    graph_node_identity,
                }),
        }
    }
}
