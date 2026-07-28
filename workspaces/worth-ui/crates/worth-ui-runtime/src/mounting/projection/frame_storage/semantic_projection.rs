use worth_ui_host_contract::{
    UiMountedProjectionAudience, UiSemanticSurfaceIdentity, UiSurfaceBindingGeneration,
};

use super::super::UiMountedNodeReceipt;

#[derive(Clone)]
pub(in crate::mounting::projection) struct UiMountedProjectionNodeRecord {
    pub(in crate::mounting::projection) receipt: UiMountedNodeReceipt,
    pub(in crate::mounting::projection) plan_index: Option<u32>,
    pub(in crate::mounting::projection) static_paint:
        Option<super::super::static_paint::UiMountedStaticPaintSeed>,
    pub(in crate::mounting::projection) hit_test:
        Option<super::super::hit_test::UiMountedHitTestSeed>,
}

#[derive(Clone, Copy)]
pub(in crate::mounting::projection) struct UiMountedProjectionSurface {
    pub(in crate::mounting::projection) surface: UiSemanticSurfaceIdentity,
    pub(in crate::mounting::projection) binding: UiSurfaceBindingGeneration,
    pub(in crate::mounting::projection) audience: UiMountedProjectionAudience,
}

#[derive(Clone)]
pub(in crate::mounting) struct UiMountedSemanticProjection {
    pub(super) nodes: crate::runtime::persistent_index::UiPersistentOrdMap<
        worth_ui_host_contract::UiMountedInstanceIdentity,
        UiMountedProjectionNodeRecord,
    >,
    pub(super) order: std::rc::Rc<[worth_ui_host_contract::UiMountedInstanceIdentity]>,
    membership: crate::runtime::persistent_index::UiPersistentOrdSet<
        worth_ui_host_contract::UiMountedInstanceIdentity,
    >,
    semantic_surfaces:
        crate::runtime::persistent_index::UiPersistentOrdSet<UiSemanticSurfaceIdentity>,
    binding_by_surface: crate::runtime::persistent_index::UiPersistentOrdMap<
        UiSemanticSurfaceIdentity,
        UiSurfaceBindingGeneration,
    >,
    pub(super) surfaces: crate::runtime::persistent_index::UiPersistentOrdMap<
        UiSurfaceBindingGeneration,
        UiMountedProjectionSurface,
    >,
}

impl UiMountedSemanticProjection {
    pub(in crate::mounting::projection) fn initial(
        nodes: Vec<UiMountedProjectionNodeRecord>,
        surfaces: Vec<UiMountedProjectionSurface>,
    ) -> Self {
        let order = nodes
            .iter()
            .map(|record| record.receipt.mounted_instance())
            .collect::<Vec<_>>();
        let mut node_index = crate::runtime::persistent_index::UiPersistentOrdMap::default();
        let mut membership = crate::runtime::persistent_index::UiPersistentOrdSet::default();
        for node in nodes {
            let instance = node.receipt.mounted_instance();
            node_index.insert(instance, node);
            membership.insert(instance);
        }
        let mut surface_index = crate::runtime::persistent_index::UiPersistentOrdMap::default();
        let mut binding_by_surface =
            crate::runtime::persistent_index::UiPersistentOrdMap::default();
        let mut semantic_surfaces = crate::runtime::persistent_index::UiPersistentOrdSet::default();
        for surface in surfaces {
            semantic_surfaces.insert(surface.surface);
            binding_by_surface.insert(surface.surface, surface.binding);
            surface_index.insert(surface.binding, surface);
        }
        Self {
            nodes: node_index,
            order: order.into(),
            membership,
            semantic_surfaces,
            binding_by_surface,
            surfaces: surface_index,
        }
    }

    pub(in crate::mounting::projection) fn membership(
        &self,
    ) -> crate::runtime::persistent_index::UiPersistentOrdSet<
        worth_ui_host_contract::UiMountedInstanceIdentity,
    > {
        self.membership.clone()
    }

    pub(in crate::mounting::projection) fn supports_surfaces(
        &self,
        surfaces: &[UiSemanticSurfaceIdentity],
    ) -> bool {
        surfaces.len() == self.semantic_surfaces.len()
            && surfaces
                .iter()
                .all(|surface| self.semantic_surfaces.contains_with_probes(surface).0)
    }

    pub(in crate::mounting::projection) fn contains(
        &self,
        instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    ) -> bool {
        self.membership.contains_with_probes(&instance).0
    }

    pub(in crate::mounting::projection) fn insert_node(
        &mut self,
        node: UiMountedProjectionNodeRecord,
    ) -> crate::runtime::persistent_index::UiPersistentIndexMutationWork {
        let instance = node.receipt.mounted_instance();
        self.membership.insert(instance);
        self.nodes.insert_with_work(instance, node)
    }

    pub(in crate::mounting::projection) fn remove_node(
        &mut self,
        instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    ) -> crate::runtime::persistent_index::UiPersistentIndexMutationWork {
        self.membership.remove_with_work(&instance);
        self.nodes.remove_with_work(&instance).1
    }

    pub(in crate::mounting::projection) fn replace_order(
        &mut self,
        order: Vec<worth_ui_host_contract::UiMountedInstanceIdentity>,
    ) {
        self.order = order.into();
    }

    pub(in crate::mounting::projection) fn replace_surface(
        &mut self,
        surface: UiMountedProjectionSurface,
    ) -> crate::runtime::persistent_index::UiPersistentIndexMutationWork {
        if let Some(previous) = self.binding_by_surface.get(&surface.surface).copied() {
            self.surfaces.remove(&previous);
        }
        self.semantic_surfaces.insert(surface.surface);
        self.binding_by_surface
            .insert(surface.surface, surface.binding);
        self.surfaces.insert_with_work(surface.binding, surface)
    }

    pub(in crate::mounting::projection) fn remove_surface(
        &mut self,
        surface: UiSemanticSurfaceIdentity,
    ) -> crate::runtime::persistent_index::UiPersistentIndexMutationWork {
        self.semantic_surfaces.remove_with_work(&surface);
        let binding = self.binding_by_surface.get(&surface).copied();
        self.binding_by_surface.remove(&surface);
        binding.map_or_else(Default::default, |binding| {
            self.surfaces.remove_with_work(&binding).1
        })
    }

    pub(in crate::mounting::projection) fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub(in crate::mounting::projection) fn nodes_in_order(
        &self,
    ) -> impl Iterator<Item = &UiMountedProjectionNodeRecord> {
        self.order.iter().map(|instance| {
            self.nodes
                .get(instance)
                .expect("mounted semantic order names an indexed node")
        })
    }

    pub(in crate::mounting) fn node_receipt_with_probes(
        &self,
        mounted_instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    ) -> (Option<&UiMountedNodeReceipt>, usize) {
        let (record, probes) = self.nodes.get_with_probes(&mounted_instance);
        (record.map(|record| &record.receipt), probes)
    }

    pub(in crate::mounting) fn retained_structural_bytes(&self) -> Option<usize> {
        std::mem::size_of::<Self>()
            .checked_add(self.nodes.retained_structural_bytes()?)?
            .checked_add(self.order.len().checked_mul(std::mem::size_of::<
                worth_ui_host_contract::UiMountedInstanceIdentity,
            >())?)?
            .checked_add(self.membership.retained_structural_bytes()?)?
            .checked_add(self.semantic_surfaces.retained_structural_bytes()?)?
            .checked_add(self.binding_by_surface.retained_structural_bytes()?)?
            .checked_add(self.surfaces.retained_structural_bytes()?)
    }

    pub(in crate::mounting::projection) fn surface_instance_count(
        &self,
        surfaces: &[UiSemanticSurfaceIdentity],
    ) -> usize {
        self.order
            .iter()
            .filter(|instance| {
                self.nodes
                    .get(instance)
                    .is_some_and(|node| surfaces.contains(&node.receipt.semantic_surface()))
            })
            .count()
    }

    pub(in crate::mounting::projection) fn surface_for(
        &self,
        surface: UiSemanticSurfaceIdentity,
    ) -> Option<UiMountedProjectionSurface> {
        self.binding_by_surface
            .get(&surface)
            .and_then(|binding| self.surfaces.get(binding))
            .copied()
    }
}
