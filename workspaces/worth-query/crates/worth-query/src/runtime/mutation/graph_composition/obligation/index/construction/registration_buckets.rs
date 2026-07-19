use std::collections::BTreeMap;

use crate::runtime::{
    WorthQueryGraphObligationRegistration, WorthQueryGraphObligationRegistrationCatalog,
};

use super::super::lookup::{
    WorthQueryGraphObligationOperatingWorldLookupKey, WorthQueryGraphObligationTouchLookupKey,
};
use super::WorthQueryGraphObligationIndexEntry;

pub(in crate::runtime::mutation::graph_composition::obligation::index) type GraphObligationBucketKey = (
    WorthQueryGraphObligationTouchLookupKey,
    WorthQueryGraphObligationOperatingWorldLookupKey,
);

pub(in crate::runtime::mutation::graph_composition::obligation::index) type GraphObligationBuckets =
    BTreeMap<GraphObligationBucketKey, Vec<WorthQueryGraphObligationRegistration>>;

pub(in crate::runtime::mutation::graph_composition::obligation::index) struct WorthQueryGraphObligationIndexRegistrationBuckets
{
    entries: Vec<WorthQueryGraphObligationIndexEntry>,
    buckets: GraphObligationBuckets,
}

impl WorthQueryGraphObligationIndexRegistrationBuckets {
    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn from_catalog(
        catalog: &WorthQueryGraphObligationRegistrationCatalog,
    ) -> Self {
        let mut entries = Vec::new();
        let mut buckets: GraphObligationBuckets = BTreeMap::new();
        for registration in catalog.registrations() {
            let touch_key = WorthQueryGraphObligationTouchLookupKey::from_selector(
                registration.touch_selector(),
            );
            let operating_world_key =
                WorthQueryGraphObligationOperatingWorldLookupKey::from_selector(
                    registration.operating_world_selector(),
                );
            entries.push(WorthQueryGraphObligationIndexEntry::new(
                &touch_key,
                operating_world_key,
                registration.clone(),
            ));
            buckets
                .entry((touch_key, operating_world_key))
                .or_default()
                .push(registration.clone());
        }
        sort_registration_entries_by_digest(&mut entries);
        sort_and_deduplicate_bucket_registrations(&mut buckets);
        Self { entries, buckets }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn entries(
        &self,
    ) -> &[WorthQueryGraphObligationIndexEntry] {
        &self.entries
    }

    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn entry_count(
        &self,
    ) -> usize {
        self.entries.len()
    }

    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn bucket_count(
        &self,
    ) -> usize {
        self.buckets.len()
    }

    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn into_parts(
        self,
    ) -> (
        Vec<WorthQueryGraphObligationIndexEntry>,
        GraphObligationBuckets,
    ) {
        (self.entries, self.buckets)
    }
}

fn sort_registration_entries_by_digest(entries: &mut [WorthQueryGraphObligationIndexEntry]) {
    entries.sort_by(|left, right| left.entry_digest().cmp(right.entry_digest()));
}

fn sort_and_deduplicate_bucket_registrations(buckets: &mut GraphObligationBuckets) {
    for registrations in buckets.values_mut() {
        registrations
            .sort_by(|left, right| left.registration_digest().cmp(right.registration_digest()));
        registrations
            .dedup_by(|left, right| left.registration_digest() == right.registration_digest());
    }
}
