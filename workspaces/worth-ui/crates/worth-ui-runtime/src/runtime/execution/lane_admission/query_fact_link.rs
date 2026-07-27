use crate::runtime::{
    WorthUiPlanNodeInput, WorthUiQueryBindingIdentity, WorthUiQuerySettledFactLink,
};
#[cfg(any(test, feature = "certification-support"))]
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
enum WorthUiQueryLaneFactLinkScope {
    PlanAdmission,
    #[cfg(any(test, feature = "certification-support"))]
    ActiveApplication {
        generation: Rc<
            crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
        >,
        witness: crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationWitness,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryLaneFactLink {
    plan_index: u32,
    binding_identity: WorthUiQueryBindingIdentity,
    settled_fact_link: WorthUiQuerySettledFactLink,
    scope: WorthUiQueryLaneFactLinkScope,
}

impl WorthUiQueryLaneFactLink {
    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn from_active_plan(
        plan_index: u32,
        binding_identity: WorthUiQueryBindingIdentity,
        settled_fact_link: WorthUiQuerySettledFactLink,
        generation: &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
        witness: crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationWitness,
    ) -> Self {
        Self {
            plan_index,
            binding_identity,
            settled_fact_link,
            scope: WorthUiQueryLaneFactLinkScope::ActiveApplication {
                generation: Rc::new(generation.clone()),
                witness,
            },
        }
    }

    pub(crate) fn from_plan_node_input(
        plan_index: u32,
        node_input: &WorthUiPlanNodeInput,
    ) -> Option<Self> {
        Some(Self {
            plan_index,
            binding_identity: node_input.query_binding_identity()?.clone(),
            settled_fact_link: node_input.query_settled_fact_link()?.clone(),
            scope: WorthUiQueryLaneFactLinkScope::PlanAdmission,
        })
    }

    pub fn plan_index(&self) -> u32 {
        self.plan_index
    }

    pub fn binding_identity(&self) -> &WorthUiQueryBindingIdentity {
        &self.binding_identity
    }

    pub fn view_binding_id(&self) -> &str {
        self.binding_identity.view_binding_id()
    }

    pub fn settled_fact_link(&self) -> &WorthUiQuerySettledFactLink {
        &self.settled_fact_link
    }

    pub fn generation_identity(
        &self,
    ) -> Option<&crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity>
    {
        match &self.scope {
            WorthUiQueryLaneFactLinkScope::PlanAdmission => None,
            #[cfg(any(test, feature = "certification-support"))]
            WorthUiQueryLaneFactLinkScope::ActiveApplication { generation, .. } => Some(generation),
        }
    }

    pub(crate) fn belongs_to_generation(
        &self,
        witness: &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationWitness,
    ) -> bool {
        #[cfg(any(test, feature = "certification-support"))]
        {
            matches!(
                &self.scope,
                WorthUiQueryLaneFactLinkScope::ActiveApplication { witness: owned, .. }
                    if owned == witness
            )
        }
        #[cfg(not(any(test, feature = "certification-support")))]
        {
            let _ = witness;
            false
        }
    }
}
