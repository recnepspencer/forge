use crate::admission_digest::hash_parts;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BasisEligibilityCounters {
    raw_intent_width: usize,
    normalized_family_count: usize,
    source_path_count: usize,
    rejection_width: usize,
    eligibility_rows_consulted: usize,
    policy_check_count: usize,
    tenant_schema_check_count: usize,
    lower_runtime_evidence_check_count: usize,
    denied_residue_count: usize,
    scoped_capability_construction_count: usize,
    lower_runtime_binding_attempt_count: usize,
    lower_runtime_readmission_check_count: usize,
    lower_runtime_mismatch_denial_count: usize,
    retained_evidence_lookup_width: usize,
    basis_receipt_emission_count: usize,
    basis_envelope_materialization_count: usize,
    basis_support_lookup_count: usize,
    basis_support_lookup_width: usize,
    basis_certification_bundle_assembly_count: usize,
    basis_certification_row_count: usize,
}

impl BasisEligibilityCounters {
    pub(crate) fn normalized(source_path_count: usize) -> Self {
        Self {
            raw_intent_width: 1,
            normalized_family_count: 1,
            source_path_count,
            ..Self::default()
        }
    }

    #[cfg(test)]
    pub(crate) fn rejected(rejection_width: usize) -> Self {
        Self {
            raw_intent_width: 1,
            rejection_width,
            ..Self::default()
        }
    }

    pub(crate) fn eligibility(
        policy_check_count: usize,
        tenant_schema_check_count: usize,
        lower_runtime_evidence_check_count: usize,
        denied_residue_count: usize,
    ) -> Self {
        Self {
            eligibility_rows_consulted: 1,
            policy_check_count,
            tenant_schema_check_count,
            lower_runtime_evidence_check_count,
            denied_residue_count,
            ..Self::default()
        }
    }

    pub(crate) fn scoped_capability() -> Self {
        Self {
            scoped_capability_construction_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn lower_runtime_readmission(
        lower_runtime_mismatch_denial_count: usize,
        retained_evidence_lookup_width: usize,
    ) -> Self {
        Self {
            lower_runtime_binding_attempt_count: 1,
            lower_runtime_readmission_check_count: 1,
            lower_runtime_mismatch_denial_count,
            retained_evidence_lookup_width,
            ..Self::default()
        }
    }

    pub(crate) fn receipt_emission(retained_evidence_lookup_width: usize) -> Self {
        Self {
            basis_receipt_emission_count: 1,
            retained_evidence_lookup_width,
            ..Self::default()
        }
    }

    pub(crate) fn envelope_materialization() -> Self {
        Self {
            basis_envelope_materialization_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn support_lookup(basis_support_lookup_width: usize) -> Self {
        Self {
            basis_support_lookup_count: 1,
            basis_support_lookup_width,
            ..Self::default()
        }
    }

    pub fn certification_bundle_assembly(basis_certification_row_count: usize) -> Self {
        Self {
            basis_certification_bundle_assembly_count: 1,
            basis_certification_row_count,
            ..Self::default()
        }
    }

    pub fn raw_intent_width(&self) -> usize {
        self.raw_intent_width
    }

    pub fn normalized_family_count(&self) -> usize {
        self.normalized_family_count
    }

    pub fn source_path_count(&self) -> usize {
        self.source_path_count
    }

    pub fn rejection_width(&self) -> usize {
        self.rejection_width
    }

    pub fn eligibility_rows_consulted(&self) -> usize {
        self.eligibility_rows_consulted
    }

    pub fn policy_check_count(&self) -> usize {
        self.policy_check_count
    }

    pub fn tenant_schema_check_count(&self) -> usize {
        self.tenant_schema_check_count
    }

    pub fn lower_runtime_evidence_check_count(&self) -> usize {
        self.lower_runtime_evidence_check_count
    }

    pub fn denied_residue_count(&self) -> usize {
        self.denied_residue_count
    }

    pub fn scoped_capability_construction_count(&self) -> usize {
        self.scoped_capability_construction_count
    }

    pub fn lower_runtime_binding_attempt_count(&self) -> usize {
        self.lower_runtime_binding_attempt_count
    }

    pub fn lower_runtime_readmission_check_count(&self) -> usize {
        self.lower_runtime_readmission_check_count
    }

    pub fn lower_runtime_mismatch_denial_count(&self) -> usize {
        self.lower_runtime_mismatch_denial_count
    }

    pub fn retained_evidence_lookup_width(&self) -> usize {
        self.retained_evidence_lookup_width
    }

    pub fn basis_receipt_emission_count(&self) -> usize {
        self.basis_receipt_emission_count
    }

    pub fn basis_envelope_materialization_count(&self) -> usize {
        self.basis_envelope_materialization_count
    }

    pub fn basis_support_lookup_count(&self) -> usize {
        self.basis_support_lookup_count
    }

    pub fn basis_support_lookup_width(&self) -> usize {
        self.basis_support_lookup_width
    }

    pub fn basis_certification_bundle_assembly_count(&self) -> usize {
        self.basis_certification_bundle_assembly_count
    }

    pub fn basis_certification_row_count(&self) -> usize {
        self.basis_certification_row_count
    }

    pub fn digest(&self) -> String {
        hash_parts(&[
            format!("raw:{}", self.raw_intent_width),
            format!("normalized:{}", self.normalized_family_count),
            format!("source_paths:{}", self.source_path_count),
            format!("rejections:{}", self.rejection_width),
            format!("eligibility_rows:{}", self.eligibility_rows_consulted),
            format!("policy_checks:{}", self.policy_check_count),
            format!("tenant_schema_checks:{}", self.tenant_schema_check_count),
            format!(
                "lower_runtime_checks:{}",
                self.lower_runtime_evidence_check_count
            ),
            format!("denied_residue:{}", self.denied_residue_count),
            format!(
                "scoped_construction:{}",
                self.scoped_capability_construction_count
            ),
            format!(
                "lower_runtime_binding_attempts:{}",
                self.lower_runtime_binding_attempt_count
            ),
            format!(
                "lower_runtime_readmission_checks:{}",
                self.lower_runtime_readmission_check_count
            ),
            format!(
                "lower_runtime_mismatch_denials:{}",
                self.lower_runtime_mismatch_denial_count
            ),
            format!(
                "retained_evidence_lookup_width:{}",
                self.retained_evidence_lookup_width
            ),
            format!("basis_receipts:{}", self.basis_receipt_emission_count),
            format!(
                "basis_envelopes:{}",
                self.basis_envelope_materialization_count
            ),
            format!("basis_support_lookups:{}", self.basis_support_lookup_count),
            format!(
                "basis_support_lookup_width:{}",
                self.basis_support_lookup_width
            ),
            format!(
                "basis_certification_bundles:{}",
                self.basis_certification_bundle_assembly_count
            ),
            format!(
                "basis_certification_rows:{}",
                self.basis_certification_row_count
            ),
        ])
    }
}
