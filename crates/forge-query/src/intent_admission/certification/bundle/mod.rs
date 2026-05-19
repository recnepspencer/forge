mod outputs;

use self::outputs::{assemble_certification_outputs, certification_bundle_digest};
use super::audits::{
    forge_query_intent_admission_proof_shape_audit, ForgeQueryIntentAdmissionProofShapeAudit,
    ForgeQueryIntentAdmissionPublicBoundaryAudit, ForgeQueryIntentAdmissionTopologyAudit,
};
use super::oracles::{
    forge_query_intent_admission_oracle_report, ForgeQueryIntentAdmissionOracleReport,
};
use super::output_manifest::forge_query_intent_admission_certification_output_manifest;
use super::reports::{
    forge_query_intent_admission_doc_example_report,
    forge_query_intent_admission_legacy_parity_report,
    forge_query_intent_admission_representative_family_report,
    forge_query_intent_admission_representative_output_report,
    forge_query_intent_admission_seeded_certification_report,
    forge_query_intent_admission_slope_report,
    forge_query_intent_admission_support_traceability_report,
    ForgeQueryIntentAdmissionCertificationCounterSnapshot,
    ForgeQueryIntentAdmissionDocExampleReport, ForgeQueryIntentAdmissionLegacyParityReport,
    ForgeQueryIntentAdmissionRepresentativeFamilyReport,
    ForgeQueryIntentAdmissionRepresentativeOutputReport,
    ForgeQueryIntentAdmissionSeededCertificationReport, ForgeQueryIntentAdmissionSlopeReport,
    ForgeQueryIntentAdmissionSupportTraceabilityReport,
};
use crate::intent_admission::{
    forge_query_intent_admission_coverage_inventory, forge_query_intent_admission_family_inventory,
    forge_query_intent_admission_support_matrix, ForgeQueryIntentAdmissionCoverageInventory,
    ForgeQueryIntentAdmissionFamilyInventory, ForgeQueryIntentAdmissionSupportMatrix,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionCertificationOutput {
    name: &'static str,
    digest: String,
}

impl ForgeQueryIntentAdmissionCertificationOutput {
    pub(crate) fn new(name: &'static str, digest: String) -> Self {
        Self { name, digest }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionCertificationBundle {
    output_manifest: Vec<&'static str>,
    family_inventory: ForgeQueryIntentAdmissionFamilyInventory,
    coverage_inventory: ForgeQueryIntentAdmissionCoverageInventory,
    support_matrix: ForgeQueryIntentAdmissionSupportMatrix,
    public_boundary_audit: ForgeQueryIntentAdmissionPublicBoundaryAudit,
    proof_shape_audit: ForgeQueryIntentAdmissionProofShapeAudit,
    topology_audit: ForgeQueryIntentAdmissionTopologyAudit,
    representative_output_report: ForgeQueryIntentAdmissionRepresentativeOutputReport,
    representative_family_report: ForgeQueryIntentAdmissionRepresentativeFamilyReport,
    doc_example_report: ForgeQueryIntentAdmissionDocExampleReport,
    oracle_report: ForgeQueryIntentAdmissionOracleReport,
    legacy_parity_report: ForgeQueryIntentAdmissionLegacyParityReport,
    support_traceability_report: ForgeQueryIntentAdmissionSupportTraceabilityReport,
    seeded_report: ForgeQueryIntentAdmissionSeededCertificationReport,
    slope_report: ForgeQueryIntentAdmissionSlopeReport,
    outputs: Vec<ForgeQueryIntentAdmissionCertificationOutput>,
    certification_bundle_digest: String,
}

impl ForgeQueryIntentAdmissionCertificationBundle {
    fn new(
        family_inventory: ForgeQueryIntentAdmissionFamilyInventory,
        coverage_inventory: ForgeQueryIntentAdmissionCoverageInventory,
        support_matrix: ForgeQueryIntentAdmissionSupportMatrix,
        public_boundary_audit: ForgeQueryIntentAdmissionPublicBoundaryAudit,
        proof_shape_audit: ForgeQueryIntentAdmissionProofShapeAudit,
        topology_audit: ForgeQueryIntentAdmissionTopologyAudit,
        representative_output_report: ForgeQueryIntentAdmissionRepresentativeOutputReport,
        representative_family_report: ForgeQueryIntentAdmissionRepresentativeFamilyReport,
        doc_example_report: ForgeQueryIntentAdmissionDocExampleReport,
        oracle_report: ForgeQueryIntentAdmissionOracleReport,
        legacy_parity_report: ForgeQueryIntentAdmissionLegacyParityReport,
        support_traceability_report: ForgeQueryIntentAdmissionSupportTraceabilityReport,
        seeded_report: ForgeQueryIntentAdmissionSeededCertificationReport,
        slope_report: ForgeQueryIntentAdmissionSlopeReport,
    ) -> Self {
        let output_manifest = forge_query_intent_admission_certification_output_manifest().to_vec();
        let output_specs = assemble_certification_outputs(
            &family_inventory,
            &coverage_inventory,
            &support_matrix,
            &public_boundary_audit,
            &proof_shape_audit,
            &topology_audit,
            &representative_output_report,
            &representative_family_report,
            &doc_example_report,
            &oracle_report,
            &legacy_parity_report,
            &support_traceability_report,
            &seeded_report,
            &slope_report,
        );
        validate_output_manifest(&output_manifest, &output_specs);
        let certification_bundle_digest = certification_bundle_digest(&output_specs);
        let outputs = output_specs
            .iter()
            .map(|output| {
                ForgeQueryIntentAdmissionCertificationOutput::new(
                    output.name(),
                    output.digest().to_string(),
                )
            })
            .collect::<Vec<_>>();
        Self {
            output_manifest,
            family_inventory,
            coverage_inventory,
            support_matrix,
            public_boundary_audit,
            proof_shape_audit,
            topology_audit,
            representative_output_report,
            representative_family_report,
            doc_example_report,
            oracle_report,
            legacy_parity_report,
            support_traceability_report,
            seeded_report,
            slope_report,
            outputs,
            certification_bundle_digest,
        }
    }

    pub fn output_manifest(&self) -> &[&'static str] {
        &self.output_manifest
    }

    pub fn family_inventory(&self) -> &ForgeQueryIntentAdmissionFamilyInventory {
        &self.family_inventory
    }

    pub fn coverage_inventory(&self) -> &ForgeQueryIntentAdmissionCoverageInventory {
        &self.coverage_inventory
    }

    pub fn support_matrix(&self) -> &ForgeQueryIntentAdmissionSupportMatrix {
        &self.support_matrix
    }

    pub fn public_boundary_audit(&self) -> &ForgeQueryIntentAdmissionPublicBoundaryAudit {
        &self.public_boundary_audit
    }

    pub fn proof_shape_audit(&self) -> &ForgeQueryIntentAdmissionProofShapeAudit {
        &self.proof_shape_audit
    }

    pub fn topology_audit(&self) -> &ForgeQueryIntentAdmissionTopologyAudit {
        &self.topology_audit
    }

    pub fn representative_output_report(
        &self,
    ) -> &ForgeQueryIntentAdmissionRepresentativeOutputReport {
        &self.representative_output_report
    }

    pub fn representative_family_report(
        &self,
    ) -> &ForgeQueryIntentAdmissionRepresentativeFamilyReport {
        &self.representative_family_report
    }

    pub fn doc_example_report(&self) -> &ForgeQueryIntentAdmissionDocExampleReport {
        &self.doc_example_report
    }

    pub fn oracle_report(&self) -> &ForgeQueryIntentAdmissionOracleReport {
        &self.oracle_report
    }

    pub fn legacy_parity_report(&self) -> &ForgeQueryIntentAdmissionLegacyParityReport {
        &self.legacy_parity_report
    }

    pub fn support_traceability_report(
        &self,
    ) -> &ForgeQueryIntentAdmissionSupportTraceabilityReport {
        &self.support_traceability_report
    }

    pub fn seeded_report(&self) -> &ForgeQueryIntentAdmissionSeededCertificationReport {
        &self.seeded_report
    }

    pub fn counter_snapshot(&self) -> &ForgeQueryIntentAdmissionCertificationCounterSnapshot {
        self.slope_report.counter_snapshot()
    }

    pub fn slope_report(&self) -> &ForgeQueryIntentAdmissionSlopeReport {
        &self.slope_report
    }

    pub fn outputs(&self) -> &[ForgeQueryIntentAdmissionCertificationOutput] {
        &self.outputs
    }

    pub fn output_digest(&self, key: &str) -> Option<&str> {
        self.outputs
            .iter()
            .find(|output| output.name() == key)
            .map(ForgeQueryIntentAdmissionCertificationOutput::digest)
    }

    pub fn certification_bundle_digest(&self) -> &str {
        &self.certification_bundle_digest
    }
}

pub fn certify_intent_admission() -> ForgeQueryIntentAdmissionCertificationBundle {
    ForgeQueryIntentAdmissionCertificationBundle::new(
        forge_query_intent_admission_family_inventory(),
        forge_query_intent_admission_coverage_inventory(),
        forge_query_intent_admission_support_matrix(),
        ForgeQueryIntentAdmissionPublicBoundaryAudit::new(),
        forge_query_intent_admission_proof_shape_audit(),
        ForgeQueryIntentAdmissionTopologyAudit::new(),
        forge_query_intent_admission_representative_output_report(),
        forge_query_intent_admission_representative_family_report(),
        forge_query_intent_admission_doc_example_report(),
        forge_query_intent_admission_oracle_report(),
        forge_query_intent_admission_legacy_parity_report(),
        forge_query_intent_admission_support_traceability_report(),
        forge_query_intent_admission_seeded_certification_report(),
        forge_query_intent_admission_slope_report(),
    )
}

fn validate_output_manifest(
    output_manifest: &[&'static str],
    outputs: &[self::outputs::ForgeQueryIntentAdmissionCertificationOutputSpec],
) {
    let actual_names = outputs
        .iter()
        .map(|output| output.name())
        .collect::<Vec<_>>();
    assert_eq!(
        actual_names.len(),
        actual_names
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        "intent-admission certification outputs must be duplicate-free"
    );
    assert_eq!(
        actual_names, output_manifest,
        "intent-admission certification outputs must match the compile-visible manifest exactly"
    );
}
