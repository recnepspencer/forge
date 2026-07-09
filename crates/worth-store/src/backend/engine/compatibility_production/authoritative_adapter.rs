use crate::authority::AuthoritativeExportBundle;
use crate::compatibility::{
    admit_authoritative_meaning_with_parity_witness, check_artifact_with_read_receipt,
    declare_authoritative_meaning, execute_declared_adapter_parity,
    first_ship_authoritative_adapter_edge_registry, first_ship_commit_envelope_adapted_lane,
    first_ship_commit_envelope_control_lane, plan_read_compatibility_for_path, AdapterParityLane,
    CompatibilityAdmissionBatch, CompatibilityAdmissionPath,
    CompatibilityAuthoritativeAdapterOutcome, CompatibilityAuthoritativeAdapterRequest,
    CompatibilityFamilyKind, CompatibilityReadIntent, CompatibilityRelation, ReaderCapabilitySet,
};
use crate::failure::{StoreError, StoreErrorKind};
use sha2::{Digest, Sha256};

use super::super::{
    compatibility_runtime::compatibility_rejection_error, StateBackedStoreBackend, StatePersistence,
};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub(crate) fn execute_compatibility_authoritative_adapter(
        &self,
        request: CompatibilityAuthoritativeAdapterRequest,
    ) -> Result<CompatibilityAuthoritativeAdapterOutcome, StoreError> {
        let artifact = self.runtime_compatibility_artifact(
            request.family_kind(),
            "execute_compatibility_authoritative_adapter",
        )?;
        if artifact.semantic_version() != request.observed_semantic_version() {
            return Err(StoreError::new(
                StoreErrorKind::CompatibilityArtifactSemanticVersionUnsupported,
                format!(
                    "compatibility authoritative adapter expected observed semantic version `{}` for family `{}`, got `{}`",
                    request.observed_semantic_version().value(),
                    request.family_kind().label(),
                    artifact.semantic_version().value()
                ),
            ));
        }

        let family_id = artifact.family_id().clone();
        let manifest_index = self.runtime_compatibility_manifest_index();
        let edge_registry = first_ship_authoritative_adapter_edge_registry();
        let reader =
            ReaderCapabilitySet::new(family_id.clone(), vec![request.target_semantic_version()]);
        let intent =
            CompatibilityReadIntent::new(family_id.clone(), request.target_semantic_version());
        let mut batch = CompatibilityAdmissionBatch::new();
        let read_receipt = plan_read_compatibility_for_path(
            &mut batch,
            &manifest_index,
            &edge_registry,
            &reader,
            &intent,
            &artifact,
            CompatibilityAdmissionPath::BatchRead,
        )
        .map_err(|rejection| {
            compatibility_rejection_error(
                "execute_compatibility_authoritative_adapter.read",
                rejection,
            )
        })?;
        if read_receipt.receipt().relation() != CompatibilityRelation::AdapterRequired {
            return Err(StoreError::new(
                StoreErrorKind::CompatibilityAdapterParityFailure,
                format!(
                    "compatibility authoritative adapter requires an adapter-required relation for family `{}`",
                    request.family_kind().label()
                ),
            ));
        }

        let checked_artifact =
            check_artifact_with_read_receipt(artifact, &read_receipt).map_err(|rejection| {
                compatibility_rejection_error(
                    "execute_compatibility_authoritative_adapter.checked_artifact",
                    rejection,
                )
            })?;
        let (control_lane, adapted_lane) =
            authoritative_family_parity_lanes(&self.export_bundle(), request.family_kind())?;
        let parity_witness = execute_declared_adapter_parity(
            batch.counters_mut(),
            &edge_registry,
            &family_id,
            request.observed_semantic_version(),
            request.target_semantic_version(),
            request.adapter_id(),
            request.adapter_digest(),
            control_lane.bytes(),
            adapted_lane.bytes(),
            adapted_lane.input_record_count(),
            adapted_lane.output_record_count(),
            adapted_lane.allocation_scope_count(),
        )
        .map_err(|rejection| {
            compatibility_rejection_error(
                "execute_compatibility_authoritative_adapter.parity",
                rejection,
            )
        })?;
        let meaning = declare_authoritative_meaning(
            family_id.clone(),
            request.target_semantic_version(),
            format!(
                "compatibility-authoritative-adapter:{}:{}->{}",
                request.family_kind().label(),
                request.observed_semantic_version().value(),
                request.target_semantic_version().value()
            ),
        );
        admit_authoritative_meaning_with_parity_witness(
            batch.counters_mut(),
            &checked_artifact,
            &read_receipt,
            Some(&meaning),
            Some(&parity_witness),
        )
        .map_err(|rejection| {
            compatibility_rejection_error(
                "execute_compatibility_authoritative_adapter.authoritative_meaning",
                rejection,
            )
        })?;

        let control_lane_digest = sha256_hex(control_lane.bytes());
        let adapted_lane_digest = sha256_hex(adapted_lane.bytes());
        Ok(CompatibilityAuthoritativeAdapterOutcome::new(
            request.family_kind(),
            read_receipt.receipt().relation(),
            control_lane_digest.clone(),
            adapted_lane_digest,
            parity_witness,
            crate::Milestone12AdmissionReport::from_admission_counters(batch.counters()),
        ))
    }
}

fn authoritative_family_parity_lanes(
    export: &AuthoritativeExportBundle,
    family_kind: CompatibilityFamilyKind,
) -> Result<(AdapterParityLane, AdapterParityLane), StoreError> {
    match family_kind {
        CompatibilityFamilyKind::CommitEnvelope => Ok((
            first_ship_commit_envelope_control_lane(export)?,
            first_ship_commit_envelope_adapted_lane(export)?,
        )),
        _ => Err(StoreError::new(
            StoreErrorKind::CompatibilityEdgeMissing,
            format!(
                "bounded first-ship authoritative adapter path does not declare family `{}`",
                family_kind.label()
            ),
        )),
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
