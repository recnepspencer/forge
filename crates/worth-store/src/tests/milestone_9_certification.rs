use crate::{
    BulkCheckpointPolicy, BulkIngestSourceRequest, BulkPlanKind, BulkSourceMember,
    BulkTransformRequest, ChunkOrdinal, ChunkWidthBudget, WORTHStore, WORTHStoreBuilder,
    Milestone9CertificationBundle,
};

use super::harness::fixtures::runtime::{
    create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
};
use super::harness::fixtures::stores::unique_test_store_path;

fn assert_certification_core(
    bundle: &Milestone9CertificationBundle,
    plan_kind: BulkPlanKind,
    chunk_count: usize,
) {
    assert_eq!(bundle.plan_kind, plan_kind);
    assert_eq!(bundle.chunk_count, chunk_count);
    assert!(bundle.certification_summary.truth_matches_control_lane);
    assert!(bundle.certification_summary.history_matches_control_lane);
    assert!(bundle.certification_summary.restore_truth_parity);
    assert!(bundle.certification_summary.restore_history_parity);
    assert!(
        bundle
            .certification_summary
            .deterministic_chunk_plan_observed
    );
    assert!(!bundle.chunk_plan_digest.is_empty());
}

#[path = "milestone_9_certification/ingest.rs"]
mod ingest;
#[path = "milestone_9_certification/transform.rs"]
mod transform;
#[path = "milestone_9_certification/wal_ingest.rs"]
mod wal_ingest;
#[path = "milestone_9_certification/wal_transform.rs"]
mod wal_transform;

use ingest::*;
use transform::*;
use wal_ingest::*;
