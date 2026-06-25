use super::super::read_family_catalog::WorthGraphReadDeclarationCatalogRecord;
use super::derivation_attempt::WorthGraphReadRequirementDerivationAttempt;
use super::derivation_outcome::WorthGraphReadRequirementDerivationOutcome;
use super::errors::WorthGraphReadRequirementDerivationError;
use super::query_projection::{
    derivation_attempt_for_catalog_record, derive_query_requirement_outcome_for_catalog_record,
};
use super::source_trace::WorthGraphReadRequirementSourceTrace;
use super::stable_identity_digest::stable_digest;
use forge_query::facade::{ForgeQueryReadFamily, ForgeQueryReadFamilyAdmission};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadRequirementDerivationRecord {
    source_catalog_record_digest: String,
    source_catalog_key_digest: String,
    touched_authority_input: String,
    read_family_target: String,
    query_family_name: String,
    query_family_digest_seed: String,
    query_family_admission_boundary: String,
    query_touch_descriptor_digest: String,
    operating_world_digest: String,
    source_trace: WorthGraphReadRequirementSourceTrace,
    derivation_attempt: WorthGraphReadRequirementDerivationAttempt,
    derivation_outcome: WorthGraphReadRequirementDerivationOutcome,
    record_digest: String,
}

impl WorthGraphReadRequirementDerivationRecord {
    pub(crate) fn from_catalog_record(
        record: &WorthGraphReadDeclarationCatalogRecord,
    ) -> Result<Self, WorthGraphReadRequirementDerivationError> {
        let source_trace = WorthGraphReadRequirementSourceTrace::new(
            record.declaration_identity_digest(),
            record.key().key_digest(),
            record.key().requirement_evidence_digest(),
            record.source_row_identities().to_vec(),
        );
        let derivation_attempt = derivation_attempt_for_catalog_record(record);
        let derivation_outcome = derive_query_requirement_outcome_for_catalog_record(record)?;
        let record_digest = stable_digest(&[
            "worth_graph_read_requirement_derivation_record_v1".to_string(),
            format!("catalog_record:{}", record.declaration_identity_digest()),
            format!("catalog_key:{}", record.key().key_digest()),
            format!(
                "query_family:{}",
                record.query_family_anchor().family_digest_seed()
            ),
            format!("trace:{}", source_trace.trace_digest()),
            format!("attempt:{}", derivation_attempt.attempt_digest()),
            format!("outcome:{}", outcome_digest_part(&derivation_outcome)),
        ]);
        Ok(Self {
            source_catalog_record_digest: record.declaration_identity_digest().to_string(),
            source_catalog_key_digest: record.key().key_digest().to_string(),
            touched_authority_input: record.key().touched_authority_input().to_string(),
            read_family_target: record.key().read_family_target().to_string(),
            query_family_name: record.query_family_anchor().family_name().to_string(),
            query_family_digest_seed: record
                .query_family_anchor()
                .family_digest_seed()
                .to_string(),
            query_family_admission_boundary: query_family_admission_boundary_label(
                record.query_family_anchor().admission_boundary(),
            ),
            query_touch_descriptor_digest: record.key().query_touch_descriptor_digest().to_string(),
            operating_world_digest: record.key().operating_world_digest().to_string(),
            source_trace,
            derivation_attempt,
            derivation_outcome,
            record_digest,
        })
    }

    pub fn source_catalog_record_digest(&self) -> &str {
        &self.source_catalog_record_digest
    }

    pub fn source_catalog_key_digest(&self) -> &str {
        &self.source_catalog_key_digest
    }

    pub fn touched_authority_input(&self) -> &str {
        &self.touched_authority_input
    }

    pub fn read_family_target(&self) -> &str {
        &self.read_family_target
    }

    pub fn query_family_name(&self) -> &str {
        &self.query_family_name
    }

    pub fn query_family_digest_seed(&self) -> &str {
        &self.query_family_digest_seed
    }

    pub fn query_family_admission_boundary(&self) -> &str {
        &self.query_family_admission_boundary
    }

    pub fn query_touch_descriptor_digest(&self) -> &str {
        &self.query_touch_descriptor_digest
    }

    pub fn operating_world_digest(&self) -> &str {
        &self.operating_world_digest
    }

    pub fn requirement_source_trace(&self) -> &WorthGraphReadRequirementSourceTrace {
        &self.source_trace
    }

    pub fn derivation_attempt(&self) -> &WorthGraphReadRequirementDerivationAttempt {
        &self.derivation_attempt
    }

    pub fn derivation_outcome(&self) -> &WorthGraphReadRequirementDerivationOutcome {
        &self.derivation_outcome
    }

    pub fn query_read_family_artifact(&self) -> Option<&ForgeQueryReadFamily> {
        None
    }

    pub fn record_digest(&self) -> &str {
        &self.record_digest
    }

    pub const fn claims_graph_read_execution(&self) -> bool {
        false
    }

    pub const fn claims_access_plan_consumption(&self) -> bool {
        false
    }
}

fn query_family_admission_boundary_label(admission: &ForgeQueryReadFamilyAdmission) -> String {
    match admission {
        ForgeQueryReadFamilyAdmission::KernelOnly => "kernel_only".to_string(),
        ForgeQueryReadFamilyAdmission::DomainInvariantAdmitted(evidence) => {
            format!("domain_invariant_admitted:{}", evidence.evidence_digest())
        }
    }
}

fn outcome_digest_part(outcome: &WorthGraphReadRequirementDerivationOutcome) -> String {
    match outcome {
        WorthGraphReadRequirementDerivationOutcome::QueryDerived(evidence) => {
            format!("query_derived:{}", evidence.query_requirement_set_digest())
        }
        WorthGraphReadRequirementDerivationOutcome::QueryCapabilityGap(gap) => {
            format!("query_gap:{}", gap.gap_digest())
        }
    }
}
