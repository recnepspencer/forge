use worth_store_physical_backend::{MediaCounterSnapshot, QualifiedFilesystemMedia};
use worth_store_physical_format::DurablePhysicalRootManifest;

use super::super::publication::{
    indeterminate_physical_work, unpublished_physical_work, unpublished_semantic, PublicationPlan,
    RecordPublicationStage,
};
use super::super::residency::publication_artifacts::PublicationRecordArtifacts;
use super::super::{PublishedRecordBatch, RecordAppendError};
use super::{
    catalog_candidate_progression::{synchronize_catalog_candidate, CatalogCandidateSynchronized},
    manifest_progression::{synchronize_manifests, DataSynchronized},
};

struct CatalogReplaced(PublicationPlan);
struct NamespaceSynchronized(PublicationPlan);

pub(in crate::physical_runtime::record_serving) fn execute_prepared_root(
    mutation: &super::super::CanonicalRecordMutationPort,
    media: &QualifiedFilesystemMedia,
    plan: PublicationPlan,
    replacement: super::super::PreparedCatalogReplacement,
    residency: &mut super::super::residency::frame_ports::StoreCandidateFramePublicationSession,
    counters_before: MediaCounterSnapshot,
    #[cfg(feature = "certification-test-authority")] reject_catalog_eligibility_join: bool,
) -> Result<(PublishedRecordBatch, DurablePhysicalRootManifest), RecordAppendError> {
    let artifacts = PublicationRecordArtifacts::new(mutation);
    let manifests_synchronized = synchronize_manifests(
        media,
        &artifacts,
        DataSynchronized::new(plan),
        residency,
        counters_before,
    )?;
    let candidate_synchronized = synchronize_catalog_candidate(
        media,
        &artifacts,
        manifests_synchronized,
        residency,
        counters_before,
    )?;
    let settled = candidate_synchronized.settled_artifacts();
    let frame_set = super::catalog_cutover_preflight::validate_frame_set(
        media,
        candidate_synchronized.plan(),
        residency,
        counters_before,
    )?;
    #[cfg(feature = "certification-test-authority")]
    let frame_set = if reject_catalog_eligibility_join {
        frame_set.certification_mismatched()
    } else {
        frame_set
    };
    let residency_prepared = super::catalog_cutover_preflight::prepare_residency(
        media,
        candidate_synchronized.plan(),
        residency,
        counters_before,
    )?;
    let eligibility =
        super::CatalogReplacementEligibility::join(settled, frame_set, residency_prepared)
            .ok_or_else(|| {
                unpublished_semantic(
                    media,
                    candidate_synchronized.plan(),
                    counters_before,
                    RecordPublicationStage::CatalogReplacement,
                    super::super::RecordAppendDenial::CatalogReplacementEligibilityMismatch,
                )
            })?;
    let catalog_replaced = replace_catalog(
        media,
        replacement,
        eligibility,
        candidate_synchronized,
        counters_before,
    )?;
    let namespace_synchronized =
        synchronize_namespace(media, &artifacts, catalog_replaced, counters_before)?;
    Ok(complete_publication(
        media,
        namespace_synchronized,
        counters_before,
    ))
}

fn replace_catalog(
    media: &QualifiedFilesystemMedia,
    replacement: super::super::PreparedCatalogReplacement,
    eligibility: super::CatalogReplacementEligibility,
    synchronized: CatalogCandidateSynchronized,
    before: MediaCounterSnapshot,
) -> Result<CatalogReplaced, RecordAppendError> {
    let identity = replacement.execute(eligibility).map_err(|failure| {
        if failure.effect_fate() == crate::physical_runtime::PhysicalWorkEffectFate::Indeterminate {
            indeterminate_physical_work(
                media,
                synchronized.plan(),
                before,
                RecordPublicationStage::CatalogReplacement,
                &failure,
            )
        } else {
            unpublished_physical_work(
                media,
                synchronized.plan(),
                before,
                RecordPublicationStage::CatalogReplacement,
                &failure,
            )
        }
    })?;
    let mut plan = synchronized.into_plan();
    plan.work
        .record(RecordPublicationStage::CatalogReplacement, identity);
    Ok(CatalogReplaced(plan))
}

fn synchronize_namespace(
    media: &QualifiedFilesystemMedia,
    artifacts: &PublicationRecordArtifacts<'_>,
    replaced: CatalogReplaced,
    before: MediaCounterSnapshot,
) -> Result<NamespaceSynchronized, RecordAppendError> {
    let mut replaced = replaced;
    let synchronization = {
        let mut stage = artifacts.at(
            RecordPublicationStage::NamespaceSynchronization,
            &mut replaced.0.work,
        );
        stage.synchronize_record_family()
    };
    synchronization.map_err(|failure| {
        indeterminate_physical_work(
            media,
            &replaced.0,
            before,
            RecordPublicationStage::NamespaceSynchronization,
            &failure,
        )
    })?;
    Ok(NamespaceSynchronized(replaced.0))
}

fn complete_publication(
    media: &QualifiedFilesystemMedia,
    mut synchronized: NamespaceSynchronized,
    before: MediaCounterSnapshot,
) -> (PublishedRecordBatch, DurablePhysicalRootManifest) {
    let after = media.counters();
    synchronized.0.observation.complete(before, after);
    let manifest = synchronized.0.manifest.clone();
    let worth_store_physical_format::RecordArtifactFile::CatalogCandidate { publication } =
        synchronized.0.candidate
    else {
        unreachable!("publication plans always own one catalog candidate")
    };
    let published = PublishedRecordBatch::from_publication(
        synchronized.0.records,
        synchronized.0.generation,
        publication,
        synchronized.0.observation,
        synchronized.0.work,
        before,
        after,
    );
    (published, manifest)
}
