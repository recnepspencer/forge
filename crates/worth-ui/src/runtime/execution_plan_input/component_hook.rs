use crate::runtime::WorthUiPlanNodeInputFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiComponentLoweringHookFamily {
    plan_node_family: WorthUiPlanNodeInputFamily,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiComponentLoweringHook {
    hook_id: String,
    admission: WorthUiComponentLoweringHookAdmission,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub(crate) enum WorthUiComponentLoweringHookAdmission {
    Registered(WorthUiComponentLoweringHookFamily),
    Unregistered(String),
}

impl WorthUiComponentLoweringHookFamily {
    #[allow(dead_code)]
    pub(crate) fn admitted(plan_node_family: WorthUiPlanNodeInputFamily) -> Self {
        Self { plan_node_family }
    }

    pub fn plan_node_family(&self) -> WorthUiPlanNodeInputFamily {
        self.plan_node_family
    }
}

impl WorthUiComponentLoweringHook {
    #[allow(dead_code)]
    pub(crate) fn registered(
        hook_id: impl Into<String>,
        family: WorthUiPlanNodeInputFamily,
    ) -> Self {
        Self {
            hook_id: hook_id.into(),
            admission: WorthUiComponentLoweringHookAdmission::Registered(
                WorthUiComponentLoweringHookFamily::admitted(family),
            ),
        }
    }

    #[cfg(test)]
    pub(crate) fn unregistered_for_test(
        hook_id: impl Into<String>,
        family: impl Into<String>,
    ) -> Self {
        Self {
            hook_id: hook_id.into(),
            admission: WorthUiComponentLoweringHookAdmission::Unregistered(family.into()),
        }
    }

    pub fn hook_id(&self) -> &str {
        &self.hook_id
    }

    pub fn admitted_family(&self) -> Option<&WorthUiComponentLoweringHookFamily> {
        match &self.admission {
            WorthUiComponentLoweringHookAdmission::Registered(family) => Some(family),
            WorthUiComponentLoweringHookAdmission::Unregistered(_) => None,
        }
    }

    pub(crate) fn admission(&self) -> &WorthUiComponentLoweringHookAdmission {
        &self.admission
    }
}
