#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct UiCommandPrefixOccupancy {
    first: super::input_stroke::UiCommandInputStroke,
    revision: u64,
    application: crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity,
    context: super::UiCommandRoutingContext,
    started_at_millis: u64,
}

pub(super) enum UiCommandPrefixCurrentness {
    Current,
    ContextChanged,
    Expired,
    BasisUnavailable,
}

impl UiCommandPrefixOccupancy {
    pub(super) fn new(
        first: super::input_stroke::UiCommandInputStroke,
        revision: u64,
        application: crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity,
        context: super::UiCommandRoutingContext,
        started_at_millis: u64,
    ) -> Self {
        Self {
            first,
            revision,
            application,
            context,
            started_at_millis,
        }
    }

    pub(super) const fn first(&self) -> super::input_stroke::UiCommandInputStroke {
        self.first
    }

    pub(super) fn currentness(
        &self,
        application: &crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity,
        context: &super::UiCommandRoutingContext,
        maximum_wait_millis: u16,
    ) -> UiCommandPrefixCurrentness {
        if &self.application != application || !self.context.same_prefix_affinity(context) {
            return UiCommandPrefixCurrentness::ContextChanged;
        }
        let Some(now) = monotonic_millis(context.time_basis()) else {
            return UiCommandPrefixCurrentness::BasisUnavailable;
        };
        if now < self.started_at_millis
            || now.saturating_sub(self.started_at_millis) > u64::from(maximum_wait_millis)
        {
            UiCommandPrefixCurrentness::Expired
        } else {
            UiCommandPrefixCurrentness::Current
        }
    }
}

pub(super) const fn monotonic_millis(
    basis: Option<worth_ui_host_contract::UiHostObservationTimeBasis>,
) -> Option<u64> {
    match basis {
        Some(worth_ui_host_contract::UiHostObservationTimeBasis::HostMonotonicMillis(value)) => {
            Some(value)
        }
        _ => None,
    }
}
