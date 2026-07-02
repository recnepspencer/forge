use crate::declaration::UiAspectContract;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiDeclaredAspectPayload {
    contract: UiAspectContract,
}

impl UiDeclaredAspectPayload {
    pub(crate) const fn new(contract: UiAspectContract) -> Self {
        Self { contract }
    }

    pub(crate) const fn contract(&self) -> &UiAspectContract {
        &self.contract
    }
}
