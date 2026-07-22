use worth_store_physical_backend::{MediaCounterSnapshot, QualifiedFilesystemMedia};
use worth_store_physical_format::DurablePhysicalRootManifest;

use super::super::publication::{
    classify_catalog_replacement_failure, indeterminate, unpublished_backend,
    unpublished_candidate_frame_contract, unpublished_residency, unpublished_semantic,
    unpublished_stream, write_candidate_data, CandidateDataArtifact, CandidateDataWriteFailure,
    PublicationPlan, RecordPublicationStage,
};
use super::super::residency::publication_artifacts::PublicationRecordArtifacts;
use super::super::UnpublishedRecordEffectFate;
use super::super::{AdmittedPhysicalRecordFormat, PublishedRecordBatch, RecordAppendError};

struct CandidateDataWritten(PublicationPlan);
struct DataSynchronized(PublicationPlan);
struct ManifestsSynchronized(PublicationPlan);
struct CatalogCandidateSynchronized(PublicationPlan);
struct CatalogReplaced(PublicationPlan);
struct NamespaceSynchronized(PublicationPlan);

pub(in crate::physical_runtime::record_serving) fn execute(
    media: &QualifiedFilesystemMedia,
    format: AdmittedPhysicalRecordFormat,
    plan: PublicationPlan,
    residency: &mut super::super::residency::frame_ports::StoreCandidateFramePublicationSession,
    counters_before: MediaCounterSnapshot,
) -> Result<(PublishedRecordBatch, DurablePhysicalRootManifest), RecordAppendError> {
    let artifacts = PublicationRecordArtifacts::new(media);
    let candidate_data =
        write_all_candidate_data(media, &artifacts, format, plan, residency, counters_before)?;
    let data_synchronized =
        synchronize_candidate_data(media, &artifacts, candidate_data, counters_before)?;
    let manifests_synchronized = synchronize_manifests(
        media,
        &artifacts,
        data_synchronized,
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
    super::catalog_cutover_preflight::validate_frame_set(
        media,
        &candidate_synchronized.0,
        residency,
        counters_before,
    )?;
    super::catalog_cutover_preflight::prepare_residency(
        media,
        &candidate_synchronized.0,
        residency,
        counters_before,
    )?;
    let catalog_replaced =
        replace_catalog(media, &artifacts, candidate_synchronized, counters_before)?;
    let namespace_synchronized =
        synchronize_namespace(media, &artifacts, catalog_replaced, counters_before)?;
    Ok(complete_publication(
        media,
        namespace_synchronized,
        counters_before,
    ))
}

fn write_all_candidate_data(
    media: &QualifiedFilesystemMedia,
    artifacts: &PublicationRecordArtifacts<'_>,
    format: AdmittedPhysicalRecordFormat,
    mut plan: PublicationPlan,
    residency: &mut super::super::residency::frame_ports::StoreCandidateFramePublicationSession,
    before: MediaCounterSnapshot,
) -> Result<CandidateDataWritten, RecordAppendError> {
    let mut prior_effect_fate = UnpublishedRecordEffectFate::DeniedBeforeEffect;
    for data in &mut plan.data {
        if let Err(failure) =
            write_candidate_data(artifacts, format, data, residency, &mut plan.observation)
        {
            return Err(match failure {
                CandidateDataWriteFailure::PreEffectDenied(denial) => {
                    if prior_effect_fate == UnpublishedRecordEffectFate::DeniedBeforeEffect {
                        RecordAppendError::Denied(denial)
                    } else {
                        unpublished_semantic(
                            media,
                            &plan,
                            before,
                            RecordPublicationStage::CandidateDataWrite,
                            denial,
                        )
                    }
                }
                CandidateDataWriteFailure::Semantic(denial) => unpublished_semantic(
                    media,
                    &plan,
                    before,
                    RecordPublicationStage::CandidateDataWrite,
                    denial,
                ),
                CandidateDataWriteFailure::Residency(denial) => unpublished_residency(
                    media,
                    &plan,
                    before,
                    RecordPublicationStage::CandidateDataWrite,
                    denial,
                ),
                CandidateDataWriteFailure::Stream(failure) => {
                    unpublished_stream(media, &plan, before, failure)
                }
                CandidateDataWriteFailure::Backend {
                    failure,
                    effect_fate,
                } => unpublished_backend(
                    media,
                    &plan,
                    before,
                    RecordPublicationStage::CandidateDataWrite,
                    failure,
                    prior_effect_fate.combine(effect_fate),
                ),
                CandidateDataWriteFailure::CandidateFrameContract(violation) => {
                    unpublished_candidate_frame_contract(
                        media,
                        &plan,
                        before,
                        RecordPublicationStage::CandidateDataWrite,
                        violation,
                    )
                }
            });
        }
        prior_effect_fate = UnpublishedRecordEffectFate::EffectPossible;
    }
    Ok(CandidateDataWritten(plan))
}

fn synchronize_candidate_data(
    media: &QualifiedFilesystemMedia,
    artifacts: &PublicationRecordArtifacts<'_>,
    written: CandidateDataWritten,
    before: MediaCounterSnapshot,
) -> Result<DataSynchronized, RecordAppendError> {
    for data in &written.0.data {
        let artifact = match data {
            CandidateDataArtifact::Segment(value) => value.artifact,
            CandidateDataArtifact::Extent(value) => value.artifact,
        };
        artifacts
            .synchronize_artifact(artifact)
            .and_then(|()| artifacts.synchronize_artifact_parent(artifact))
            .map_err(|failure| {
                unpublished_backend(
                    media,
                    &written.0,
                    before,
                    RecordPublicationStage::DataSynchronization,
                    failure,
                    UnpublishedRecordEffectFate::EffectPossible,
                )
            })?;
    }
    Ok(DataSynchronized(written.0))
}

fn synchronize_manifests(
    media: &QualifiedFilesystemMedia,
    artifacts: &PublicationRecordArtifacts<'_>,
    mut synchronized: DataSynchronized,
    residency: &mut super::super::residency::frame_ports::StoreCandidateFramePublicationSession,
    before: MediaCounterSnapshot,
) -> Result<ManifestsSynchronized, RecordAppendError> {
    for index in 0..synchronized.0.manifests.len() {
        let (artifact, bytes) = &mut synchronized.0.manifests[index];
        let artifact = *artifact;
        let resident = residency.write_frame(
            super::super::residency::frame_ports::CandidateFrame::new(
                super::super::residency::frame_ports::CandidateFrameRole::ManifestBlock,
                super::super::residency::frame_ports::CandidateFrameCoordinate::new(artifact, 0),
                std::mem::take(bytes),
            ),
            &mut |bytes| {
                let physical = artifacts.write_new_frame(artifact, bytes)?;
                artifacts.synchronize_artifact(artifact)?;
                artifacts.synchronize_artifact_parent(artifact)?;
                Ok(physical)
            },
        );
        let resident = match resident {
            Ok(resident) => resident,
            Err(super::super::residency::frame_ports::CandidateFrameWriteFailure::Backend(
                failure,
            )) => {
                return Err(unpublished_backend(
                    media,
                    &synchronized.0,
                    before,
                    RecordPublicationStage::ManifestSynchronization,
                    failure,
                    UnpublishedRecordEffectFate::EffectPossible,
                ));
            }
            Err(super::super::residency::frame_ports::CandidateFrameWriteFailure::Contract(
                violation,
            )) => {
                return Err(unpublished_candidate_frame_contract(
                    media,
                    &synchronized.0,
                    before,
                    RecordPublicationStage::ManifestSynchronization,
                    violation,
                ));
            }
            Err(super::super::residency::frame_ports::CandidateFrameWriteFailure::Residency(
                denial,
            )) => {
                return Err(unpublished_residency(
                    media,
                    &synchronized.0,
                    before,
                    RecordPublicationStage::ManifestSynchronization,
                    denial,
                ));
            }
        };
        synchronized
            .0
            .observation
            .observe_transfer(resident.frame_bytes() as usize);
    }
    let root = synchronized.0.root;
    let resident_root = residency
        .write_frame(
            super::super::residency::frame_ports::CandidateFrame::new(
                super::super::residency::frame_ports::CandidateFrameRole::RootManifest,
                super::super::residency::frame_ports::CandidateFrameCoordinate::new(root, 0),
                std::mem::take(&mut synchronized.0.root_bytes),
            ),
            &mut |bytes| {
                let physical = artifacts.write_new_frame(root, bytes)?;
                artifacts.synchronize_artifact(root)?;
                artifacts.synchronize_artifact_parent(root)?;
                Ok(physical)
            },
        )
        .map_err(|failure| match failure {
            super::super::residency::frame_ports::CandidateFrameWriteFailure::Backend(failure) => {
                unpublished_backend(
                    media,
                    &synchronized.0,
                    before,
                    RecordPublicationStage::ManifestSynchronization,
                    failure,
                    UnpublishedRecordEffectFate::EffectPossible,
                )
            }
            super::super::residency::frame_ports::CandidateFrameWriteFailure::Contract(
                violation,
            ) => unpublished_candidate_frame_contract(
                media,
                &synchronized.0,
                before,
                RecordPublicationStage::ManifestSynchronization,
                violation,
            ),
            super::super::residency::frame_ports::CandidateFrameWriteFailure::Residency(denial) => {
                unpublished_residency(
                    media,
                    &synchronized.0,
                    before,
                    RecordPublicationStage::ManifestSynchronization,
                    denial,
                )
            }
        })?;
    synchronized
        .0
        .observation
        .observe_transfer(resident_root.frame_bytes() as usize);
    Ok(ManifestsSynchronized(synchronized.0))
}

fn synchronize_catalog_candidate(
    media: &QualifiedFilesystemMedia,
    artifacts: &PublicationRecordArtifacts<'_>,
    mut synchronized: ManifestsSynchronized,
    residency: &mut super::super::residency::frame_ports::StoreCandidateFramePublicationSession,
    before: MediaCounterSnapshot,
) -> Result<CatalogCandidateSynchronized, RecordAppendError> {
    let candidate = synchronized.0.candidate;
    let resident_catalog = residency
        .write_frame(
            super::super::residency::frame_ports::CandidateFrame::new(
                super::super::residency::frame_ports::CandidateFrameRole::CatalogCandidate,
                super::super::residency::frame_ports::CandidateFrameCoordinate::new(candidate, 0),
                std::mem::take(&mut synchronized.0.catalog_bytes),
            ),
            &mut |bytes| {
                let physical = artifacts.write_new_frame(candidate, bytes)?;
                artifacts.synchronize_artifact(candidate)?;
                Ok(physical)
            },
        )
        .map_err(|failure| match failure {
            super::super::residency::frame_ports::CandidateFrameWriteFailure::Backend(failure) => {
                unpublished_backend(
                    media,
                    &synchronized.0,
                    before,
                    RecordPublicationStage::CatalogCandidateSynchronization,
                    failure,
                    UnpublishedRecordEffectFate::EffectPossible,
                )
            }
            super::super::residency::frame_ports::CandidateFrameWriteFailure::Contract(
                violation,
            ) => unpublished_candidate_frame_contract(
                media,
                &synchronized.0,
                before,
                RecordPublicationStage::CatalogCandidateSynchronization,
                violation,
            ),
            super::super::residency::frame_ports::CandidateFrameWriteFailure::Residency(denial) => {
                unpublished_residency(
                    media,
                    &synchronized.0,
                    before,
                    RecordPublicationStage::CatalogCandidateSynchronization,
                    denial,
                )
            }
        })?;
    synchronized
        .0
        .observation
        .observe_transfer(resident_catalog.frame_bytes() as usize);
    Ok(CatalogCandidateSynchronized(synchronized.0))
}

fn replace_catalog(
    media: &QualifiedFilesystemMedia,
    artifacts: &PublicationRecordArtifacts<'_>,
    synchronized: CatalogCandidateSynchronized,
    before: MediaCounterSnapshot,
) -> Result<CatalogReplaced, RecordAppendError> {
    artifacts
        .replace_catalog(synchronized.0.candidate)
        .map_err(|failure| {
            classify_catalog_replacement_failure(media, &synchronized.0, before, failure)
        })?;
    Ok(CatalogReplaced(synchronized.0))
}

fn synchronize_namespace(
    media: &QualifiedFilesystemMedia,
    artifacts: &PublicationRecordArtifacts<'_>,
    replaced: CatalogReplaced,
    before: MediaCounterSnapshot,
) -> Result<NamespaceSynchronized, RecordAppendError> {
    artifacts.synchronize_record_family().map_err(|failure| {
        indeterminate(
            media,
            &replaced.0,
            before,
            RecordPublicationStage::NamespaceSynchronization,
            failure,
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
        before,
        after,
    );
    (published, manifest)
}
