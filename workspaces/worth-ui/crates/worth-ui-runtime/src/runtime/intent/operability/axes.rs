#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentSupportPosture {
    Supported,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentMutabilityPosture {
    Writable,
    Readonly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentReadinessPosture {
    Ready,
    Pending,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentOccupancyPosture {
    Idle,
    InFlight,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentPolicyPosture {
    Admitted,
    Denied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentAffinityPosture {
    Current,
    Stale,
    WrongWorld,
    RebindRequired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiIntentConfirmationPosture {
    NotRequired,
    Required { policy_identity: Box<str> },
}

impl UiIntentConfirmationPosture {
    pub fn required_policy_identity(&self) -> Option<&str> {
        match self {
            Self::NotRequired => None,
            Self::Required { policy_identity } => Some(policy_identity),
        }
    }
}
