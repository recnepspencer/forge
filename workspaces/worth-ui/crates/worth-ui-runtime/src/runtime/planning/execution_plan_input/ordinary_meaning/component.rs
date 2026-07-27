use crate::capability::{
    ComponentAllocationMeasurementContract, ComponentDescriptor, ThemeTokenId,
};

use super::digest::{fold, fold_text};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiComponentPlanMeaning {
    descriptor: ComponentDescriptor,
    child_range_identity: Option<String>,
}

impl WorthUiComponentPlanMeaning {
    pub(crate) fn new(
        descriptor: ComponentDescriptor,
        child_range_identity: Option<String>,
    ) -> Self {
        Self {
            descriptor,
            child_range_identity,
        }
    }

    pub(crate) fn child_range_identity(&self) -> Option<&str> {
        self.child_range_identity.as_deref()
    }

    pub(crate) fn descriptor(&self) -> &ComponentDescriptor {
        &self.descriptor
    }

    pub(crate) fn static_paint_theme_token_dependency(&self) -> Option<&ThemeTokenId> {
        if self.descriptor.allocation_measurement_contract()
            != Some(ComponentAllocationMeasurementContract::fill_viewport())
        {
            return None;
        }
        let [token_id] = self.descriptor.theme_token_dependencies() else {
            return None;
        };
        Some(token_id)
    }

    pub(crate) fn semantic_digest(&self) -> u64 {
        self.descriptor.theme_token_dependencies().iter().fold(
            fold_text(0x636f_6d70_6f6e_656e, self.descriptor.id().as_str()),
            |digest, token| fold_text(fold(digest, 1), token.as_str()),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::capability::{
        ComponentAllocationMeasurementContract, ComponentChildPolicy, ComponentDescriptor,
        ComponentId, ComponentPropSchema, ComponentStateOwnership, ThemeTokenId,
    };

    use super::WorthUiComponentPlanMeaning;

    #[test]
    fn static_paint_requires_fill_viewport_and_one_theme_token() {
        let token = ThemeTokenId::new("theme.pulse.fill").expect("valid token id");
        let generic = meaning(component().with_theme_token_dependency(token.clone()));
        let pulse = meaning(
            component()
                .with_theme_token_dependency(token.clone())
                .with_allocation_measurement_contract(
                    ComponentAllocationMeasurementContract::fill_viewport(),
                ),
        );

        assert_eq!(generic.static_paint_theme_token_dependency(), None);
        assert_eq!(pulse.static_paint_theme_token_dependency(), Some(&token));
    }

    fn component() -> ComponentDescriptor {
        ComponentDescriptor::new(
            ComponentId::new("workspace.component.pulse").expect("valid component id"),
            ComponentPropSchema::named("workspace.component.pulse.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        )
    }

    fn meaning(descriptor: ComponentDescriptor) -> WorthUiComponentPlanMeaning {
        WorthUiComponentPlanMeaning::new(descriptor, None)
    }
}
