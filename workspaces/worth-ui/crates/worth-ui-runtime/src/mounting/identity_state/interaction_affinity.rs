use worth_ui_host_contract::{
    UiMountedHitTestMechanic, UiMountedInstanceIdentity, UiMountedNodeReceiptIdentity,
    UiSemanticSurfaceIdentity, UiSurfaceBindingGeneration,
};

use super::UiMountedIdentityState;

/// Why a historically presented hit row can no longer name a live target.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiCurrentHitTargetAffinityDenial {
    SurfaceNoLongerBound,
    BindingNoLongerCurrent,
    MountedInstanceNoLongerCurrent,
    MountedSurfaceAffinityChanged,
}

/// Mounting-owned admission that one exact presented row still names the same
/// live mounted incarnation. The row cannot be substituted after admission.
pub(crate) struct UiCurrentHitTarget {
    row: UiMountedHitTestMechanic,
}

pub(crate) struct UiCurrentInteractionAffinity {
    graph_node: crate::graph::UiGraphNodeIdentity,
}

#[derive(Clone, Copy)]
pub(crate) struct UiMountedInteractionAffinityInput {
    pub(crate) surface: UiSemanticSurfaceIdentity,
    pub(crate) binding: UiSurfaceBindingGeneration,
    pub(crate) mounted_instance: UiMountedInstanceIdentity,
    pub(crate) node_receipt: UiMountedNodeReceiptIdentity,
}

impl UiMountedIdentityState {
    pub(crate) fn admit_current_hit_target(
        &self,
        row: UiMountedHitTestMechanic,
    ) -> Result<UiCurrentHitTarget, UiCurrentHitTargetAffinityDenial> {
        let binding = self
            .bindings
            .get(&row.surface())
            .ok_or(UiCurrentHitTargetAffinityDenial::SurfaceNoLongerBound)?;
        if binding.view.binding_generation() != row.binding() {
            return Err(UiCurrentHitTargetAffinityDenial::BindingNoLongerCurrent);
        }
        let instance = self
            .instances
            .get(&row.mounted_instance())
            .ok_or(UiCurrentHitTargetAffinityDenial::MountedInstanceNoLongerCurrent)?;
        if instance.basis.semantic_surface_identity() != row.surface() {
            return Err(UiCurrentHitTargetAffinityDenial::MountedSurfaceAffinityChanged);
        }
        Ok(UiCurrentHitTarget { row })
    }

    pub(crate) fn admit_current_interaction_affinity(
        &self,
        input: UiMountedInteractionAffinityInput,
    ) -> Result<UiCurrentInteractionAffinity, UiCurrentHitTargetAffinityDenial> {
        let binding = self
            .bindings
            .get(&input.surface)
            .ok_or(UiCurrentHitTargetAffinityDenial::SurfaceNoLongerBound)?;
        if binding.view.binding_generation() != input.binding {
            return Err(UiCurrentHitTargetAffinityDenial::BindingNoLongerCurrent);
        }
        let instance = self
            .instances
            .get(&input.mounted_instance)
            .ok_or(UiCurrentHitTargetAffinityDenial::MountedInstanceNoLongerCurrent)?;
        if instance.basis.semantic_surface_identity() != input.surface {
            return Err(UiCurrentHitTargetAffinityDenial::MountedSurfaceAffinityChanged);
        }
        let receipt_is_current = self
            .current_receipt_basis
            .as_ref()
            .and_then(|basis| basis.receipt_for(input.mounted_instance))
            .is_some_and(|receipt| receipt == input.node_receipt);
        if !receipt_is_current {
            return Err(UiCurrentHitTargetAffinityDenial::MountedInstanceNoLongerCurrent);
        }
        Ok(UiCurrentInteractionAffinity {
            graph_node: instance.basis.graph_node_identity(),
        })
    }
}

impl UiCurrentHitTarget {
    pub(crate) const fn row(&self) -> UiMountedHitTestMechanic {
        self.row
    }
}

impl UiCurrentInteractionAffinity {
    pub(crate) const fn graph_node(&self) -> crate::graph::UiGraphNodeIdentity {
        self.graph_node
    }
}
