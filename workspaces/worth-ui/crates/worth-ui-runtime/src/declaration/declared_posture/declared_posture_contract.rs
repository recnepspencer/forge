use super::{
    UiDeclaredHostCapabilityPosture, UiDeclaredMeasurementPolicyPosture,
    UiDeclaredPostureApplicability, UiDeclaredQueryBindingPosture, UiDeclaredServiceUsagePosture,
    UiDeclaredTouchMeaningPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiDeclaredPostureLane<T> {
    applicability: UiDeclaredPostureApplicability,
    admitted: Option<T>,
}

impl<T> UiDeclaredPostureLane<T> {
    pub(crate) const fn new(
        applicability: UiDeclaredPostureApplicability,
        admitted: Option<T>,
    ) -> Self {
        Self {
            applicability,
            admitted,
        }
    }

    pub const fn applicability(&self) -> UiDeclaredPostureApplicability {
        self.applicability
    }

    pub fn admitted(&self) -> Option<&T> {
        self.admitted.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiDeclaredPostureContract {
    query_binding: UiDeclaredPostureLane<UiDeclaredQueryBindingPosture>,
    service_usage: UiDeclaredPostureLane<UiDeclaredServiceUsagePosture>,
    touch_meaning: UiDeclaredPostureLane<UiDeclaredTouchMeaningPosture>,
    measurement_policy: UiDeclaredPostureLane<UiDeclaredMeasurementPolicyPosture>,
    host_capability: UiDeclaredPostureLane<UiDeclaredHostCapabilityPosture>,
}

impl UiDeclaredPostureContract {
    pub(crate) const fn new(
        query_binding: UiDeclaredPostureLane<UiDeclaredQueryBindingPosture>,
        service_usage: UiDeclaredPostureLane<UiDeclaredServiceUsagePosture>,
        touch_meaning: UiDeclaredPostureLane<UiDeclaredTouchMeaningPosture>,
        measurement_policy: UiDeclaredPostureLane<UiDeclaredMeasurementPolicyPosture>,
        host_capability: UiDeclaredPostureLane<UiDeclaredHostCapabilityPosture>,
    ) -> Self {
        Self {
            query_binding,
            service_usage,
            touch_meaning,
            measurement_policy,
            host_capability,
        }
    }

    pub const fn query_binding(&self) -> &UiDeclaredPostureLane<UiDeclaredQueryBindingPosture> {
        &self.query_binding
    }

    pub const fn service_usage(&self) -> &UiDeclaredPostureLane<UiDeclaredServiceUsagePosture> {
        &self.service_usage
    }

    pub const fn touch_meaning(&self) -> &UiDeclaredPostureLane<UiDeclaredTouchMeaningPosture> {
        &self.touch_meaning
    }

    pub const fn measurement_policy(
        &self,
    ) -> &UiDeclaredPostureLane<UiDeclaredMeasurementPolicyPosture> {
        &self.measurement_policy
    }

    pub const fn host_capability(&self) -> &UiDeclaredPostureLane<UiDeclaredHostCapabilityPosture> {
        &self.host_capability
    }
}
