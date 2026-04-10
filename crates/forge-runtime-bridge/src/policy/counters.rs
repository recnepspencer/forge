use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    AdmittedBridgePolicyContract, BridgePolicyDeclaration, BridgePolicyProvenanceRecord,
    BridgePolicyReplayBundle, BridgePolicyRejection, BridgePolicyRejectionKind,
    BridgePolicyResolution,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePolicyCounters {
    declaration_count: usize,
    declaration_width_count: usize,
    admitted_contract_count: usize,
    admission_width_count: usize,
    rejected_contract_count: usize,
    provenance_entry_count: usize,
    provenance_width_count: usize,
    narrowed_field_count: usize,
    inherited_field_count: usize,
    override_count: usize,
    ignored_field_count: usize,
    replay_bundle_count: usize,
    replay_mismatch_count: usize,
    ambient_policy_leak_count: usize,
    policy_request_count: usize,
    truth_view_interleave_count: usize,
    preview_equivalence_preserved_count: usize,
    policy_source_ambiguity_count: usize,
    substantive_illegality_count: usize,
    fallback_success_count: usize,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgePolicyCounters {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        declaration_count: usize,
        declaration_width_count: usize,
        admitted_contract_count: usize,
        admission_width_count: usize,
        rejected_contract_count: usize,
        provenance_entry_count: usize,
        provenance_width_count: usize,
        narrowed_field_count: usize,
        inherited_field_count: usize,
        override_count: usize,
        ignored_field_count: usize,
        replay_bundle_count: usize,
        replay_mismatch_count: usize,
        ambient_policy_leak_count: usize,
        policy_request_count: usize,
        truth_view_interleave_count: usize,
        preview_equivalence_preserved_count: usize,
        policy_source_ambiguity_count: usize,
        substantive_illegality_count: usize,
        fallback_success_count: usize,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            concat!(
                "bridge-policy-counters|declaration-count:{}|declaration-width-count:{}|",
                "admitted-contract-count:{}|admission-width-count:{}|",
                "rejected-contract-count:{}|provenance-entry-count:{}|",
                "provenance-width-count:{}|narrowed-field-count:{}|inherited-field-count:{}|",
                "override-count:{}|ignored-field-count:{}|replay-bundle-count:{}|",
                "replay-mismatch-count:{}|ambient-policy-leak-count:{}|policy-request-count:{}|truth-view-interleave-count:{}|",
                "preview-equivalence-preserved-count:{}|policy-source-ambiguity-count:{}|",
                "substantive-illegality-count:{}|fallback-success-count:{}"
            ),
            declaration_count,
            declaration_width_count,
            admitted_contract_count,
            admission_width_count,
            rejected_contract_count,
            provenance_entry_count,
            provenance_width_count,
            narrowed_field_count,
            inherited_field_count,
            override_count,
            ignored_field_count,
            replay_bundle_count,
            replay_mismatch_count,
            ambient_policy_leak_count,
            policy_request_count,
            truth_view_interleave_count,
            preview_equivalence_preserved_count,
            policy_source_ambiguity_count,
            substantive_illegality_count,
            fallback_success_count,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            declaration_count,
            declaration_width_count,
            admitted_contract_count,
            admission_width_count,
            rejected_contract_count,
            provenance_entry_count,
            provenance_width_count,
            narrowed_field_count,
            inherited_field_count,
            override_count,
            ignored_field_count,
            replay_bundle_count,
            replay_mismatch_count,
            ambient_policy_leak_count,
            policy_request_count,
            truth_view_interleave_count,
            preview_equivalence_preserved_count,
            policy_source_ambiguity_count,
            substantive_illegality_count,
            fallback_success_count,
            canonical_basis,
            digest: Arc::from(format!("bridge-policy-counters:sha256:{digest:x}")),
        }
    }

    pub fn declaration_count(&self) -> usize { self.declaration_count }
    pub fn declaration_width_count(&self) -> usize { self.declaration_width_count }
    pub fn admitted_contract_count(&self) -> usize { self.admitted_contract_count }
    pub fn admission_width_count(&self) -> usize { self.admission_width_count }
    pub fn rejected_contract_count(&self) -> usize { self.rejected_contract_count }
    pub fn provenance_entry_count(&self) -> usize { self.provenance_entry_count }
    pub fn provenance_width_count(&self) -> usize { self.provenance_width_count }
    pub fn narrowed_field_count(&self) -> usize { self.narrowed_field_count }
    pub fn inherited_field_count(&self) -> usize { self.inherited_field_count }
    pub fn override_count(&self) -> usize { self.override_count }
    pub fn ignored_field_count(&self) -> usize { self.ignored_field_count }
    pub fn replay_bundle_count(&self) -> usize { self.replay_bundle_count }
    pub fn replay_mismatch_count(&self) -> usize { self.replay_mismatch_count }
    pub fn ambient_policy_leak_count(&self) -> usize { self.ambient_policy_leak_count }
    pub fn policy_request_count(&self) -> usize { self.policy_request_count }
    pub fn truth_view_interleave_count(&self) -> usize { self.truth_view_interleave_count }
    pub fn preview_equivalence_preserved_count(&self) -> usize {
        self.preview_equivalence_preserved_count
    }
    pub fn policy_source_ambiguity_count(&self) -> usize { self.policy_source_ambiguity_count }
    pub fn substantive_illegality_count(&self) -> usize { self.substantive_illegality_count }
    pub fn fallback_success_count(&self) -> usize { self.fallback_success_count }
    pub fn canonical_basis(&self) -> &str { self.canonical_basis.as_ref() }
    pub fn digest(&self) -> &str { self.digest.as_ref() }

    #[allow(clippy::too_many_arguments)]
    pub fn from_admitted_artifacts(
        declarations: &[&BridgePolicyDeclaration],
        contracts: &[&AdmittedBridgePolicyContract],
        provenances: &[&BridgePolicyProvenanceRecord],
        replay_bundles: &[&BridgePolicyReplayBundle],
        rejected_contract_count: usize,
        replay_mismatch_count: usize,
        ambient_policy_leak_count: usize,
        policy_request_count: usize,
        truth_view_interleave_count: usize,
        preview_equivalence_preserved_count: usize,
        policy_source_ambiguity_count: usize,
        substantive_illegality_count: usize,
        fallback_success_count: usize,
    ) -> Self {
        let declaration_count = declarations.len();
        let declaration_width_count = declarations
            .iter()
            .map(|declaration| declaration.policy_field_count())
            .sum();
        let admitted_contract_count = contracts.len();
        let admission_width_count = contracts
            .iter()
            .map(|contract| contract.resolution_entries().len())
            .sum();
        let provenance_entry_count = provenances
            .iter()
            .map(|provenance| provenance.entries().len())
            .sum();
        let provenance_width_count = provenance_entry_count;
        let narrowed_field_count = provenances
            .iter()
            .map(|provenance| {
                provenance
                    .entries()
                    .iter()
                    .filter(|entry| entry.resolution() == BridgePolicyResolution::Narrowed)
                    .count()
            })
            .sum();
        let inherited_field_count = provenances
            .iter()
            .map(|provenance| {
                provenance
                    .entries()
                    .iter()
                    .filter(|entry| entry.resolution() == BridgePolicyResolution::Inherited)
                    .count()
            })
            .sum();
        let override_count = provenances
            .iter()
            .map(|provenance| {
                provenance
                    .entries()
                    .iter()
                    .filter(|entry| entry.declared_source() != entry.operative_source())
                    .count()
            })
            .sum();
        let ignored_field_count = inherited_field_count;

        Self::new(
            declaration_count,
            declaration_width_count,
            admitted_contract_count,
            admission_width_count,
            rejected_contract_count,
            provenance_entry_count,
            provenance_width_count,
            narrowed_field_count,
            inherited_field_count,
            override_count,
            ignored_field_count,
            replay_bundles.len(),
            replay_mismatch_count,
            ambient_policy_leak_count,
            policy_request_count,
            truth_view_interleave_count,
            preview_equivalence_preserved_count,
            policy_source_ambiguity_count,
            substantive_illegality_count,
            fallback_success_count,
        )
    }

    pub fn from_rejections(
        declarations: &[&BridgePolicyDeclaration],
        rejections: &[&BridgePolicyRejection],
        fallback_success_count: usize,
    ) -> Self {
        let policy_source_ambiguity_count = rejections
            .iter()
            .filter(|rejection| {
                rejection.kind() == BridgePolicyRejectionKind::PolicySourceAmbiguity
            })
            .count();
        Self::from_admitted_artifacts(
            declarations,
            &[],
            &[],
            &[],
            rejections.len(),
            0,
            0,
            0,
            0,
            0,
            policy_source_ambiguity_count,
            rejections.len(),
            fallback_success_count,
        )
    }
}
