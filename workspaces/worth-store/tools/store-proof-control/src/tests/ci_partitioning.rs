use std::collections::BTreeSet;

use crate::ci::{
    catalog, required_lanes, CiCacheIdentity, CiCertificationAggregate, CiPartitionEvidence,
};
use crate::evidence::sha256_serialized;

use super::{current_inventory, workspace_root};

#[test]
fn every_ci_certifiable_product_has_one_partition_authority() {
    let inventory = current_inventory(&workspace_root());
    let catalog = catalog(&inventory);
    let products: BTreeSet<_> = inventory
        .inventory()
        .proofs
        .iter()
        .flat_map(|proof| proof.products.iter())
        .filter(|product| product.starts_with("store-ci:") || *product == "store-ui")
        .cloned()
        .collect();
    for product in products {
        let owners: Vec<_> = catalog
            .iter()
            .filter(|partition| partition.products.contains(&product))
            .map(|partition| partition.identity.as_str())
            .collect();
        assert_eq!(
            owners.len(),
            1,
            "partition ownership for {product}: {owners:?}"
        );
    }
}

#[test]
fn aggregate_denies_a_missing_windows_claim_and_retains_rerun_history() {
    let inventory = current_inventory(&workspace_root());
    let mut evidence: Vec<_> = required_lanes(&inventory)
        .into_iter()
        .map(|lane| synthetic_evidence(&lane.partition, &lane.operating_system, 1, true))
        .collect();
    evidence.push(synthetic_evidence("owner-unit", "linux", 0, false));
    let aggregate = CiCertificationAggregate::certify(&inventory, &evidence).unwrap();
    let owner_lane = aggregate
        .evidence_history
        .iter()
        .find(|history| history.lane.partition == "owner-unit")
        .unwrap();
    assert_eq!(owner_lane.evidence_identities.len(), 2);

    evidence.retain(|bundle| {
        !(bundle.partition == "fresh-process" && bundle.operating_system == "windows")
    });
    let denial = CiCertificationAggregate::certify(&inventory, &evidence).unwrap_err();
    assert!(denial.iter().any(|missing| {
        missing.partition == "fresh-process"
            && missing.operating_system == "windows"
            && missing.reason.contains("no evidence")
    }));
}

#[test]
fn cache_identity_rejects_toolchain_feature_and_profile_poisoning() {
    let cache = synthetic_cache();
    cache.validate().unwrap();
    let mutations: [fn(&mut CiCacheIdentity); 3] = [
        |cache: &mut CiCacheIdentity| cache.rustc_identity.push_str("-new"),
        |cache: &mut CiCacheIdentity| cache.feature_lanes.push("authority".to_owned()),
        |cache: &mut CiCacheIdentity| cache.profile.push("release".to_owned()),
    ];
    for mutate in mutations {
        let mut poisoned = cache.clone();
        mutate(&mut poisoned);
        assert!(poisoned.validate().is_err());
    }
}

fn synthetic_evidence(
    partition: &str,
    operating_system: &str,
    observed_unix_millis: u128,
    closeout_eligible: bool,
) -> CiPartitionEvidence {
    let mut evidence = CiPartitionEvidence {
        schema_version: 1,
        evidence_identity: String::new(),
        partition: partition.to_owned(),
        operating_system: operating_system.to_owned(),
        source_identity: "one-source".to_owned(),
        plan_digest: format!("plan-{partition}-{operating_system}"),
        run_identity: format!("run-{observed_unix_millis}"),
        cache_identity: synthetic_cache_for(partition, operating_system),
        shard_plan: None,
        behavioral_verdict: if closeout_eligible {
            "passed"
        } else {
            "failed"
        }
        .to_owned(),
        attempt_identities: if partition == "structural-preflight" {
            Vec::new()
        } else {
            vec![format!("attempt-{observed_unix_millis}")]
        },
        external_observer_authorities: if partition == "structural-preflight" {
            Vec::new()
        } else {
            vec!["independent-observer-process".to_owned()]
        },
        formal_tool_receipts: if partition == "formal-external" {
            vec!["formal-receipt".to_owned()]
        } else {
            Vec::new()
        },
        closeout_eligible,
        observed_unix_millis,
    };
    evidence.evidence_identity = sha256_serialized(&evidence).unwrap();
    evidence
}

fn synthetic_cache() -> CiCacheIdentity {
    synthetic_cache_for("partition", "linux")
}

fn synthetic_cache_for(partition: &str, operating_system: &str) -> CiCacheIdentity {
    let mut cache = CiCacheIdentity {
        identity: String::new(),
        operating_system: operating_system.to_owned(),
        architecture: "x86_64".to_owned(),
        rustc_identity: "rustc".to_owned(),
        profile: vec!["ci-test".to_owned()],
        feature_lanes: vec!["production-equivalent".to_owned()],
        lockfile_digest: "lock".to_owned(),
        workspace_manifest_digest: "manifest".to_owned(),
        cargo_config_digest: "config".to_owned(),
        partition: partition.to_owned(),
    };
    cache.identity = sha256_serialized(&cache).unwrap();
    cache
}
