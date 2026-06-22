use forge_runtime_bridge::facade::{
    BridgeHistoricalLineageAuthority, BridgeHistoricalLineageRequest,
    BridgeHistoricalResolvedLineageIdentity, BridgeHistoricalResolvedRecordIdentity,
    BridgeLineageSourceError, BridgeLineageSourceErrorKind, ContinuityLineageSource,
};

use super::snapshot_authority::resolve_snapshot_version;
use super::RuntimeBridgeRelationalSource;
use crate::lineage::data::{HistoricalResolutionBoundednessBasis, RecordHistoryRequest};
use crate::presentation::bridge::identities::{
    record_ref_from_identity_parts, record_ref_identity,
};
use crate::transactions::data::RecordRef;

impl ContinuityLineageSource for RuntimeBridgeRelationalSource {
    fn historical_lineage(
        &self,
        request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        let branch_id = crate::history::data::BranchId(
            request
                .authority_basis()
                .branch_identity()
                .relational_branch_id()
                .ok_or_else(|| {
                    BridgeLineageSourceError::new(
                        BridgeLineageSourceErrorKind::HistoricalResolutionFailure,
                        "unsupported relational bridge branch identity",
                    )
                })?
                .to_string(),
        );
        let record = request
            .prior_slice()
            .relational_record_identity_parts()
            .ok_or_else(|| {
                BridgeLineageSourceError::new(
                    BridgeLineageSourceErrorKind::HistoricalResolutionFailure,
                    "relational bridge continuity requires typed record identity parts",
                )
            })
            .and_then(|parts| {
                record_ref_from_identity_parts(parts).map_err(|error| {
                    BridgeLineageSourceError::new(
                        BridgeLineageSourceErrorKind::HistoricalResolutionFailure,
                        error.to_string(),
                    )
                })
            })?;
        let RecordRef::Entity(entity_id) = record else {
            return Err(BridgeLineageSourceError::new(
                BridgeLineageSourceErrorKind::UnsupportedContinuityClass,
                "bridge continuity lineage adapter currently supports entity record identities only",
            ));
        };
        let entity_label = format!(
            "entity:{}:{}:{}",
            entity_id.partition_id.0, entity_id.local_slot.0, entity_id.generation.0
        );
        let resolution = self
            .runtime
            .lineage_access()
            .resolve_record_history(RecordHistoryRequest {
                branch_id,
                entity_id,
                boundedness_basis: HistoricalResolutionBoundednessBasis::BranchScopedLineageSeed,
            })
            .ok_or_else(|| {
                BridgeLineageSourceError::new(
                    BridgeLineageSourceErrorKind::HistoricalResolutionFailure,
                    format!(
                        "bridge continuity lineage adapter could not resolve record history for `{}`",
                        entity_label
                    ),
                )
            })?;
        let mut canonical_resolved_lineage_identities = resolution
            .resolved
            .iter()
            .map(|lineage| {
                BridgeHistoricalResolvedLineageIdentity::from_relational_lineage_id(lineage.0)
            })
            .collect::<Vec<_>>();
        canonical_resolved_lineage_identities.sort_unstable();
        canonical_resolved_lineage_identities.dedup();

        let snapshot_version_id =
            resolve_snapshot_version(&self.runtime, request.authority_basis().snapshot_identity())
                .map_err(|error| {
                    BridgeLineageSourceError::new(
                        BridgeLineageSourceErrorKind::HistoricalResolutionFailure,
                        error.to_string(),
                    )
                })?;
        let mut canonical_resolved_record_identities = self
            .runtime
            .lineage_access()
            .visible_entity_ids_for_lineages_at_version(&resolution.resolved, snapshot_version_id)
            .into_iter()
            .map(|entity_id| {
                BridgeHistoricalResolvedRecordIdentity::from_relational_record(record_ref_identity(
                    &RecordRef::Entity(entity_id),
                ))
            })
            .collect::<Vec<_>>();
        canonical_resolved_record_identities.sort_unstable();
        canonical_resolved_record_identities.dedup();

        let mut traversed_event_ids = resolution.traversed_event_ids.clone();
        traversed_event_ids.sort_unstable();
        traversed_event_ids.dedup();

        BridgeHistoricalLineageAuthority::try_new(
            request.authority_basis().clone(),
            canonical_resolved_lineage_identities,
            canonical_resolved_record_identities,
            traversed_event_ids,
        )
    }
}
