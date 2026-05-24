use crate::application::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDomainEntryMarker,
};

pub trait ForgeQueryDomainOperatingContext<D: ForgeQueryDomainEntryMarker>: Clone + Eq {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily];

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily];

    fn context_identity_digest(&self) -> String;
}
