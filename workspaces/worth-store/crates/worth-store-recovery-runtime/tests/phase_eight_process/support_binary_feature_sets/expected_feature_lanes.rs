use std::collections::{BTreeMap, BTreeSet};

use super::FeatureMap;

pub(super) fn writer_local_map() -> FeatureMap {
    expected_map(&[
        ("worth-foundational", "worth_foundational", &[]),
        ("worth-proof", "worth_proof", &[]),
        (
            "worth-signal",
            "worth_signal",
            &["default", "profile-extended"],
        ),
        ("worth-store", "build-script-build", &[]),
        ("worth-store", "physical_store_c8_writer", &[]),
        ("worth-store", "worth_store", &[]),
        (
            "worth-store-aspect-native",
            "worth_store_aspect_native",
            &[],
        ),
        ("worth-store-authority", "worth_store_authority", &[]),
        ("worth-store-budgets", "worth_store_budgets", &[]),
        ("worth-store-buffer-pool", "worth_store_buffer_pool", &[]),
        ("worth-store-contracts", "worth_store_contracts", &[]),
        ("worth-store-io-scheduler", "worth_store_io_scheduler", &[]),
        ("worth-store-modes", "worth_store_modes", &[]),
        (
            "worth-store-physical-backend",
            "build-script-build",
            &["store-runtime-owner"],
        ),
        (
            "worth-store-physical-backend",
            "worth_store_physical_backend",
            &["store-runtime-owner"],
        ),
        (
            "worth-store-physical-format",
            "worth_store_physical_format",
            &[],
        ),
        ("worth-store-security", "worth_store_security", &[]),
        ("worth-store-wal", "worth_store_wal", &["default"]),
    ])
}

pub(super) fn observer_local_map() -> FeatureMap {
    expected_map(&[
        ("worth-foundational", "worth_foundational", &[]),
        ("worth-proof", "worth_proof", &[]),
        (
            "worth-signal",
            "worth_signal",
            &["default", "profile-extended"],
        ),
        ("worth-store", "build-script-build", &["default"]),
        ("worth-store", "worth_store", &["default"]),
        (
            "worth-store-aspect-native",
            "worth_store_aspect_native",
            &[],
        ),
        ("worth-store-authority", "worth_store_authority", &[]),
        ("worth-store-blob-chunks", "worth_store_blob_chunks", &[]),
        ("worth-store-budgets", "worth_store_budgets", &[]),
        ("worth-store-buffer-pool", "worth_store_buffer_pool", &[]),
        (
            "worth-store-compatibility",
            "worth_store_compatibility",
            &[],
        ),
        ("worth-store-contracts", "worth_store_contracts", &[]),
        ("worth-store-io-scheduler", "worth_store_io_scheduler", &[]),
        (
            "worth-store-layout-indexes",
            "worth_store_layout_indexes",
            &[],
        ),
        (
            "worth-store-lsm-authority",
            "worth_store_lsm_authority",
            &["default"],
        ),
        ("worth-store-modes", "worth_store_modes", &[]),
        (
            "worth-store-offline-verifier",
            "physical_store_offline_observer",
            &[],
        ),
        (
            "worth-store-offline-verifier",
            "worth_store_offline_verifier",
            &[],
        ),
        (
            "worth-store-physical-backend",
            "build-script-build",
            &["store-runtime-owner"],
        ),
        (
            "worth-store-physical-backend",
            "worth_store_physical_backend",
            &["store-runtime-owner"],
        ),
        (
            "worth-store-physical-format",
            "worth_store_physical_format",
            &[],
        ),
        (
            "worth-store-physical-integrity",
            "worth_store_physical_integrity",
            &[],
        ),
        (
            "worth-store-physical-isolation",
            "worth_store_physical_isolation",
            &[],
        ),
        (
            "worth-store-reclaim-policy",
            "worth_store_reclaim_policy",
            &[],
        ),
        (
            "worth-store-recovery-physics",
            "worth_store_recovery_physics",
            &["default"],
        ),
        (
            "worth-store-replication",
            "worth_store_replication",
            &["default"],
        ),
        ("worth-store-retention", "worth_store_retention", &[]),
        ("worth-store-security", "worth_store_security", &[]),
        ("worth-store-tiering", "worth_store_tiering", &[]),
        ("worth-store-wal", "worth_store_wal", &["default"]),
    ])
}

pub(super) fn recovery_local_map() -> FeatureMap {
    expected_map(&[
        ("worth-foundational", "worth_foundational", &[]),
        ("worth-proof", "worth_proof", &[]),
        (
            "worth-signal",
            "worth_signal",
            &["default", "profile-extended"],
        ),
        (
            "worth-store",
            "build-script-build",
            &[
                "certification-test-authority",
                "default",
                "recovery-runtime-owner",
            ],
        ),
        (
            "worth-store",
            "worth_store",
            &[
                "certification-test-authority",
                "default",
                "recovery-runtime-owner",
            ],
        ),
        (
            "worth-store-aspect-native",
            "worth_store_aspect_native",
            &[],
        ),
        ("worth-store-authority", "worth_store_authority", &[]),
        ("worth-store-budgets", "worth_store_budgets", &[]),
        (
            "worth-store-buffer-pool",
            "worth_store_buffer_pool",
            &["certification-test-authority"],
        ),
        ("worth-store-contracts", "worth_store_contracts", &[]),
        (
            "worth-store-io-scheduler",
            "worth_store_io_scheduler",
            &["certification-test-authority"],
        ),
        ("worth-store-modes", "worth_store_modes", &[]),
        (
            "worth-store-physical-backend",
            "build-script-build",
            &[
                "certification-test-authority",
                "recovery-runtime-owner",
                "store-runtime-owner",
            ],
        ),
        (
            "worth-store-physical-backend",
            "worth_store_physical_backend",
            &[
                "certification-test-authority",
                "recovery-runtime-owner",
                "store-runtime-owner",
            ],
        ),
        (
            "worth-store-physical-format",
            "worth_store_physical_format",
            &[],
        ),
        (
            "worth-store-recovery-physics",
            "worth_store_recovery_physics",
            &["default"],
        ),
        (
            "worth-store-recovery-runtime",
            "physical_store_recover",
            &["certification-test-authority"],
        ),
        (
            "worth-store-recovery-runtime",
            "worth_store_recovery_runtime",
            &["certification-test-authority"],
        ),
        (
            "worth-store-security",
            "worth_store_security",
            &["certification-test-authority"],
        ),
        ("worth-store-wal", "worth_store_wal", &["default"]),
    ])
}

pub(super) fn recovery_projection() -> BTreeMap<String, BTreeSet<String>> {
    [
        (
            "worth-store".to_owned(),
            BTreeSet::from([
                "certification-test-authority".to_owned(),
                "recovery-runtime-owner".to_owned(),
            ]),
        ),
        (
            "worth-store-physical-backend".to_owned(),
            BTreeSet::from([
                "certification-test-authority".to_owned(),
                "recovery-runtime-owner".to_owned(),
            ]),
        ),
        (
            "worth-store-buffer-pool".to_owned(),
            BTreeSet::from(["certification-test-authority".to_owned()]),
        ),
        (
            "worth-store-io-scheduler".to_owned(),
            BTreeSet::from(["certification-test-authority".to_owned()]),
        ),
        (
            "worth-store-security".to_owned(),
            BTreeSet::from(["certification-test-authority".to_owned()]),
        ),
        (
            "worth-store-recovery-runtime".to_owned(),
            BTreeSet::from(["certification-test-authority".to_owned()]),
        ),
    ]
    .into_iter()
    .collect()
}

fn expected_map(entries: &[(&str, &str, &[&str])]) -> FeatureMap {
    entries
        .iter()
        .map(|(package, target, features)| {
            (
                ((*package).to_owned(), (*target).to_owned()),
                features
                    .iter()
                    .map(|feature| (*feature).to_owned())
                    .collect(),
            )
        })
        .collect()
}
