use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TierResidenceClass {
    Hot,
    Warm,
    Cold,
}

impl TierResidenceClass {
    pub fn label(self) -> &'static str {
        match self {
            Self::Hot => "hot",
            Self::Warm => "warm",
            Self::Cold => "cold",
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "hot" => Some(Self::Hot),
            "warm" => Some(Self::Warm),
            "cold" => Some(Self::Cold),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PlacementBudgetClass {
    ForegroundResidentOnly,
    ForegroundBoundedRecall,
    BackgroundOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RecallCostClass {
    Inline,
    Bounded,
    Deferred,
}

impl RecallCostClass {
    pub fn label(self) -> &'static str {
        match self {
            Self::Inline => "inline",
            Self::Bounded => "bounded",
            Self::Deferred => "deferred",
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "inline" => Some(Self::Inline),
            "bounded" => Some(Self::Bounded),
            "deferred" => Some(Self::Deferred),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PlacementExecutionOrigin {
    Foreground,
    Background,
    RestartRecovery,
}

impl PlacementExecutionOrigin {
    pub fn label(self) -> &'static str {
        match self {
            Self::Foreground => "foreground",
            Self::Background => "background",
            Self::RestartRecovery => "restart_recovery",
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "foreground" => Some(Self::Foreground),
            "background" => Some(Self::Background),
            "restart_recovery" => Some(Self::RestartRecovery),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RecallAmplificationBudget {
    SingleFamilyLocalUnit,
    BroadenedPlanRequired,
}

impl RecallAmplificationBudget {
    pub fn label(self) -> &'static str {
        match self {
            Self::SingleFamilyLocalUnit => "single_family_local_unit",
            Self::BroadenedPlanRequired => "broadened_plan_required",
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "single_family_local_unit" => Some(Self::SingleFamilyLocalUnit),
            "broadened_plan_required" => Some(Self::BroadenedPlanRequired),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HotnessClassificationVerdict {
    Hot,
    Warm,
    CoolingDebt,
}
