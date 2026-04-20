use crate::{
    DurableMutationRequest, ForgeStoreBuilder, MaintenanceArtifactFamily,
    MaintenanceRecoveryDisposition, ObservedPublicationFailure, ObservedRecoveryFailure356,
    PublicationClassification, RecoveryOperatorActionKind, RecoveryOperatorDisposition,
    SnapshotCaptureRequest, StoreError,
};

use super::harness::{
    certification::{
        assertions::{assert_all_equal, assert_rejection_payloads_present},
        core::{AssertionClass, CanonicalRow, CertificationSuite, LaneResult, RejectionRow},
        requirements::{
            evaluate_completeness, ADVERSARIAL_CRASH_RECOVERY_SOURCE_PRECEDENCE_TEST,
            DURABLE_MEDIA_WRITE_PATH_CERTIFICATION_TEST,
        },
    },
    corruption::local_file::force_branch_head_gap,
    fixtures::{
        runtime::{
            create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
        },
        stores::{unique_test_sqlite_path, unique_test_store_path},
    },
    scenarios::{
        publication::{create_alpha_commit, durable_publication_reports},
        recovery::{create_beta_commit, recovery_and_rebuild_equivalence},
    },
};

#[path = "milestone_3_5_3_6_certification/helpers.rs"]
mod helpers;
#[path = "milestone_3_5_3_6_certification/suite_publication.rs"]
mod suite_publication;
#[path = "milestone_3_5_3_6_certification/suite_recovery.rs"]
mod suite_recovery;
#[path = "milestone_3_5_3_6_certification/evidence_bundles.rs"]
mod evidence_bundles;
#[path = "milestone_3_5_3_6_certification/snapshot_maintenance.rs"]
mod snapshot_maintenance;

use helpers::*;
