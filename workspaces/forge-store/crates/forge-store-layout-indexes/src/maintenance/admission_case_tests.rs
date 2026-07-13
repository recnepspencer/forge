use super::live_tests::validated_root_publication_authority;
use super::{layout_maintenance, IndexPublicationProtocol};
use crate::catalog::{ArtifactFamilyAccessLane, DurableArtifactMigrationPosture};
use crate::strategy::tests_support::{admit_btree_page_strategy, admit_lsm_wal_strategy};
use crate::strategy::LayoutStrategyFamily;
use crate::{
    access_planning, ExactPublicationAuthoritySource, IndexLagWitness, IndexMaintenanceMode,
    LagReason, LiveMaintenanceRequest, PhysicalMutationShape,
};
use forge_store_recovery_physics::LogSequenceNumber;
use std::collections::BTreeSet;

#[test]
fn maintenance_declares_exactly_the_cases_ordinary_admission_emits() {
    let btree = admit_btree_page_strategy();
    let lsm = admit_lsm_wal_strategy();
    let btree_authority = validated_root_publication_authority(71);
    let ExactPublicationAuthoritySource::CurrentRootPublication(validation) = btree_authority
    else {
        unreachable!()
    };
    let catalog = crate::bootstrap::test_support::bootstrap_catalog_read_admission();
    let btree_coverage = access_planning()
        .admit_btree_publication_materialization(btree.admitted_family(), &catalog, validation)
        .unwrap()
        .coverage()
        .clone();
    let other_btree_coverage = {
        let ExactPublicationAuthoritySource::CurrentRootPublication(other) =
            validated_root_publication_authority(73)
        else {
            unreachable!()
        };
        access_planning()
            .admit_btree_publication_materialization(btree.admitted_family(), &catalog, other)
            .unwrap()
            .coverage()
            .clone()
    };
    let lsm_coverage = access_planning()
        .exact_wal_lsn_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lsm.lifecycle().declaration().family(),
            ),
            LogSequenceNumber::new(79),
        )
        .unwrap();

    let lag = |mode, protocol, coverage| {
        IndexLagWitness::new(
            lsm.lifecycle().declaration().family(),
            coverage,
            mode,
            protocol,
            LagReason::BackgroundCatchUp,
        )
    };
    let lsm_request = |mode, lane, shape, protocol| {
        LiveMaintenanceRequest::new(
            lsm.admitted_family(),
            lsm.admitted_key_domain(),
            lsm.family(),
            lane,
            mode,
            shape,
            protocol,
        )
    };
    let btree_request = |mode, lane, shape, protocol| {
        LiveMaintenanceRequest::new(
            btree.admitted_family(),
            btree.admitted_key_domain(),
            btree.family(),
            lane,
            mode,
            shape,
            protocol,
        )
    };

    let mut observed = BTreeSet::new();
    let mut admit = |request| {
        observed.insert(layout_maintenance().admit_mutation(request).case_id());
    };

    admit(
        btree_request(
            IndexMaintenanceMode::SynchronousExact,
            ArtifactFamilyAccessLane::HotPath,
            PhysicalMutationShape::ObservationOnly,
            IndexPublicationProtocol::StableRootSwap,
        )
        .with_exact_coverage(btree_coverage.clone())
        .with_exact_publication_authority(btree_authority),
    );
    for (mode, lane, protocol) in [
        (
            IndexMaintenanceMode::AsynchronousLagged,
            ArtifactFamilyAccessLane::HotPath,
            IndexPublicationProtocol::DeferredCatchUp,
        ),
        (
            IndexMaintenanceMode::RebuildOnly,
            ArtifactFamilyAccessLane::MaintenancePath,
            IndexPublicationProtocol::CompactionCutover,
        ),
        (
            IndexMaintenanceMode::LazyMaterializedOnDemand,
            ArtifactFamilyAccessLane::MaintenancePath,
            IndexPublicationProtocol::DeferredCatchUp,
        ),
        (
            IndexMaintenanceMode::AdvisoryOnly,
            ArtifactFamilyAccessLane::MaintenancePath,
            IndexPublicationProtocol::DeferredCatchUp,
        ),
        (
            IndexMaintenanceMode::VerifierOnly,
            ArtifactFamilyAccessLane::VerifierPath,
            IndexPublicationProtocol::VerifierObservationOnly,
        ),
        (
            IndexMaintenanceMode::MigrationOnly,
            ArtifactFamilyAccessLane::MaintenancePath,
            IndexPublicationProtocol::MigrationCutover,
        ),
    ] {
        admit(
            lsm_request(mode, lane, PhysicalMutationShape::ObservationOnly, protocol)
                .with_lag_witness(lag(mode, protocol, lsm_coverage.clone())),
        );
    }

    admit(LiveMaintenanceRequest::new(
        btree.admitted_family(),
        btree.admitted_key_domain(),
        LayoutStrategyFamily::BaselineLsmWriteOptimized,
        ArtifactFamilyAccessLane::HotPath,
        IndexMaintenanceMode::SynchronousExact,
        PhysicalMutationShape::ObservationOnly,
        IndexPublicationProtocol::StableRootSwap,
    ));
    admit(btree_request(
        IndexMaintenanceMode::SynchronousExact,
        ArtifactFamilyAccessLane::MaintenancePath,
        PhysicalMutationShape::ObservationOnly,
        IndexPublicationProtocol::StableRootSwap,
    ));
    admit(btree_request(
        IndexMaintenanceMode::AsynchronousLagged,
        ArtifactFamilyAccessLane::HotPath,
        PhysicalMutationShape::LogStructuredAppend,
        IndexPublicationProtocol::CompactionCutover,
    ));
    admit(btree_request(
        IndexMaintenanceMode::SynchronousExact,
        ArtifactFamilyAccessLane::HotPath,
        PhysicalMutationShape::ObservationOnly,
        IndexPublicationProtocol::StableManifestInstall,
    ));
    admit(
        btree_request(
            IndexMaintenanceMode::SynchronousExact,
            ArtifactFamilyAccessLane::HotPath,
            PhysicalMutationShape::ObservationOnly,
            IndexPublicationProtocol::StableRootSwap,
        )
        .with_exact_coverage(btree_coverage.clone()),
    );
    admit(
        btree_request(
            IndexMaintenanceMode::SynchronousExact,
            ArtifactFamilyAccessLane::HotPath,
            PhysicalMutationShape::ObservationOnly,
            IndexPublicationProtocol::StableRootSwap,
        )
        .with_exact_coverage(other_btree_coverage.clone())
        .with_exact_publication_authority(btree_authority),
    );
    admit(btree_request(
        IndexMaintenanceMode::SynchronousExact,
        ArtifactFamilyAccessLane::HotPath,
        PhysicalMutationShape::ObservationOnly,
        IndexPublicationProtocol::StableRootSwap,
    ));
    admit(
        btree_request(
            IndexMaintenanceMode::SynchronousExact,
            ArtifactFamilyAccessLane::HotPath,
            PhysicalMutationShape::ObservationOnly,
            IndexPublicationProtocol::StableRootSwap,
        )
        .with_exact_coverage(lsm_coverage.clone()),
    );
    admit(lsm_request(
        IndexMaintenanceMode::AsynchronousLagged,
        ArtifactFamilyAccessLane::HotPath,
        PhysicalMutationShape::ObservationOnly,
        IndexPublicationProtocol::DeferredCatchUp,
    ));
    admit(
        btree_request(
            IndexMaintenanceMode::SynchronousExact,
            ArtifactFamilyAccessLane::HotPath,
            PhysicalMutationShape::ObservationOnly,
            IndexPublicationProtocol::StableRootSwap,
        )
        .with_exact_coverage(btree_coverage.clone())
        .with_exact_publication_authority(btree_authority)
        .with_lag_witness(IndexLagWitness::new(
            btree.lifecycle().declaration().family(),
            btree_coverage.clone(),
            IndexMaintenanceMode::SynchronousExact,
            IndexPublicationProtocol::StableRootSwap,
            LagReason::BackgroundCatchUp,
        )),
    );
    let wrong_posture = match lsm.migration_posture() {
        DurableArtifactMigrationPosture::StableNoMigration => {
            DurableArtifactMigrationPosture::VersionedMigration
        }
        _ => DurableArtifactMigrationPosture::StableNoMigration,
    };
    admit(
        lsm_request(
            IndexMaintenanceMode::MigrationOnly,
            ArtifactFamilyAccessLane::MaintenancePath,
            PhysicalMutationShape::ObservationOnly,
            IndexPublicationProtocol::MigrationCutover,
        )
        .with_lag_witness(lag(
            IndexMaintenanceMode::MigrationOnly,
            IndexPublicationProtocol::MigrationCutover,
            lsm_coverage.clone(),
        ))
        .require_migration_posture(wrong_posture),
    );
    admit(
        lsm_request(
            IndexMaintenanceMode::AsynchronousLagged,
            ArtifactFamilyAccessLane::HotPath,
            PhysicalMutationShape::LogStructuredAppend,
            IndexPublicationProtocol::DeferredCatchUp,
        )
        .with_lag_witness(lag(
            IndexMaintenanceMode::AsynchronousLagged,
            IndexPublicationProtocol::DeferredCatchUp,
            lsm_coverage.clone(),
        )),
    );
    admit(
        lsm_request(
            IndexMaintenanceMode::SynchronousExact,
            ArtifactFamilyAccessLane::HotPath,
            PhysicalMutationShape::ObservationOnly,
            IndexPublicationProtocol::StableManifestInstall,
        )
        .with_exact_coverage(lsm_coverage.clone())
        .with_exact_publication_authority(btree_authority),
    );
    let other_lsm_coverage = access_planning()
        .exact_wal_lsn_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lsm.lifecycle().declaration().family(),
            ),
            LogSequenceNumber::new(83),
        )
        .unwrap();
    admit(
        lsm_request(
            IndexMaintenanceMode::AsynchronousLagged,
            ArtifactFamilyAccessLane::HotPath,
            PhysicalMutationShape::ObservationOnly,
            IndexPublicationProtocol::DeferredCatchUp,
        )
        .with_exact_coverage(other_lsm_coverage)
        .with_lag_witness(lag(
            IndexMaintenanceMode::AsynchronousLagged,
            IndexPublicationProtocol::DeferredCatchUp,
            lsm_coverage,
        )),
    );

    assert_eq!(
        observed,
        super::maintenance_admission_cases().collect::<BTreeSet<_>>()
    );
}
