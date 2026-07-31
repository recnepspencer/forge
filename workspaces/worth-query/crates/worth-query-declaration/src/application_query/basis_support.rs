#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryBasisSupport {
    current: bool,
    pinned: bool,
    preview: bool,
}

impl ApplicationQueryBasisSupport {
    pub const fn current_and_pinned() -> Self {
        Self {
            current: true,
            pinned: true,
            preview: false,
        }
    }

    pub const fn with_preview(mut self) -> Self {
        self.preview = true;
        self
    }

    pub const fn current(self) -> bool {
        self.current
    }

    pub const fn pinned(self) -> bool {
        self.pinned
    }

    pub const fn preview(self) -> bool {
        self.preview
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationQueryDisclosurePosture {
    Public,
    InstalledPolicyRequired,
    PhaseSevenGovernanceRequired,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryDisclosureContract {
    posture: ApplicationQueryDisclosurePosture,
    classification: &'static str,
}

impl ApplicationQueryDisclosureContract {
    pub const fn public() -> Self {
        Self {
            posture: ApplicationQueryDisclosurePosture::Public,
            classification: "public",
        }
    }

    pub const fn installed_policy(classification: &'static str) -> Self {
        Self {
            posture: ApplicationQueryDisclosurePosture::InstalledPolicyRequired,
            classification,
        }
    }

    pub const fn phase_seven_required(classification: &'static str) -> Self {
        Self {
            posture: ApplicationQueryDisclosurePosture::PhaseSevenGovernanceRequired,
            classification,
        }
    }

    pub const fn posture(&self) -> ApplicationQueryDisclosurePosture {
        self.posture
    }

    pub const fn classification(&self) -> &'static str {
        self.classification
    }
}
