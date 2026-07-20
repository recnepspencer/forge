use crate::runtime::WorthQueryGraphObligationRegistration;

use super::error::{
    WorthQueryGraphObligationConsumerKitError, WorthQueryGraphObligationConsumerKitErrorKind,
};
use super::kit_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationConsumerRegistrationDeclaration {
    family: String,
    registrations: Vec<WorthQueryGraphObligationRegistration>,
    declaration_digest: String,
}

impl WorthQueryGraphObligationConsumerRegistrationDeclaration {
    pub fn new(
        family: impl Into<String>,
        registrations: impl IntoIterator<Item = WorthQueryGraphObligationRegistration>,
    ) -> Result<Self, WorthQueryGraphObligationConsumerKitError> {
        let family = non_empty(family.into(), "registration family")?;
        let registrations = registrations.into_iter().collect::<Vec<_>>();
        if registrations.is_empty() {
            return Err(WorthQueryGraphObligationConsumerKitError::new(
                WorthQueryGraphObligationConsumerKitErrorKind::EmptyRegistrationDeclaration,
                "graph obligation consumer registration declaration must contain at least one registration",
            ));
        }
        let declaration_digest = kit_digest(
            "graph-obligation-consumer-registration",
            std::iter::once(family.as_str()).chain(
                registrations
                    .iter()
                    .map(WorthQueryGraphObligationRegistration::registration_digest),
            ),
        );
        Ok(Self {
            family,
            registrations,
            declaration_digest,
        })
    }

    pub fn for_runtime_family(
        family: impl Into<String>,
        registrations: impl IntoIterator<Item = WorthQueryGraphObligationRegistration>,
    ) -> Result<Self, WorthQueryGraphObligationConsumerKitError> {
        Self::new(family, registrations)
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn registrations(&self) -> &[WorthQueryGraphObligationRegistration] {
        &self.registrations
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }
}

pub(super) fn non_empty(
    value: String,
    label: &'static str,
) -> Result<String, WorthQueryGraphObligationConsumerKitError> {
    let value = value.trim().to_string();
    if value.is_empty() {
        return Err(WorthQueryGraphObligationConsumerKitError::new(
            WorthQueryGraphObligationConsumerKitErrorKind::BlankConsumerName,
            format!("{label} must not be blank"),
        ));
    }
    Ok(value)
}
