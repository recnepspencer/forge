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
pub(crate) enum WorthUiComponentLoweringHookAdmission {
    #[cfg(test)]
    Registered(WorthUiComponentLoweringHookFamily),
    #[cfg(test)]
    Unregistered,
}

impl WorthUiComponentLoweringHookFamily {
    #[cfg(test)]
    pub(crate) fn admitted(plan_node_family: WorthUiPlanNodeInputFamily) -> Self {
        Self { plan_node_family }
    }

    pub fn plan_node_family(&self) -> WorthUiPlanNodeInputFamily {
        self.plan_node_family
    }
}

impl WorthUiComponentLoweringHook {
    #[cfg(test)]
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
        let _ = family.into();
        Self {
            hook_id: hook_id.into(),
            admission: WorthUiComponentLoweringHookAdmission::Unregistered,
        }
    }

    pub fn hook_id(&self) -> &str {
        &self.hook_id
    }

    pub fn admitted_family(&self) -> Option<&WorthUiComponentLoweringHookFamily> {
        #[cfg(test)]
        match &self.admission {
            WorthUiComponentLoweringHookAdmission::Registered(family) => Some(family),
            #[cfg(test)]
            WorthUiComponentLoweringHookAdmission::Unregistered => None,
        }

        #[cfg(not(test))]
        {
            None
        }
    }

    #[cfg(test)]
    pub(crate) fn admission(&self) -> &WorthUiComponentLoweringHookAdmission {
        &self.admission
    }
}
