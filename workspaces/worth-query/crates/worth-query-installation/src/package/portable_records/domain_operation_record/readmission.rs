//! Fresh semantic re-entry for one authority-free domain-operation record.

use worth_query_declaration::facade::canonicalization::{
    validate_portable_query_bundle_freshly_with_work,
    WorthQueryPortableCanonicalQueryReadmissionLimits,
};

use crate::domain_operation::{
    WorthQueryDomainOperationSemanticClosure, WorthQueryPortableDomainOperationDefinition,
};
use crate::package::WorthQueryPortablePackageReconstructionDenial as Denial;

use super::{
    WorthQueryPortableDomainOperationRecord, WorthQueryPortableDomainOperationSemanticRecord,
};

pub(crate) fn readmit_portable_domain_operation(
    record: WorthQueryPortableDomainOperationRecord,
    limits: WorthQueryPortableCanonicalQueryReadmissionLimits,
    maximum_canonical_work_bytes: u64,
) -> Result<(WorthQueryPortableDomainOperationDefinition, u64), Denial> {
    let WorthQueryPortableDomainOperationRecord {
        identity,
        semantics,
        canonical_identity: stored_identity,
    } = record;
    let operation_slot = identity.slot();
    let WorthQueryPortableDomainOperationSemanticRecord {
        parameters,
        native_projection,
        canonical_query,
        collection,
        required_capabilities,
        required_domains,
        workflow,
        evidence,
        conditional_nodes,
        graph_reads,
        decision_facts,
        touches,
        effects,
        invariants,
        invariant_execution,
        replay,
        lineage,
        promotion,
        publication,
        projection_consumption,
        terminal,
        cost,
        resources,
        support,
        lowering,
    } = semantics;
    let canonical_query_readmission =
        validate_portable_query_bundle_freshly_with_work(canonical_query, limits).map_err(
            |denial| Denial::CanonicalQueryReadmissionDenied {
                operation_slot: operation_slot.clone(),
                denial,
            },
        )?;
    let query_work_bytes = canonical_query_readmission.logical_work_bytes();
    let canonical_query = canonical_query_readmission.into_bundle();
    let semantics = WorthQueryDomainOperationSemanticClosure {
        parameters,
        native_projection,
        canonical_query,
        collection,
        required_capabilities,
        required_domains,
        workflow,
        evidence,
        conditional_nodes,
        graph_reads,
        decision_facts,
        touches,
        effects,
        invariants,
        invariant_execution,
        replay,
        aftermath: None,
        lineage,
        promotion,
        publication,
        projection_consumption,
        terminal,
        cost,
        resources,
        support,
        lowering,
    };
    let operation_work_bytes =
        crate::domain_operation::canonical_operation_encoded_bytes(&identity, &semantics);
    let total_work_bytes = query_work_bytes
        .checked_add(operation_work_bytes)
        .ok_or(Denial::WorkObservationOverflow)?;
    if total_work_bytes > maximum_canonical_work_bytes {
        return Err(Denial::CanonicalWorkBudgetExceeded {
            observed: total_work_bytes,
            maximum: maximum_canonical_work_bytes,
        });
    }
    let reconstructed =
        WorthQueryPortableDomainOperationDefinition::reconstruct_exact(identity, semantics)
            .map_err(|()| Denial::NonCanonicalDomainOperationSemantics {
                operation_slot: operation_slot.clone(),
            })?;
    if reconstructed.canonical_identity() != stored_identity {
        return Err(Denial::DomainOperationIdentityMismatch { operation_slot });
    }
    Ok((reconstructed, total_work_bytes))
}
