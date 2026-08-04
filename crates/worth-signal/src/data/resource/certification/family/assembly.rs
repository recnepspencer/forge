use super::super::digest::resource_canonical_digest;
use super::catalog::{ResourceCertificationFamily, REQUIRED_RESOURCE_CERTIFICATION_FAMILIES};
use super::contract::{
    ResourceCertificationBundle, ResourceCertificationFailure, ResourceCertificationRecord,
    ResourceCertificationSummary, RESOURCE_CERTIFICATION_BUNDLE_SCHEMA_VERSION,
};
use super::digest_basis::ResourceCertificationBundleDigestBasis;
use std::collections::{BTreeMap, BTreeSet};

pub fn resource_certification_bundle(
    records: impl IntoIterator<Item = ResourceCertificationRecord>,
) -> ResourceCertificationBundle {
    let mut records = records.into_iter().collect::<Vec<_>>();
    records.sort_by_key(|record| record.family);

    let mut by_family: BTreeMap<ResourceCertificationFamily, Vec<&ResourceCertificationRecord>> =
        BTreeMap::new();
    for record in &records {
        by_family.entry(record.family).or_default().push(record);
    }

    let mut failures = Vec::new();
    for family in REQUIRED_RESOURCE_CERTIFICATION_FAMILIES {
        match by_family.get(&family) {
            None => failures.push(ResourceCertificationFailure::MissingRequiredFamily { family }),
            Some(records_for_family) if records_for_family.len() > 1 => {
                failures.push(ResourceCertificationFailure::DuplicateFamily {
                    family,
                    count: records_for_family.len() as u32,
                });
            }
            Some(_) => {}
        }
    }

    for record in &records {
        if record.evidence_digest.is_empty() {
            failures.push(ResourceCertificationFailure::EmptyEvidenceDigest {
                family: record.family,
            });
        }
        if !record.passed {
            failures.push(ResourceCertificationFailure::FailedFamily {
                family: record.family,
            });
        }
    }

    let failed_families = failures
        .iter()
        .map(ResourceCertificationFailure::family)
        .collect::<BTreeSet<_>>();
    let passed_family_count = REQUIRED_RESOURCE_CERTIFICATION_FAMILIES
        .iter()
        .filter(|family| {
            by_family
                .get(family)
                .is_some_and(|records_for_family| records_for_family.len() == 1)
                && !failed_families.contains(family)
        })
        .count() as u32;
    let missing_family_count = REQUIRED_RESOURCE_CERTIFICATION_FAMILIES
        .iter()
        .filter(|family| !by_family.contains_key(family))
        .count() as u32;
    let duplicate_family_count = by_family
        .values()
        .filter(|records_for_family| records_for_family.len() > 1)
        .count() as u32;
    let failed_family_count = failed_families.len() as u32;
    let summary = ResourceCertificationSummary {
        required_family_count: REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len() as u32,
        provided_record_count: records.len() as u32,
        passed_family_count,
        failed_family_count,
        missing_family_count,
        duplicate_family_count,
    };
    let bundle_digest = resource_canonical_digest(&ResourceCertificationBundleDigestBasis {
        schema_version: RESOURCE_CERTIFICATION_BUNDLE_SCHEMA_VERSION,
        records: &records,
    });
    let passed = failures.is_empty();
    ResourceCertificationBundle {
        schema_version: RESOURCE_CERTIFICATION_BUNDLE_SCHEMA_VERSION.to_owned(),
        records,
        summary,
        bundle_digest,
        passed,
        failures,
    }
}
