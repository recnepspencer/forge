use crate::declaration::UiAspectFamily;
use crate::fact_contract::UiProducedFactFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiSubsystemConsumedFactRule {
    fact_family: UiProducedFactFamily,
    affected_aspect_family: UiAspectFamily,
}

impl UiSubsystemConsumedFactRule {
    const HOST_VIEWPORT: [Self; 1] = [Self::new(
        UiProducedFactFamily::HostViewport,
        UiAspectFamily::Layout,
    )];
    const HOST_DEVICE_SCALE: [Self; 1] = [Self::new(
        UiProducedFactFamily::HostDeviceScale,
        UiAspectFamily::Appearance,
    )];
    const MEASUREMENT: [Self; 1] = [Self::new(
        UiProducedFactFamily::Measurement,
        UiAspectFamily::Layout,
    )];
    const QUERY: [Self; 2] = [
        Self::new(UiProducedFactFamily::Query, UiAspectFamily::Content),
        Self::new(UiProducedFactFamily::Query, UiAspectFamily::Structure),
    ];
    const COMMITTED_SCROLL_EXTENT: [Self; 1] = [Self::new(
        UiProducedFactFamily::CommittedScrollExtent,
        UiAspectFamily::Layout,
    )];
    const COMMITTED_PORTAL_ANCHOR: [Self; 2] = [
        Self::new(
            UiProducedFactFamily::CommittedPortalAnchor,
            UiAspectFamily::Layout,
        ),
        Self::new(
            UiProducedFactFamily::CommittedPortalAnchor,
            UiAspectFamily::Presence,
        ),
    ];

    pub fn all() -> impl Iterator<Item = Self> {
        Self::HOST_VIEWPORT
            .into_iter()
            .chain(Self::HOST_DEVICE_SCALE)
            .chain(Self::MEASUREMENT)
            .chain(Self::QUERY)
            .chain(Self::COMMITTED_SCROLL_EXTENT)
            .chain(Self::COMMITTED_PORTAL_ANCHOR)
    }

    pub fn for_fact_family(family: UiProducedFactFamily) -> &'static [Self] {
        match family {
            UiProducedFactFamily::AuthoredSource => &[],
            UiProducedFactFamily::HostViewport => &Self::HOST_VIEWPORT,
            UiProducedFactFamily::HostDeviceScale => &Self::HOST_DEVICE_SCALE,
            UiProducedFactFamily::Measurement => &Self::MEASUREMENT,
            UiProducedFactFamily::Query => &Self::QUERY,
            UiProducedFactFamily::IntentPosture => &[],
            UiProducedFactFamily::CommittedScrollExtent => &Self::COMMITTED_SCROLL_EXTENT,
            UiProducedFactFamily::CommittedPortalAnchor => &Self::COMMITTED_PORTAL_ANCHOR,
        }
    }

    const fn new(
        fact_family: UiProducedFactFamily,
        affected_aspect_family: UiAspectFamily,
    ) -> Self {
        Self {
            fact_family,
            affected_aspect_family,
        }
    }

    pub const fn fact_family(self) -> UiProducedFactFamily {
        self.fact_family
    }

    pub const fn affected_aspect_family(self) -> UiAspectFamily {
        self.affected_aspect_family
    }
}
