#[derive(Clone, Copy)]
pub(super) struct ModelInput {
    pub(super) supported: bool,
    pub(super) writable: bool,
    pub(super) ready: bool,
    pub(super) idle: bool,
    pub(super) policy_admitted: bool,
    pub(super) affinity: ModelAffinity,
    pub(super) confirmation_required: bool,
}

#[derive(Clone, Copy)]
pub(super) enum ModelAffinity {
    Current,
    Stale,
    WrongWorld,
    RebindRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ModelCause {
    Unsupported,
    WrongWorld,
    RebindRequired,
    StaleTarget,
    PolicyDenied,
    Occupied,
    Readonly,
    Pending,
    ConfirmationRequired,
}

pub(super) fn causes(input: ModelInput) -> Vec<ModelCause> {
    let mut causes = Vec::with_capacity(9);
    if !input.supported {
        causes.push(ModelCause::Unsupported);
    }
    match input.affinity {
        ModelAffinity::Current => {}
        ModelAffinity::Stale => causes.push(ModelCause::StaleTarget),
        ModelAffinity::WrongWorld => causes.push(ModelCause::WrongWorld),
        ModelAffinity::RebindRequired => causes.push(ModelCause::RebindRequired),
    }
    if !input.policy_admitted {
        causes.push(ModelCause::PolicyDenied);
    }
    if !input.idle {
        causes.push(ModelCause::Occupied);
    }
    if !input.writable {
        causes.push(ModelCause::Readonly);
    }
    if !input.ready {
        causes.push(ModelCause::Pending);
    }
    if input.confirmation_required {
        causes.push(ModelCause::ConfirmationRequired);
    }
    causes
}
