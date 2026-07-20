use super::super::consumed::{
    ConsumedContinuityAuthorityIdentity, ConsumedEffectContinuityFact, ConsumedProjectionFactSet,
    ConsumedRelationEndpointFact, ConsumedSourceReferenceFact, ConsumedTargetIdentityFact,
    ProjectionFactExtractionCounters,
};
use super::super::contracts::MaterializedProjectionContract;
use super::super::facts::ProjectionFactKind;
use super::super::source::ProjectionSourceFamily;
use crate::projection_consumption::ProjectionFactExtractionError;
use crate::runtime::WorthQueryWriteReceipt;

pub(super) fn extract_write_receipt_facts(
    contract: &MaterializedProjectionContract,
    receipt: &WorthQueryWriteReceipt,
) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> {
    super::ensure_contract_family(contract, ProjectionSourceFamily::QueryWriteReceipt)?;
    let receipt_commit_identity = receipt.commit_evidence_identity();
    super::ensure_source_identity(contract.source_identity(), receipt_commit_identity.as_str())?;
    let receipt_source_identity = receipt_commit_identity.as_str().to_string();

    let mut target_identities = Vec::new();
    let mut source_references = Vec::new();
    let mut effect_continuity_facts = Vec::new();
    let mut relation_endpoints = Vec::new();
    let mut evidence_lookup_width = 0;

    for fact_family in contract.fact_families() {
        match fact_family.kind() {
            ProjectionFactKind::TargetIdentity => {
                evidence_lookup_width += 1;
                let Some(identity) = receipt.target_entity_identity() else {
                    return Err(ProjectionFactExtractionError::MissingWriteReceiptEvidence {
                        source_identity: receipt_source_identity.clone(),
                        fact_kind: ProjectionFactKind::TargetIdentity,
                    });
                };
                target_identities.push(ConsumedTargetIdentityFact::new(identity.clone()));
            }
            ProjectionFactKind::SourceReference => {
                evidence_lookup_width += 1;
                source_references = write_receipt_source_references(receipt);
            }
            ProjectionFactKind::EffectContinuity => {
                evidence_lookup_width += 1;
                let Some(continuity) = receipt.continuity_mutation_evidence() else {
                    return Err(ProjectionFactExtractionError::MissingWriteReceiptEvidence {
                        source_identity: receipt_source_identity.clone(),
                        fact_kind: ProjectionFactKind::EffectContinuity,
                    });
                };
                effect_continuity_facts.push(ConsumedEffectContinuityFact::new(
                    continuity.family(),
                    continuity.outcome_class(),
                    ConsumedContinuityAuthorityIdentity::new(
                        continuity.prior_authoritative_identity().as_str(),
                    ),
                    continuity
                        .successor_authoritative_identities()
                        .iter()
                        .map(|identity| ConsumedContinuityAuthorityIdentity::new(identity.as_str()))
                        .collect(),
                    continuity.resolved_target_entity_identity().cloned(),
                    continuity
                        .target_collection()
                        .map(|collection| collection.as_str().to_string()),
                    continuity.lineage_digest().as_str(),
                    continuity.continuity_resolution_digest().as_str(),
                ));
            }
            ProjectionFactKind::RelationEndpoint => {
                evidence_lookup_width += 1;
                let resolved = receipt.target_evidence().resolved();
                if resolved.collection().is_none() || resolved.entity_identity().is_none() {
                    return Err(ProjectionFactExtractionError::MissingWriteReceiptEvidence {
                        source_identity: receipt_source_identity.clone(),
                        fact_kind: ProjectionFactKind::RelationEndpoint,
                    });
                }
                relation_endpoints.push(ConsumedRelationEndpointFact::new(
                    resolved.target_class(),
                    resolved
                        .collection()
                        .map(|collection| collection.as_str().to_string()),
                    resolved.entity_identity().cloned(),
                ));
            }
            ProjectionFactKind::EntityIdentity
            | ProjectionFactKind::ViewLocalIdentity
            | ProjectionFactKind::Membership
            | ProjectionFactKind::DisplayField
            | ProjectionFactKind::DerivedField => {}
        }
    }

    if !super::source_reference_inventory_matches(
        contract.source_reference_identities(),
        &source_references,
    ) {
        return Err(
            ProjectionFactExtractionError::SourceReferenceEvidenceMismatch {
                expected_count: contract.source_reference_identities().len(),
                actual_count: source_references.len(),
            },
        );
    }

    Ok(ConsumedProjectionFactSet::new(
        contract.declaration_digest(),
        contract.contract_digest(),
        contract.source_family(),
        contract.source_identity_handle().clone(),
        contract.support_posture().clone(),
        contract.materialized_fact_posture().cloned(),
        ProjectionFactExtractionCounters::new(
            contract.fact_families().len(),
            contract.fact_families().len(),
            target_identities.len()
                + source_references.len()
                + effect_continuity_facts.len()
                + relation_endpoints.len(),
            0,
            evidence_lookup_width,
        ),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        target_identities,
        source_references,
        effect_continuity_facts,
        relation_endpoints,
    ))
}

fn write_receipt_source_references(
    receipt: &WorthQueryWriteReceipt,
) -> Vec<ConsumedSourceReferenceFact> {
    let mut references = Vec::new();
    if let Some(provenance) = receipt.provenance_evidence() {
        references.push(ConsumedSourceReferenceFact::new(
            "bridge_provenance_execution_record",
            provenance.execution_record_digest().as_str(),
        ));
    }
    if let Some(symbolic) = receipt.symbolic_target_reference_evidence() {
        references.push(ConsumedSourceReferenceFact::new(
            "symbolic_target_reference",
            symbolic.symbol().as_str(),
        ));
    }
    references
}
