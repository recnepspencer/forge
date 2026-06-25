use forge_query::facade::ForgeQueryReadFamily;

use super::super::read_family_adoption::WorthGraphReadAccessPlanAdoptionSeedPairing;

pub(crate) struct WorthGraphReadAccessPlanAdoptionAdmissionInput<'a> {
    pairing: &'a WorthGraphReadAccessPlanAdoptionSeedPairing,
    query_read_family: Option<&'a ForgeQueryReadFamily>,
}

impl<'a> WorthGraphReadAccessPlanAdoptionAdmissionInput<'a> {
    pub(crate) const fn missing_query_read_family_artifact(
        pairing: &'a WorthGraphReadAccessPlanAdoptionSeedPairing,
    ) -> Self {
        Self {
            pairing,
            query_read_family: None,
        }
    }

    pub(crate) const fn pairing(&self) -> &'a WorthGraphReadAccessPlanAdoptionSeedPairing {
        self.pairing
    }

    pub(crate) const fn query_read_family(&self) -> Option<&'a ForgeQueryReadFamily> {
        self.query_read_family
    }
}

pub(crate) const fn query_admission_api_required() -> &'static str {
    "forge_query::facade::admit_graph_read_access_for_family_in_authority(...)"
}
