use serde::Serialize;

use crate::topology_operators::TopologyTouchedGraphBasis;

use super::{
    DerivedTopologyConsumedGraphFacts, DerivedTopologyDiagnosticPosture,
    DerivedTopologyInvalidationPredicate, DerivedTopologyLegalityReceiptPosture,
    DerivedTopologyProductFamilyIdentity, DerivedTopologyQueryReceiptPosture,
    DerivedTopologySpatialEvidencePosture, DerivedTopologySupportPosture,
    DerivedTopologyUpdatePosture,
};
use crate::derived_topology::invalidation_plan::catalog::{
    DerivedInvalidationFamilyCatalogError, DerivedInvalidationFamilyCatalogErrorKind,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedTopologyProductFamilyRecord {
    identity: DerivedTopologyProductFamilyIdentity,
    consumed_graph_facts: DerivedTopologyConsumedGraphFacts,
    invalidation_predicate: DerivedTopologyInvalidationPredicate,
    update_posture: DerivedTopologyUpdatePosture,
    spatial_evidence_posture: DerivedTopologySpatialEvidencePosture,
    query_receipt_posture: DerivedTopologyQueryReceiptPosture,
    legality_receipt_posture: DerivedTopologyLegalityReceiptPosture,
    diagnostic_posture: DerivedTopologyDiagnosticPosture,
    support_posture: DerivedTopologySupportPosture,
    family_digest: String,
}

pub(crate) struct DerivedTopologyProductFamilyRecordInput {
    pub(crate) identity: DerivedTopologyProductFamilyIdentity,
    pub(crate) consumed_graph_facts: Option<DerivedTopologyConsumedGraphFacts>,
    pub(crate) invalidation_predicate: Option<DerivedTopologyInvalidationPredicate>,
    pub(crate) update_posture: Option<DerivedTopologyUpdatePosture>,
    pub(crate) spatial_evidence_posture: Option<DerivedTopologySpatialEvidencePosture>,
    pub(crate) query_receipt_posture: Option<DerivedTopologyQueryReceiptPosture>,
    pub(crate) legality_receipt_posture: Option<DerivedTopologyLegalityReceiptPosture>,
    pub(crate) diagnostic_posture: Option<DerivedTopologyDiagnosticPosture>,
    pub(crate) support_posture: Option<DerivedTopologySupportPosture>,
}

impl DerivedTopologyProductFamilyRecord {
    pub(crate) fn from_input(
        input: DerivedTopologyProductFamilyRecordInput,
    ) -> Result<Self, DerivedInvalidationFamilyCatalogError> {
        let family = input.identity.as_str();
        let consumed_graph_facts = input.consumed_graph_facts.ok_or_else(|| {
            DerivedInvalidationFamilyCatalogError::new(
                DerivedInvalidationFamilyCatalogErrorKind::MissingConsumedGraphFacts { family },
                format!("derived product family `{family}` must declare consumed graph facts"),
            )
        })?;
        if consumed_graph_facts.is_empty() {
            return Err(DerivedInvalidationFamilyCatalogError::new(
                DerivedInvalidationFamilyCatalogErrorKind::EmptyConsumedGraphFacts { family },
                format!("derived product family `{family}` declared no consumed graph facts"),
            ));
        }
        let invalidation_predicate = required(
            input.invalidation_predicate,
            family,
            DerivedInvalidationFamilyCatalogErrorKind::MissingInvalidationPredicate { family },
            "invalidation predicate",
        )?;
        let update_posture = required(
            input.update_posture,
            family,
            DerivedInvalidationFamilyCatalogErrorKind::MissingUpdatePosture { family },
            "update posture",
        )?;
        let spatial_evidence_posture = required(
            input.spatial_evidence_posture,
            family,
            DerivedInvalidationFamilyCatalogErrorKind::MissingSpatialEvidencePosture { family },
            "spatial evidence posture",
        )?;
        let query_receipt_posture = required(
            input.query_receipt_posture,
            family,
            DerivedInvalidationFamilyCatalogErrorKind::MissingQueryReceiptPosture { family },
            "Query receipt posture",
        )?;
        let legality_receipt_posture = required(
            input.legality_receipt_posture,
            family,
            DerivedInvalidationFamilyCatalogErrorKind::MissingLegalityReceiptPosture { family },
            "legality receipt posture",
        )?;
        let diagnostic_posture = required(
            input.diagnostic_posture,
            family,
            DerivedInvalidationFamilyCatalogErrorKind::MissingDiagnosticPosture { family },
            "diagnostic posture",
        )?;
        let support_posture = required(
            input.support_posture,
            family,
            DerivedInvalidationFamilyCatalogErrorKind::MissingSupportPosture { family },
            "support posture",
        )?;
        let family_digest = family_digest(
            input.identity,
            &consumed_graph_facts,
            invalidation_predicate,
            update_posture,
            spatial_evidence_posture,
            query_receipt_posture,
            legality_receipt_posture,
            diagnostic_posture,
            support_posture,
        );
        Ok(Self {
            identity: input.identity,
            consumed_graph_facts,
            invalidation_predicate,
            update_posture,
            spatial_evidence_posture,
            query_receipt_posture,
            legality_receipt_posture,
            diagnostic_posture,
            support_posture,
            family_digest,
        })
    }

    pub const fn identity(&self) -> DerivedTopologyProductFamilyIdentity {
        self.identity
    }

    pub const fn consumed_graph_facts(&self) -> &DerivedTopologyConsumedGraphFacts {
        &self.consumed_graph_facts
    }

    pub const fn invalidation_predicate(&self) -> DerivedTopologyInvalidationPredicate {
        self.invalidation_predicate
    }

    pub const fn query_receipt_posture(&self) -> DerivedTopologyQueryReceiptPosture {
        self.query_receipt_posture
    }

    pub const fn legality_receipt_posture(&self) -> DerivedTopologyLegalityReceiptPosture {
        self.legality_receipt_posture
    }

    pub const fn update_posture(&self) -> DerivedTopologyUpdatePosture {
        self.update_posture
    }

    pub const fn spatial_evidence_posture(&self) -> DerivedTopologySpatialEvidencePosture {
        self.spatial_evidence_posture
    }

    pub fn family_digest(&self) -> &str {
        &self.family_digest
    }

    pub fn matches_touched_basis(&self, basis: &TopologyTouchedGraphBasis) -> bool {
        self.invalidation_predicate
            .matches_touched_basis(&self.consumed_graph_facts, basis)
    }
}

fn required<T>(
    value: Option<T>,
    family: &'static str,
    kind: DerivedInvalidationFamilyCatalogErrorKind,
    label: &str,
) -> Result<T, DerivedInvalidationFamilyCatalogError> {
    value.ok_or_else(|| {
        DerivedInvalidationFamilyCatalogError::new(
            kind,
            format!("derived product family `{family}` must declare {label}"),
        )
    })
}

fn family_digest(
    identity: DerivedTopologyProductFamilyIdentity,
    consumed_graph_facts: &DerivedTopologyConsumedGraphFacts,
    invalidation_predicate: DerivedTopologyInvalidationPredicate,
    update_posture: DerivedTopologyUpdatePosture,
    spatial_evidence_posture: DerivedTopologySpatialEvidencePosture,
    query_receipt_posture: DerivedTopologyQueryReceiptPosture,
    legality_receipt_posture: DerivedTopologyLegalityReceiptPosture,
    diagnostic_posture: DerivedTopologyDiagnosticPosture,
    support_posture: DerivedTopologySupportPosture,
) -> String {
    let mut parts = vec![
        "worth-topo:derived-invalidation-family:v1".to_string(),
        format!("family:{}", identity.as_str()),
        format!("predicate:{}", invalidation_predicate.as_str()),
        format!("update:{}", update_posture.as_str()),
        format!("spatial:{}", spatial_evidence_posture.as_str()),
        format!("query:{}", query_receipt_posture.as_str()),
        format!("legality:{}", legality_receipt_posture.as_str()),
        format!("diagnostic:{}", diagnostic_posture.as_str()),
        format!("support:{}", support_posture.as_str()),
    ];
    parts.extend(consumed_graph_facts.digest_parts());
    super::super::catalog_digest(parts)
}
