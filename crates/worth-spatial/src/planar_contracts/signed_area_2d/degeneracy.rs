#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AreaDegeneracyPolicy {
    ClassifyWithoutRepair,
}

impl AreaDegeneracyPolicy {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClassifyWithoutRepair => "classify-without-repair",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AreaDegeneracyClass {
    WellFormed,
    ZeroArea,
    Sliver,
    Needle,
    TinyHole,
    PolicyRequired,
}

impl AreaDegeneracyClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WellFormed => "well-formed",
            Self::ZeroArea => "zero-area",
            Self::Sliver => "sliver",
            Self::Needle => "needle",
            Self::TinyHole => "tiny-hole",
            Self::PolicyRequired => "policy-required",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignedAreaOrientation {
    CounterClockwise,
    Clockwise,
    Zero,
}

impl SignedAreaOrientation {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CounterClockwise => "counter-clockwise",
            Self::Clockwise => "clockwise",
            Self::Zero => "zero",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignedAreaRepairAction {
    None,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SignedAreaDegeneracyCause {
    AreaSum {
        loop_identity: String,
        frame_identity: String,
        precision_fact_digest: String,
    },
    NeedleEdge {
        loop_identity: String,
        edge_index: usize,
        frame_identity: String,
        precision_fact_digest: String,
    },
    ContainmentPolicyRequired {
        loop_identity: String,
        containment: String,
        policy: String,
    },
}

impl SignedAreaDegeneracyCause {
    pub fn identity(&self) -> String {
        match self {
            Self::AreaSum {
                loop_identity,
                frame_identity,
                precision_fact_digest,
            } => format!("loop:{loop_identity};edge:area-sum;frame:{frame_identity};precision:{precision_fact_digest}"),
            Self::NeedleEdge {
                loop_identity,
                edge_index,
                frame_identity,
                precision_fact_digest,
            } => format!("loop:{loop_identity};edge:{edge_index};frame:{frame_identity};precision:{precision_fact_digest}"),
            Self::ContainmentPolicyRequired {
                loop_identity,
                containment,
                policy,
            } => format!("loop:{loop_identity};containment:{containment};policy:{policy}"),
        }
    }
}
