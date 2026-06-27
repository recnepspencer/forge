use crate::workload_platform::evidence_lookup_family_catalog::{
    EvidenceLookupFamilyCatalogCloseout, EvidenceLookupFamilyQueryPosture,
    EvidenceLookupProjectionFactFamily, EvidenceLookupQueryImportEvidence,
};
use forge_query::facade::consumer_kit::ForgeQueryGraphObligationSupportPin;
use forge_query::facade::ProjectionConsumptionReceipt;

use super::error::{EvidenceLookupInputAdmissionError, EvidenceLookupInputAdmissionErrorKind};
use crate::workload_platform::evidence_lookup_query_surface_contract::EvidenceLookupQuerySurfaceContract;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupQueryAdmissionSupport {
    family_identity: String,
    state: EvidenceLookupQuerySupportState,
    query_surface_contract: Option<EvidenceLookupQuerySurfaceContract>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupQueryAdmissionEvidenceSet {
    evidence: Vec<EvidenceLookupQueryAdmissionEvidence>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct EvidenceLookupQueryAdmissionEvidence {
    evidence_digest: String,
    query_surface_type_name: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceLookupQuerySupportState {
    NotRequired,
    Satisfied {
        imported_evidence_digest: String,
        query_surface_type_name: &'static str,
    },
}

impl EvidenceLookupQueryAdmissionSupport {
    pub(crate) fn not_required(family_identity: impl Into<String>) -> Self {
        Self {
            family_identity: family_identity.into(),
            state: EvidenceLookupQuerySupportState::NotRequired,
            query_surface_contract: None,
        }
    }

    pub(crate) fn from_catalog_posture(
        family_identity: impl Into<String>,
        posture: &EvidenceLookupFamilyQueryPosture,
        evidence_set: Option<&EvidenceLookupQueryAdmissionEvidenceSet>,
    ) -> Result<Self, EvidenceLookupInputAdmissionError> {
        let family_identity = family_identity.into();
        if !posture.requires_query_evidence() {
            return Ok(Self::not_required(family_identity));
        }
        let Some(imported_evidence) = posture.imported_evidence() else {
            return Err(EvidenceLookupInputAdmissionError::new(
                EvidenceLookupInputAdmissionErrorKind::MissingQueryImportEvidence,
                "query posture requires evidence but did not declare an import surface",
            ));
        };
        let Some(evidence_set) = evidence_set else {
            return Err(EvidenceLookupInputAdmissionError::new(
                EvidenceLookupInputAdmissionErrorKind::MissingQueryImportEvidence,
                imported_evidence.evidence_digest(),
            ));
        };
        if !evidence_set.contains(imported_evidence) {
            return Err(EvidenceLookupInputAdmissionError::new(
                EvidenceLookupInputAdmissionErrorKind::MissingQueryImportEvidence,
                imported_evidence.evidence_digest(),
            ));
        }
        Ok(Self::from_imported_evidence(
            family_identity,
            posture,
            imported_evidence,
        ))
    }

    fn from_imported_evidence(
        family_identity: impl Into<String>,
        posture: &EvidenceLookupFamilyQueryPosture,
        imported_evidence: &EvidenceLookupQueryImportEvidence,
    ) -> Self {
        Self {
            family_identity: family_identity.into(),
            state: EvidenceLookupQuerySupportState::Satisfied {
                imported_evidence_digest: imported_evidence.evidence_digest().to_string(),
                query_surface_type_name: imported_evidence.query_surface_type_name(),
            },
            query_surface_contract: Some(
                EvidenceLookupQuerySurfaceContract::from_imported_evidence(
                    posture.kind(),
                    imported_evidence,
                ),
            ),
        }
    }

    pub fn family_identity(&self) -> &str {
        &self.family_identity
    }

    pub const fn state(&self) -> &EvidenceLookupQuerySupportState {
        &self.state
    }

    pub const fn query_surface_contract(&self) -> Option<&EvidenceLookupQuerySurfaceContract> {
        self.query_surface_contract.as_ref()
    }

    pub const fn claims_lookup_product_authority(&self) -> bool {
        false
    }

    pub(crate) fn digest_summary_part(&self) -> String {
        match self.state() {
            EvidenceLookupQuerySupportState::NotRequired => {
                format!("query:{}:not-required", self.family_identity)
            }
            EvidenceLookupQuerySupportState::Satisfied {
                imported_evidence_digest,
                query_surface_type_name,
            } => format!(
                "query:{}:satisfied:{}:{}",
                self.family_identity, query_surface_type_name, imported_evidence_digest
            ),
        }
    }
}

impl EvidenceLookupQueryAdmissionEvidenceSet {
    pub fn from_family_catalog(catalog: &EvidenceLookupFamilyCatalogCloseout) -> Self {
        let evidence = catalog
            .declarations()
            .iter()
            .filter_map(|family| family.query_posture().imported_evidence())
            .map(EvidenceLookupQueryAdmissionEvidence::from_imported_evidence)
            .collect();
        Self { evidence }
    }

    pub fn from_query_import_evidence(
        imported_evidence: &EvidenceLookupQueryImportEvidence,
    ) -> Self {
        Self {
            evidence: vec![
                EvidenceLookupQueryAdmissionEvidence::from_imported_evidence(imported_evidence),
            ],
        }
    }

    pub fn from_query_import_evidence_iter<'a>(
        imported_evidence: impl IntoIterator<Item = &'a EvidenceLookupQueryImportEvidence>,
    ) -> Self {
        Self {
            evidence: imported_evidence
                .into_iter()
                .map(EvidenceLookupQueryAdmissionEvidence::from_imported_evidence)
                .collect(),
        }
    }

    pub fn from_support_pin(support_pin: ForgeQueryGraphObligationSupportPin) -> Self {
        Self::from_query_import_evidence(
            &EvidenceLookupQueryImportEvidence::ConsumerKitSupportPin { support_pin },
        )
    }

    pub fn from_projection_consumption_receipt(
        receipt: &ProjectionConsumptionReceipt,
        fact_family: EvidenceLookupProjectionFactFamily,
    ) -> Self {
        let imported_evidence =
            EvidenceLookupFamilyQueryPosture::imported_projection_consumption_required(fact_family)
                .imported_evidence()
                .expect("projection posture should declare imported evidence")
                .clone();
        assert!(
            !receipt.receipt_digest().is_empty(),
            "projection consumption receipts must carry a runtime digest"
        );
        Self::from_query_import_evidence(&imported_evidence)
    }

    fn contains(&self, imported_evidence: &EvidenceLookupQueryImportEvidence) -> bool {
        self.evidence.iter().any(|evidence| {
            evidence.evidence_digest == imported_evidence.evidence_digest()
                && evidence.query_surface_type_name == imported_evidence.query_surface_type_name()
        })
    }

    pub fn evidence_count(&self) -> usize {
        self.evidence.len()
    }
}

impl EvidenceLookupQueryAdmissionEvidence {
    fn from_imported_evidence(imported_evidence: &EvidenceLookupQueryImportEvidence) -> Self {
        Self {
            evidence_digest: imported_evidence.evidence_digest().to_string(),
            query_surface_type_name: imported_evidence.query_surface_type_name(),
        }
    }
}
