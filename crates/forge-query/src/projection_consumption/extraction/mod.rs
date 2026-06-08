mod aspect_value_projection;
mod grouped;
mod query_context;
mod row_like;

use super::consumed::{
    ConsumedEffectContinuityFact, ConsumedProjectionFactSet, ConsumedRelationEndpointFact,
    ConsumedSourceReferenceFact, ConsumedTargetIdentityFact, ProjectionFactExtractionCounters,
};
use super::contracts::MaterializedProjectionContract;
use super::facts::ProjectionFactKind;
use super::source::ProjectionSourceFamily;
use crate::query_context::QueryContextExecutionArtifact;
use crate::runtime::{ForgeQueryReadResult, ForgeQueryWriteReceipt};
use forge_relational::facade::grouped_truth::{
    RelationalAuthoritativeRowSetArtifact, RelationalGroupedProjectionArtifact,
};
use forge_runtime_bridge::facade::{
    BridgeGroupedTruthViewArtifact, BridgeMaterializedRowSetArtifact,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProjectionFactExtractionError {
    ContractSourceFamilyMismatch {
        contract_family: ProjectionSourceFamily,
        extractor_family: ProjectionSourceFamily,
    },
    SourceIdentityMismatch {
        contract_source_identity: String,
        provided_source_identity: String,
    },
    SourceArtifactMetadataMismatch {
        metadata_label: &'static str,
        contract_value: String,
        provided_value: String,
    },
    MissingDeclaredFieldEvidence {
        source_family: ProjectionSourceFamily,
        source_identity: String,
        field_key: String,
        fact_kind: ProjectionFactKind,
    },
    InvalidDeclaredFieldValueShape {
        source_family: ProjectionSourceFamily,
        source_identity: String,
        field_key: String,
        fact_kind: ProjectionFactKind,
        expected_shape: &'static str,
    },
    MissingWriteReceiptEvidence {
        source_identity: String,
        fact_kind: ProjectionFactKind,
    },
    SourceReferenceEvidenceMismatch {
        expected_count: usize,
        actual_count: usize,
    },
}

impl MaterializedProjectionContract {
    pub fn extract_from_read_result(
        &self,
        result: &ForgeQueryReadResult,
    ) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> {
        ensure_optional_metadata(
            "query_digest",
            self.query_digest(),
            Some(result.receipt().query_digest()),
        )?;
        ensure_optional_metadata(
            "basis_digest",
            self.basis_digest(),
            Some(result.receipt().basis_digest()),
        )?;
        ensure_optional_metadata(
            "result_digest",
            self.result_digest(),
            Some(result.receipt().result_digest()),
        )?;
        row_like::extract_read_result_facts(self, result)
    }

    pub fn extract_from_relational_row_set(
        &self,
        row_set: &RelationalAuthoritativeRowSetArtifact,
    ) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> {
        row_like::extract_relational_row_set_facts(self, row_set)
    }

    pub fn extract_from_bridge_truth_view_row_set(
        &self,
        row_set: &BridgeMaterializedRowSetArtifact,
    ) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> {
        row_like::extract_bridge_row_set_facts(self, row_set)
    }

    pub fn extract_from_relational_grouped_projection(
        &self,
        projection: &RelationalGroupedProjectionArtifact,
    ) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> {
        grouped::extract_relational_grouped_facts(self, projection)
    }

    pub fn extract_from_bridge_grouped_truth_view(
        &self,
        grouped_truth_view: &BridgeGroupedTruthViewArtifact,
    ) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> {
        grouped::extract_bridge_grouped_facts(self, grouped_truth_view)
    }

    pub fn extract_from_query_context_execution(
        &self,
        execution: &QueryContextExecutionArtifact,
    ) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> {
        ensure_optional_metadata(
            "query_digest",
            self.query_digest(),
            Some(execution.query_digest()),
        )?;
        ensure_optional_metadata(
            "basis_digest",
            self.basis_digest(),
            Some(execution.basis_digest()),
        )?;
        ensure_optional_metadata(
            "result_digest",
            self.result_digest(),
            Some(execution.result_digest()),
        )?;
        ensure_optional_metadata(
            "result_shape_digest",
            Some(self.canonical_result_shape_digest()),
            Some(execution.result_shape_digest()),
        )?;
        query_context::extract_query_context_facts(self, execution)
    }

    pub fn extract_from_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
    ) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> {
        ensure_contract_family(self, ProjectionSourceFamily::QueryWriteReceipt)?;
        ensure_source_identity(self.source_identity(), receipt.commit_identity())?;

        let mut target_identities = Vec::new();
        let mut source_references = Vec::new();
        let mut effect_continuity_facts = Vec::new();
        let mut relation_endpoints = Vec::new();
        let mut evidence_lookup_width = 0;

        for fact_family in self.fact_families() {
            match fact_family.kind() {
                ProjectionFactKind::TargetIdentity => {
                    evidence_lookup_width += 1;
                    let Some(identity) = receipt.target_entity_identity() else {
                        return Err(ProjectionFactExtractionError::MissingWriteReceiptEvidence {
                            source_identity: receipt.commit_identity().to_string(),
                            fact_kind: ProjectionFactKind::TargetIdentity,
                        });
                    };
                    target_identities.push(ConsumedTargetIdentityFact::new(identity));
                }
                ProjectionFactKind::SourceReference => {
                    evidence_lookup_width += 1;
                    source_references = write_receipt_source_references(receipt);
                }
                ProjectionFactKind::EffectContinuity => {
                    evidence_lookup_width += 1;
                    let Some(continuity) = receipt.continuity_mutation_evidence() else {
                        return Err(ProjectionFactExtractionError::MissingWriteReceiptEvidence {
                            source_identity: receipt.commit_identity().to_string(),
                            fact_kind: ProjectionFactKind::EffectContinuity,
                        });
                    };
                    effect_continuity_facts.push(ConsumedEffectContinuityFact::new(
                        continuity.family(),
                        continuity.outcome_class(),
                        continuity.prior_authoritative_identity(),
                        continuity.successor_authoritative_identities().to_vec(),
                        continuity
                            .resolved_target_entity_identity()
                            .map(str::to_string),
                        continuity.target_collection().map(str::to_string),
                        continuity.lineage_digest(),
                        continuity.continuity_resolution_digest(),
                    ));
                }
                ProjectionFactKind::RelationEndpoint => {
                    evidence_lookup_width += 1;
                    let resolved = receipt.target_evidence().resolved();
                    if resolved.collection().is_none() || resolved.entity_identity().is_none() {
                        return Err(ProjectionFactExtractionError::MissingWriteReceiptEvidence {
                            source_identity: receipt.commit_identity().to_string(),
                            fact_kind: ProjectionFactKind::RelationEndpoint,
                        });
                    }
                    relation_endpoints.push(ConsumedRelationEndpointFact::new(
                        resolved.target_class(),
                        resolved.collection().map(str::to_string),
                        resolved.entity_identity().map(str::to_string),
                    ));
                }
                ProjectionFactKind::EntityIdentity
                | ProjectionFactKind::ViewLocalIdentity
                | ProjectionFactKind::Membership
                | ProjectionFactKind::DisplayField
                | ProjectionFactKind::DerivedScalarField => {}
            }
        }

        if !source_reference_inventory_matches(self, &source_references) {
            return Err(
                ProjectionFactExtractionError::SourceReferenceEvidenceMismatch {
                    expected_count: self.source_reference_identities().len(),
                    actual_count: source_references.len(),
                },
            );
        }

        Ok(ConsumedProjectionFactSet::new(
            self.declaration_digest(),
            self.contract_digest(),
            self.source_family(),
            self.source_identity(),
            self.support_posture().clone(),
            self.materialized_fact_posture().cloned(),
            ProjectionFactExtractionCounters::new(
                self.fact_families().len(),
                self.fact_families().len(),
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
}

fn ensure_optional_metadata(
    metadata_label: &'static str,
    contract_value: Option<&str>,
    provided_value: Option<&str>,
) -> Result<(), ProjectionFactExtractionError> {
    match (contract_value, provided_value) {
        (Some(contract_value), Some(provided_value)) if contract_value == provided_value => Ok(()),
        (Some(contract_value), Some(provided_value)) => Err(
            ProjectionFactExtractionError::SourceArtifactMetadataMismatch {
                metadata_label,
                contract_value: contract_value.to_string(),
                provided_value: provided_value.to_string(),
            },
        ),
        (None, None) => Ok(()),
        (Some(contract_value), None) => Err(
            ProjectionFactExtractionError::SourceArtifactMetadataMismatch {
                metadata_label,
                contract_value: contract_value.to_string(),
                provided_value: "<missing>".to_string(),
            },
        ),
        (None, Some(provided_value)) => Err(
            ProjectionFactExtractionError::SourceArtifactMetadataMismatch {
                metadata_label,
                contract_value: "<absent>".to_string(),
                provided_value: provided_value.to_string(),
            },
        ),
    }
}

pub(super) fn ensure_contract_family(
    contract: &MaterializedProjectionContract,
    extractor_family: ProjectionSourceFamily,
) -> Result<(), ProjectionFactExtractionError> {
    if contract.source_family() == extractor_family {
        Ok(())
    } else {
        Err(
            ProjectionFactExtractionError::ContractSourceFamilyMismatch {
                contract_family: contract.source_family(),
                extractor_family,
            },
        )
    }
}

pub(super) fn ensure_source_identity(
    contract_source_identity: &str,
    provided_source_identity: &str,
) -> Result<(), ProjectionFactExtractionError> {
    if contract_source_identity == provided_source_identity {
        Ok(())
    } else {
        Err(ProjectionFactExtractionError::SourceIdentityMismatch {
            contract_source_identity: contract_source_identity.to_string(),
            provided_source_identity: provided_source_identity.to_string(),
        })
    }
}

fn write_receipt_source_references(
    receipt: &ForgeQueryWriteReceipt,
) -> Vec<ConsumedSourceReferenceFact> {
    let mut references = Vec::new();
    if let Some(provenance) = receipt.provenance_evidence() {
        references.push(ConsumedSourceReferenceFact::new(
            "bridge_provenance_execution_record",
            provenance.execution_record_digest(),
        ));
    }
    if let Some(symbolic) = receipt.symbolic_target_reference_evidence() {
        references.push(ConsumedSourceReferenceFact::new(
            "symbolic_target_reference",
            symbolic.symbol(),
        ));
    }
    references
}

fn source_reference_inventory_matches(
    contract: &MaterializedProjectionContract,
    actual: &[ConsumedSourceReferenceFact],
) -> bool {
    contract.source_reference_identities().len() == actual.len()
        && contract
            .source_reference_identities()
            .iter()
            .zip(actual.iter())
            .all(|(expected, actual)| {
                expected.label() == actual.label() && expected.identity() == actual.identity()
            })
}
