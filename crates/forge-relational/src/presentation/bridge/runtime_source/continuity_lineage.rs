use std::sync::Arc;

use forge_runtime_bridge::facade::{
    BridgeHistoricalLineageAuthority, BridgeHistoricalLineageRequest, BridgeLineageSourceError,
    BridgeLineageSourceErrorKind, ContinuityLineageSource,
};

use super::snapshot_authority::resolve_snapshot_version;
use super::RuntimeBridgeRelationalSource;
use crate::lineage::data::{HistoricalResolutionBoundednessBasis, RecordHistoryRequest};
use crate::presentation::bridge::identities::{parse_bridge_record_identity, record_ref_identity};
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
                .as_str()
                .to_string(),
        );
        let record = parse_bridge_record_identity(request.prior_slice().entity_identity())
            .map_err(|error| {
                BridgeLineageSourceError::new(
                    BridgeLineageSourceErrorKind::HistoricalResolutionFailure,
                    error.to_string(),
                )
            })?;
        let RecordRef::Entity(entity_id) = record else {
            return Err(BridgeLineageSourceError::new(
                BridgeLineageSourceErrorKind::UnsupportedContinuityClass,
                "bridge continuity lineage adapter currently supports entity record identities only",
            ));
        };
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
                        request.prior_slice().entity_identity()
                    ),
                )
            })?;
        let mut canonical_resolved_lineage_keys = resolution
            .resolved
            .iter()
            .map(|lineage| Arc::<str>::from(format!("lineage:{}", lineage.0)))
            .collect::<Vec<_>>();
        canonical_resolved_lineage_keys.sort_unstable();
        canonical_resolved_lineage_keys.dedup();

        let snapshot_version_id =
            resolve_snapshot_version(&self.runtime, request.authority_basis().snapshot_identity())
                .map_err(|error| {
                    BridgeLineageSourceError::new(
                        BridgeLineageSourceErrorKind::HistoricalResolutionFailure,
                        error.to_string(),
                    )
                })?;
        let mut canonical_resolved_record_keys = self
            .runtime
            .lineage_access()
            .visible_entity_ids_for_lineages_at_version(&resolution.resolved, snapshot_version_id)
            .into_iter()
            .map(|entity_id| Arc::<str>::from(record_ref_identity(&RecordRef::Entity(entity_id))))
            .collect::<Vec<_>>();
        canonical_resolved_record_keys.sort_unstable();
        canonical_resolved_record_keys.dedup();

        let mut traversed_event_ids = resolution.traversed_event_ids.clone();
        traversed_event_ids.sort_unstable();
        traversed_event_ids.dedup();

        BridgeHistoricalLineageAuthority::try_new(
            request.authority_basis().clone(),
            canonical_resolved_lineage_keys,
            canonical_resolved_record_keys,
            traversed_event_ids,
        )
    }
}
