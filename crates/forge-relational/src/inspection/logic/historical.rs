use crate::history::data::{AspectFilter, BranchId};
use crate::inspection::data::{
    HistoricalAspectObservation, HistoricalAvailabilityObservation, HistoricalInspectionMode,
    HistoricalOpenResult, HistoricalRecordInspection, HistoricalRecordObservation,
    HistoricalRecordValue, HistoricalSnapshotView, InspectionAccessPath, InspectionAvailability,
    InspectionDegradation, InspectionOrigin, InspectionScope,
};
use crate::transactions::data::RecordRef;
use crate::visibility::cache_state::cached_state_for_version;

use super::access::InspectionAccess;

impl<'runtime> InspectionAccess<'runtime> {
    pub fn open_historical_view(
        &self,
        version_id: crate::identity::data::VersionId,
        mode: HistoricalInspectionMode,
    ) -> HistoricalOpenResult {
        self.runtime.services.instrumentation.count(|counters| {
            counters.inspection_historical_view_opens += 1;
        });
        let direct_available = version_id == self.runtime.current_version_id()
            || cached_state_for_version(self.runtime, version_id).is_some();
        match mode {
            HistoricalInspectionMode::RetainedOnly if !direct_available => HistoricalOpenResult {
                view: None,
                origin: InspectionOrigin::VisibilitySnapshot,
                access_path: InspectionAccessPath::HistoricalRetainedRead,
                availability: InspectionAvailability::UnavailableByRetention,
                degradations: vec![InspectionDegradation::ReconstructionOmittedByMode],
            },
            HistoricalInspectionMode::RetainedOnly
            | HistoricalInspectionMode::AllowCanonicalReconstruction => {
                let read_view = self.runtime.visibility_reads().read_version(version_id);
                let availability = if direct_available {
                    InspectionAvailability::Direct
                } else {
                    InspectionAvailability::Reconstructed
                };
                let access_path = if direct_available {
                    InspectionAccessPath::HistoricalRetainedRead
                } else {
                    InspectionAccessPath::HistoricalReconstructedRead
                };
                HistoricalOpenResult {
                    view: Some(HistoricalSnapshotView {
                        snapshot: read_view.snapshot().clone(),
                        read_view,
                        origin: InspectionOrigin::VisibilitySnapshot,
                        access_path,
                        availability,
                    }),
                    origin: InspectionOrigin::VisibilitySnapshot,
                    access_path,
                    availability,
                    degradations: Vec::new(),
                }
            }
        }
    }

    pub fn inspect_historical_record(
        &self,
        branch_id: &BranchId,
        version_id: crate::identity::data::VersionId,
        target: RecordRef,
        mode: HistoricalInspectionMode,
    ) -> HistoricalRecordInspection {
        let open_result = self.open_historical_view(version_id, mode);
        let record_observation = match (&open_result.view, target) {
            (Some(view), RecordRef::Entity(entity_id)) => HistoricalRecordObservation {
                target: RecordRef::Entity(entity_id),
                version_id,
                value: view
                    .read_view
                    .get_entity(entity_id)
                    .cloned()
                    .map(HistoricalRecordValue::Entity),
                origin: InspectionOrigin::VisibilitySnapshot,
                access_path: open_result.access_path,
                availability: open_result.availability,
            },
            (Some(view), RecordRef::Relation(relation_id)) => HistoricalRecordObservation {
                target: RecordRef::Relation(relation_id),
                version_id,
                value: view
                    .read_view
                    .get_relation(relation_id)
                    .cloned()
                    .map(HistoricalRecordValue::Relation),
                origin: InspectionOrigin::VisibilitySnapshot,
                access_path: open_result.access_path,
                availability: open_result.availability,
            },
            (None, target) => HistoricalRecordObservation {
                target,
                version_id,
                value: None,
                origin: InspectionOrigin::VisibilitySnapshot,
                access_path: open_result.access_path,
                availability: open_result.availability,
            },
        };
        let lineage_resolution_context = match record_observation.target {
            RecordRef::Entity(entity_id) => self.runtime.lineage_access().resolve_record_history(
                crate::facade::lineage::RecordHistoryRequest {
                    branch_id: branch_id.clone(),
                    entity_id,
                    boundedness_basis:
                        crate::facade::lineage::HistoricalResolutionBoundednessBasis::BranchScopedLineageSeed,
                },
            ),
            RecordRef::Relation(_) => None,
        };
        let aspect_history_observation = match record_observation.target {
            RecordRef::Entity(entity_id) => Some(HistoricalAspectObservation {
                query_result: self
                    .runtime
                    .history_access()
                    .entity_aspect_history_with_trace(branch_id, entity_id, None::<&AspectFilter>),
                origin: InspectionOrigin::CanonicalCommitStorage,
                access_path: InspectionAccessPath::CommitIndexRead,
                availability: InspectionAvailability::Direct,
            }),
            RecordRef::Relation(relation_id) => Some(HistoricalAspectObservation {
                query_result: self
                    .runtime
                    .history_access()
                    .relation_aspect_history_with_trace(
                        branch_id,
                        relation_id,
                        None::<&AspectFilter>,
                    ),
                origin: InspectionOrigin::CanonicalCommitStorage,
                access_path: InspectionAccessPath::CommitIndexRead,
                availability: InspectionAvailability::Direct,
            }),
        };
        let structural_identity_evidence = open_result.view.as_ref().and_then(|_| {
            self.structural_identity(
                InspectionScope::Version(version_id),
                record_observation.target.clone(),
            )
        });
        HistoricalRecordInspection {
            branch_id: branch_id.clone(),
            record_observation,
            lineage_resolution_context,
            aspect_history_observation,
            structural_identity_evidence,
            retention_availability_observation: Some(HistoricalAvailabilityObservation {
                version_id,
                availability: open_result.availability,
                retained_directly: open_result.availability == InspectionAvailability::Direct,
            }),
        }
    }
}
