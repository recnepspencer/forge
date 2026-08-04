use super::super::super::digest::resource_canonical_digest;
use super::super::catalog::{
    ResourceMilestoneCPolicyCertificationFamily,
    REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES,
};
use super::super::digest_basis::ResourceMilestoneCPolicyCertificationBundleDigestBasis;
use super::contract::{
    ResourceMilestoneCPolicyCertificationBundle, ResourceMilestoneCPolicyCertificationRecord,
    ResourceMilestoneCPolicyCertificationSummary,
    RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_BUNDLE_SCHEMA_VERSION,
};
use std::collections::BTreeMap;

pub fn resource_milestone_c_policy_certification_bundle(
    records: impl IntoIterator<Item = ResourceMilestoneCPolicyCertificationRecord>,
) -> ResourceMilestoneCPolicyCertificationBundle {
    let mut records = records.into_iter().collect::<Vec<_>>();
    records.sort_by_key(|record| record.family);

    let mut by_family: BTreeMap<
        ResourceMilestoneCPolicyCertificationFamily,
        Vec<&ResourceMilestoneCPolicyCertificationRecord>,
    > = BTreeMap::new();
    for record in &records {
        by_family.entry(record.family).or_default().push(record);
    }

    let certified_family_count = REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES
        .iter()
        .filter(|family| {
            by_family.get(family).is_some_and(|records_for_family| {
                records_for_family.len() == 1 && records_for_family[0].passed()
            })
        })
        .count() as u32;
    let missing_family_count = REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES
        .iter()
        .filter(|family| !by_family.contains_key(family))
        .count() as u32;
    let duplicate_family_count = by_family
        .values()
        .filter(|records_for_family| records_for_family.len() > 1)
        .count() as u32;
    let failed_family_count = records.iter().filter(|record| !record.passed()).count() as u32
        + missing_family_count
        + duplicate_family_count;
    let summary = ResourceMilestoneCPolicyCertificationSummary {
        required_family_count: REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES.len()
            as u32,
        provided_record_count: records.len() as u32,
        certified_family_count,
        failed_family_count,
        missing_family_count,
        duplicate_family_count,
    };
    let bundle_digest =
        resource_canonical_digest(&ResourceMilestoneCPolicyCertificationBundleDigestBasis {
            schema_version: RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_BUNDLE_SCHEMA_VERSION,
            records: &records,
        });
    ResourceMilestoneCPolicyCertificationBundle {
        schema_version: RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_BUNDLE_SCHEMA_VERSION.to_owned(),
        records,
        summary,
        bundle_digest,
        passed: failed_family_count == 0,
    }
}
