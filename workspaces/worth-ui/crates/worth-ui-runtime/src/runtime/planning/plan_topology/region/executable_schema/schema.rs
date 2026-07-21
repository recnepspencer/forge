use crate::runtime::WorthUiPlanNodeInput;

use super::WorthUiPlanRegionIdentity;

#[derive(Clone, Debug)]
pub(crate) struct WorthUiPlanRegionSchema {
    identity: WorthUiPlanRegionIdentity,
    input: WorthUiPlanNodeInput,
    narrowing_fingerprint: u64,
}

impl WorthUiPlanRegionSchema {
    pub(crate) fn from_node_input(input: WorthUiPlanNodeInput) -> Self {
        let identity = WorthUiPlanRegionIdentity::from_exact_basis(input.identity_basis());
        let narrowing_fingerprint = schema_fingerprint(&input);
        Self {
            identity,
            input,
            narrowing_fingerprint,
        }
    }

    pub(crate) fn identity(&self) -> &WorthUiPlanRegionIdentity {
        &self.identity
    }

    pub(crate) fn input(&self) -> &WorthUiPlanNodeInput {
        &self.input
    }

    pub(crate) fn narrowing_fingerprint(&self) -> u64 {
        self.narrowing_fingerprint
    }

    pub(crate) fn exactly_matches(&self, other: &Self) -> bool {
        self.input.executable_schema_matches(&other.input)
    }

    pub(crate) fn exactly_matches_after_narrowing(&self, other: &Self) -> bool {
        self.exactly_matches(other)
    }
}

fn schema_fingerprint(input: &WorthUiPlanNodeInput) -> u64 {
    let family = input.family() as u64;
    let topology = input.topology_input();
    0x7265_6769_6f6e_0001
        ^ family.rotate_left(17)
        ^ (topology.root_region_count() as u64).rotate_left(29)
        ^ (topology.region_count() as u64).rotate_left(37)
        ^ (topology.mount_count() as u64).rotate_left(43)
        ^ (topology.max_region_depth() as u64).rotate_left(53)
}
