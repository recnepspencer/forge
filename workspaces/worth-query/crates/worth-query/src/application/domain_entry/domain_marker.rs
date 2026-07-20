use crate::application::WorthQueryCapabilityFamily;

pub trait WorthQueryDomainEntryMarker: Clone + Copy + Eq {
    fn domain_key(&self) -> &'static str;

    fn display_name(&self) -> &'static str;

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily];
}
