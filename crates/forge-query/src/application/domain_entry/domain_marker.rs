use crate::application::ForgeQueryCapabilityFamily;

pub trait ForgeQueryDomainEntryMarker: Clone + Copy + Eq {
    fn domain_key(&self) -> &'static str;

    fn display_name(&self) -> &'static str;

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily];
}
