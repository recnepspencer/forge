#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyWorkloadFamily {
    SeededTopology,
    PrimitiveCorpus,
    OperatorTopology,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyWorkloadSupport {
    Admitted,
    Unsupported,
    Blocked,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyWorkloadSupportPosture {
    family: TopologyWorkloadFamily,
    support: TopologyWorkloadSupport,
    reason: String,
}

impl TopologyWorkloadSupportPosture {
    pub(crate) fn admitted(family: TopologyWorkloadFamily) -> Self {
        Self::new(
            family,
            TopologyWorkloadSupport::Admitted,
            "topology workload declaration is admitted",
        )
    }

    pub fn unsupported(family: TopologyWorkloadFamily, reason: impl Into<String>) -> Self {
        Self::new(family, TopologyWorkloadSupport::Unsupported, reason)
    }

    pub fn blocked(family: TopologyWorkloadFamily, reason: impl Into<String>) -> Self {
        Self::new(family, TopologyWorkloadSupport::Blocked, reason)
    }

    fn new(
        family: TopologyWorkloadFamily,
        support: TopologyWorkloadSupport,
        reason: impl Into<String>,
    ) -> Self {
        let reason = normalize_reason(reason);
        Self {
            family,
            support,
            reason,
        }
    }

    pub fn family(&self) -> TopologyWorkloadFamily {
        self.family
    }

    pub fn support(&self) -> TopologyWorkloadSupport {
        self.support
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

fn normalize_reason(reason: impl Into<String>) -> String {
    let reason = reason.into();
    if reason.trim().is_empty() {
        "topology workload support posture requires a human-readable reason".to_string()
    } else {
        reason
    }
}
