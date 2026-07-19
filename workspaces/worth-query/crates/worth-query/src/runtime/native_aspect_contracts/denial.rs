use worth_foundational::facade::AspectKey;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryAspectContractRegistrationDenialKind {
    ConflictingContract,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAspectContractRegistrationDenial {
    kind: WorthQueryAspectContractRegistrationDenialKind,
    aspect_key: AspectKey,
}

impl WorthQueryAspectContractRegistrationDenial {
    pub(super) fn conflicting_contract(aspect_key: AspectKey) -> Self {
        Self {
            kind: WorthQueryAspectContractRegistrationDenialKind::ConflictingContract,
            aspect_key,
        }
    }

    pub fn kind(&self) -> WorthQueryAspectContractRegistrationDenialKind {
        self.kind
    }

    pub fn aspect_key(&self) -> &AspectKey {
        &self.aspect_key
    }
}

impl std::fmt::Display for WorthQueryAspectContractRegistrationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "conflicting Foundational aspect contracts were registered for `{}`",
            self.aspect_key.as_str()
        )
    }
}

impl std::error::Error for WorthQueryAspectContractRegistrationDenial {}
