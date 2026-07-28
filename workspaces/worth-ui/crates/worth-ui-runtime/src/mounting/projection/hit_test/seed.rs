#[derive(Clone, Copy)]
pub(in crate::mounting::projection) struct UiMountedHitTestSeed {
    order: worth_ui_host_contract::UiMountedHitTestOrder,
}

pub(in crate::mounting::projection) fn lower_hit_test_seed(
    plan: super::super::super::UiMountedPlanProjectionSource<'_>,
    plan_index: Option<u32>,
) -> Result<Option<UiMountedHitTestSeed>, super::super::UiMountedProjectionDenial> {
    let Some(plan_index) = plan_index else {
        return Ok(None);
    };
    let Some(meaning) = plan.ordinary_meaning(plan_index) else {
        return Ok(None);
    };
    let crate::runtime::planning::execution_plan_input::WorthUiPlanOrdinaryMeaning::Component(
        component,
    ) = meaning.as_ref()
    else {
        return Ok(None);
    };
    Ok(component
        .hit_test_contract()
        .map(|contract| UiMountedHitTestSeed {
            order: worth_ui_host_contract::UiMountedHitTestOrder::from_runtime_plan(
                contract.order().rank(),
            ),
        }))
}

impl UiMountedHitTestSeed {
    pub(in crate::mounting::projection) const fn order(
        self,
    ) -> worth_ui_host_contract::UiMountedHitTestOrder {
        self.order
    }
}
