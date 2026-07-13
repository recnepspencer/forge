use crate::contribution_descriptor::ContributionDescriptor;
use crate::contribution_kind::ContributionKind;
use crate::pack_name::PackName;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackRegistration {
    descriptor: ContributionDescriptor,
}

impl PackRegistration {
    pub fn new(descriptor: ContributionDescriptor) -> Self {
        Self { descriptor }
    }

    pub fn contribution_kind(&self) -> ContributionKind {
        self.descriptor.contribution_kind()
    }

    pub fn pack_name(&self) -> &PackName {
        self.descriptor.pack_name()
    }
}

#[cfg(test)]
mod composition_contract {
    use super::*;

    /// Structural cutover proof: composition, not copied fields.
    ///
    /// Exhaustive destructure admits only the canonical `descriptor` field.
    /// Restoring pre-cutover `contribution_kind`/`pack_name` storage fails to
    /// compile; a `..` rest pattern would silently hide that regression.
    #[test]
    fn pack_registration_composes_single_descriptor_field() {
        let pack_name = PackName::new("worth-pack-wall-basic").expect("valid pack name");
        let descriptor =
            ContributionDescriptor::new(ContributionKind::Component, pack_name.clone());
        let registration = PackRegistration::new(descriptor.clone());

        let PackRegistration { descriptor: held } = registration.clone();
        assert_eq!(held, descriptor);
        assert_eq!(registration.contribution_kind(), held.contribution_kind());
        assert_eq!(registration.pack_name(), held.pack_name());
        assert_eq!(registration.contribution_kind(), ContributionKind::Component);
        assert_eq!(registration.pack_name(), &pack_name);
        assert_eq!(registration.pack_name().as_str(), "worth-pack-wall-basic");
    }
}
