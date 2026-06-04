use std::collections::BTreeMap;

use forge_foundational::facade::AspectValue;

use crate::facade::{SnapshotReadRecord, TruthSnapshotIdentity};
use crate::harness::fixtures::SnapshotFixture;

use super::{PricingDomainWorld, PricingMaterial};

impl PricingDomainWorld {
    pub(in crate::harness::tests) fn snapshot_fixture(
        &self,
        snapshot_identity: TruthSnapshotIdentity,
    ) -> SnapshotFixture {
        self.snapshot_fixture_with_overrides(snapshot_identity, [])
    }

    pub(in crate::harness::tests) fn snapshot_fixture_with_overrides<I>(
        &self,
        snapshot_identity: TruthSnapshotIdentity,
        overrides: I,
    ) -> SnapshotFixture
    where
        I: IntoIterator<Item = (PricingMaterial, i64)>,
    {
        let override_map = overrides.into_iter().collect::<BTreeMap<_, _>>();
        let mut records = Vec::new();
        for material in self.current_prices_microunits.keys() {
            let aspect_value_text = override_map
                .get(material)
                .copied()
                .unwrap_or_else(|| self.current_material_price_microunits(*material))
                .to_string();
            records.push(SnapshotReadRecord::for_request(
                &material.snapshot_read_request(),
                AspectValue::String(aspect_value_text.into()),
            ));
        }

        SnapshotFixture::new(snapshot_identity, records)
    }
}
