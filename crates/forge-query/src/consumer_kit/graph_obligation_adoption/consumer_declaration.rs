use crate::runtime::ForgeQueryGraphObligationRegistration;

use super::error::{
    ForgeQueryGraphObligationConsumerKitError, ForgeQueryGraphObligationConsumerKitErrorKind,
};
use super::kit_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationConsumerRegistrationDeclaration {
    family: String,
    registrations: Vec<ForgeQueryGraphObligationRegistration>,
    declaration_digest: String,
}

impl ForgeQueryGraphObligationConsumerRegistrationDeclaration {
    pub fn new(
        family: impl Into<String>,
        registrations: impl IntoIterator<Item = ForgeQueryGraphObligationRegistration>,
    ) -> Result<Self, ForgeQueryGraphObligationConsumerKitError> {
        let family = non_empty(family.into(), "registration family")?;
        let registrations = registrations.into_iter().collect::<Vec<_>>();
        if registrations.is_empty() {
            return Err(ForgeQueryGraphObligationConsumerKitError::new(
                ForgeQueryGraphObligationConsumerKitErrorKind::EmptyRegistrationDeclaration,
                "graph obligation consumer registration declaration must contain at least one registration",
            ));
        }
        let declaration_digest = kit_digest(
            "graph-obligation-consumer-registration",
            std::iter::once(family.as_str()).chain(
                registrations
                    .iter()
                    .map(ForgeQueryGraphObligationRegistration::registration_digest),
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
        registrations: impl IntoIterator<Item = ForgeQueryGraphObligationRegistration>,
    ) -> Result<Self, ForgeQueryGraphObligationConsumerKitError> {
        Self::new(family, registrations)
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn registrations(&self) -> &[ForgeQueryGraphObligationRegistration] {
        &self.registrations
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }
}

pub(super) fn non_empty(
    value: String,
    label: &'static str,
) -> Result<String, ForgeQueryGraphObligationConsumerKitError> {
    let value = value.trim().to_string();
    if value.is_empty() {
        return Err(ForgeQueryGraphObligationConsumerKitError::new(
            ForgeQueryGraphObligationConsumerKitErrorKind::BlankConsumerName,
            format!("{label} must not be blank"),
        ));
    }
    Ok(value)
}
