use crate::capability::{
    ComponentDescriptor, ComponentHitTestContract, ComponentStaticPaintContract,
    ComponentStaticPaintOrder, ThemeTokenId,
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
        Some(self.static_paint_contract()?.theme_token())
    }

    pub(crate) fn static_paint_order(&self) -> Option<ComponentStaticPaintOrder> {
        Some(self.static_paint_contract()?.order())
    }

    pub(crate) fn semantic_text_theme_token_dependency(&self) -> Option<&ThemeTokenId> {
        Some(self.descriptor.semantic_text_contract()?.theme_token())
    }

    pub(crate) fn semantic_text_layer_order(&self) -> Option<u32> {
        Some(
            self.descriptor
                .semantic_text_contract()?
                .layer_semantic_order(),
        )
    }

    pub(crate) fn hit_test_contract(&self) -> Option<ComponentHitTestContract> {
        self.descriptor.hit_test_contract()
    }

    fn static_paint_contract(&self) -> Option<&ComponentStaticPaintContract> {
        self.descriptor.static_paint_contract()
    }

    pub(crate) fn semantic_digest(&self) -> u64 {
        let digest = self.descriptor.theme_token_dependencies().iter().fold(
            fold_text(0x636f_6d70_6f6e_656e, self.descriptor.id().as_str()),
            |digest, token| fold_text(fold(digest, 1), token.as_str()),
        );
        let digest = self
            .static_paint_order()
            .map_or(digest, |order| fold(digest, u64::from(order.rank())));
        let digest = self
            .semantic_text_layer_order()
            .map_or(digest, |order| fold(digest, u64::from(order)));
        self.hit_test_contract().map_or(digest, |contract| {
            fold(
                fold(digest, 0x6869_745f_7465_7374),
                u64::from(contract.order().rank()),
            )
        })
    }

    #[cfg(test)]
    pub(crate) fn with_static_paint_order_for_test(rank: u32) -> Self {
        let descriptor = ComponentDescriptor::new(
            crate::capability::ComponentId::new("workspace.component.paint")
                .expect("valid component id"),
            crate::capability::ComponentPropSchema::named("workspace.component.paint.props"),
            crate::capability::ComponentChildPolicy::no_children(),
            crate::capability::ComponentStateOwnership::runtime_owned(),
        )
        .with_static_paint(
            ComponentStaticPaintContract::opaque_fill(
                crate::capability::ThemeTokenId::new("theme.paint.fill").expect("valid token id"),
                ComponentStaticPaintOrder::back_to_front(rank),
            ),
            crate::capability::ComponentAllocationMeasurementContract::fill_viewport(),
        );
        Self::new(descriptor, None)
    }
}

#[cfg(test)]
mod tests {
    use crate::capability::{
        ComponentAllocationMeasurementContract, ComponentChildPolicy, ComponentDescriptor,
        ComponentId, ComponentPropSchema, ComponentStateOwnership, ComponentStaticPaintContract,
        ComponentStaticPaintOrder, ComponentViewportInset, ThemeTokenId,
    };

    use super::WorthUiComponentPlanMeaning;

    #[test]
    fn static_paint_requires_an_explicit_complete_contract() {
        let token = ThemeTokenId::new("theme.pulse.fill").expect("valid token id");
        let generic = meaning(component().with_theme_token_dependency(token.clone()));
        let inferred = meaning(
            component()
                .with_theme_token_dependency(token.clone())
                .with_allocation_measurement_contract(
                    ComponentAllocationMeasurementContract::fill_viewport(),
                ),
        );
        let viewport = meaning(component().with_static_paint(
            ComponentStaticPaintContract::opaque_fill(
                token.clone(),
                ComponentStaticPaintOrder::back_to_front(0),
            ),
            ComponentAllocationMeasurementContract::fill_viewport(),
        ));
        let inset = meaning(component().with_static_paint(
            ComponentStaticPaintContract::opaque_fill(
                token.clone(),
                ComponentStaticPaintOrder::back_to_front(1),
            ),
            ComponentAllocationMeasurementContract::viewport_inset(
                ComponentViewportInset::symmetric(48, 24),
            ),
        ));

        assert_eq!(generic.static_paint_theme_token_dependency(), None);
        assert_eq!(inferred.static_paint_theme_token_dependency(), None);
        assert_eq!(viewport.static_paint_theme_token_dependency(), Some(&token));
        assert_eq!(inset.static_paint_theme_token_dependency(), Some(&token));
        assert_eq!(
            viewport.static_paint_order(),
            Some(ComponentStaticPaintOrder::back_to_front(0))
        );
        assert_eq!(
            inset.static_paint_order(),
            Some(ComponentStaticPaintOrder::back_to_front(1))
        );
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
