use super::PlatformPulseProductComponent;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformPulseStaticCopy {
    component: PlatformPulseProductComponent,
    text: &'static str,
}

impl PlatformPulseStaticCopy {
    pub const ALL: [Self; 19] = [
        Self::new(PlatformPulseProductComponent::Brand, "W  O  R  T  H"),
        Self::new(
            PlatformPulseProductComponent::RuntimeBadge,
            "●  LIVE     NATIVE PROCESS",
        ),
        Self::new(
            PlatformPulseProductComponent::EvidenceTitle,
            "SOURCE SIGNAL",
        ),
        Self::new(PlatformPulseProductComponent::EvidenceBody, "Continuous"),
        Self::new(
            PlatformPulseProductComponent::SourceSignalTitle,
            "Bound Query · intent · paint",
        ),
        Self::new(
            PlatformPulseProductComponent::ServiceEyebrow,
            "PLATFORM PULSE",
        ),
        Self::new(
            PlatformPulseProductComponent::ServiceTitle,
            "Platform\nPulse",
        ),
        Self::new(
            PlatformPulseProductComponent::ServiceBody,
            "Bound Query, intent admission, and native publication — only as they happen.",
        ),
        Self::new(PlatformPulseProductComponent::QueryLabel, "QUERY POSTURE"),
        Self::new(PlatformPulseProductComponent::NativeLabel, "NATIVE HOST"),
        Self::new(
            PlatformPulseProductComponent::NativeBody,
            "Window · Query · Paint",
        ),
        Self::new(
            PlatformPulseProductComponent::ActionLabel,
            "Run live action",
        ),
        Self::new(PlatformPulseProductComponent::PortalLabel, "Details"),
        Self::new(PlatformPulseProductComponent::PortalIconText, "↗"),
        Self::new(
            PlatformPulseProductComponent::PortalTitle,
            "Run live action",
        ),
        Self::new(
            PlatformPulseProductComponent::PortalBody,
            "Publish one admitted action.\nObserve the resulting Query posture.",
        ),
        Self::new(PlatformPulseProductComponent::PortalCancelLabel, "Cancel"),
        Self::new(
            PlatformPulseProductComponent::PortalPrimaryLabel,
            "Run action",
        ),
        Self::new(
            PlatformPulseProductComponent::StatusText,
            "✓   Native frame published",
        ),
    ];

    const fn new(component: PlatformPulseProductComponent, text: &'static str) -> Self {
        Self { component, text }
    }

    pub const fn component(self) -> PlatformPulseProductComponent {
        self.component
    }

    pub const fn text(self) -> &'static str {
        self.text
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn static_copy_has_one_truthful_owner_per_component() {
        let owners = PlatformPulseStaticCopy::ALL
            .into_iter()
            .map(|copy| copy.component().id())
            .collect::<BTreeSet<_>>();
        assert_eq!(owners.len(), PlatformPulseStaticCopy::ALL.len());
        assert!(PlatformPulseStaticCopy::ALL
            .into_iter()
            .all(|copy| !copy.text().trim().is_empty()));
    }
}
