/// Target continuity retained by one admitted execution after the input frame
/// itself is no longer current.
///
/// Construction requires mounting-owned current-incarnation evidence. Later
/// checks may therefore cross frame-local node-receipt churn without allowing
/// coordinates, raw identities, or an unrelated current target to retarget the
/// execution.
#[derive(Clone)]
pub(crate) struct UiIntentExecutionTargetAffinity {
    mounted: crate::mounting::UiMountedIncarnationAffinityInput,
    graph_node: crate::graph::UiGraphNodeIdentity,
}

pub(crate) fn admit_presented_intent_execution_affinity(
    origin: super::UiPresentedInteractionTargetView,
    mounted: &crate::mounting::WorthUiMountedSessionState,
) -> Result<UiIntentExecutionTargetAffinity, super::UiInteractionTargetingDenial> {
    seal(origin, super::admit_current_target(mounted, origin)?)
}

pub(crate) fn admit_continued_intent_execution_affinity(
    origin: super::UiPresentedInteractionTargetView,
    mounted: &crate::mounting::WorthUiMountedSessionState,
) -> Result<UiIntentExecutionTargetAffinity, super::UiInteractionTargetingDenial> {
    seal(
        origin,
        super::admit_current_target_incarnation(mounted, origin)?,
    )
}

fn seal(
    origin: super::UiPresentedInteractionTargetView,
    admitted: crate::mounting::UiCurrentInteractionAffinity,
) -> Result<UiIntentExecutionTargetAffinity, super::UiInteractionTargetingDenial> {
    Ok(UiIntentExecutionTargetAffinity {
        mounted: crate::mounting::UiMountedIncarnationAffinityInput {
            surface: origin.surface(),
            binding: origin.binding(),
            mounted_instance: origin.mounted_instance(),
        },
        graph_node: admitted.graph_node(),
    })
}

impl UiIntentExecutionTargetAffinity {
    pub(crate) const fn graph_node(&self) -> crate::graph::UiGraphNodeIdentity {
        self.graph_node
    }

    pub(crate) fn require_current(
        &self,
        mounted: &crate::mounting::WorthUiMountedSessionState,
    ) -> Result<(), super::UiInteractionTargetingDenial> {
        let current = mounted
            .admit_current_mounted_incarnation_affinity(self.mounted)
            .map_err(super::presented_frame::map_current_affinity_denial)?;
        if current.graph_node() != self.graph_node {
            return Err(super::UiInteractionTargetingDenial::MountedInstanceNoLongerCurrent);
        }
        Ok(())
    }
}
