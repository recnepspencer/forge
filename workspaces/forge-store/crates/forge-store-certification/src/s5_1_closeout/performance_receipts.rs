use forge_foundational::{
    FoundationalAuthoritativePerformanceClaim, FoundationalCounterBackedPerformanceReceipt,
};

use crate::foundational_boundary_performance::counter_receipt;

use super::{S51CertificationCloseoutDenial, S51CloseoutCounterMatrix};

type Receipt =
    FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>;

const REQUIRED_COUNTER_NAMES: [&str; 18] = [
    "store.s5_1.closeout.scenario_evidence_rows",
    "store.s5_1.closeout.replay_transcripts",
    "store.s5_1.closeout.lower_store_requests",
    "store.s5_1.closeout.lower_store_current_authority_checks",
    "store.s5_1.closeout.lower_store_witness_sets_issued",
    "store.s5_1.closeout.lower_store_denials",
    "store.s5_1.closeout.physical_scope_drift",
    "store.s5_1.closeout.stale_key_posture",
    "store.s5_1.closeout.wrong_tenant_scope",
    "store.s5_1.closeout.missing_authenticity_requirement",
    "store.s5_1.closeout.replayed_custody_posture",
    "store.s5_1.closeout.replay_wrong_tenant_scope",
    "store.s5_1.closeout.replay_stale_key_posture",
    "store.s5_1.closeout.replay_missing_authenticity_requirement",
    "store.s5_1.closeout.replay_baseline_admissions",
    "store.s5_1.closeout.replay_attempts",
    "store.s5_1.closeout.replay_denials_before_decode",
    "store.s5_1.closeout.handoff_admitted",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S51CloseoutPerformanceReceipts {
    counter_backed: Receipt,
    rows: S51CloseoutPerformanceRows,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S51CloseoutPerformanceRows {
    scenario_evidence_rows: u64,
    replay_transcripts: u64,
    lower_store_requests: u64,
    lower_store_current_authority_checks: u64,
    lower_store_witness_sets_issued: u64,
    lower_store_denials: u64,
    physical_scope_drift: u64,
    stale_key_posture: u64,
    wrong_tenant_scope: u64,
    missing_authenticity_requirement: u64,
    replayed_custody_posture: u64,
    replay_wrong_tenant_scope: u64,
    replay_stale_key_posture: u64,
    replay_missing_authenticity_requirement: u64,
    replay_baseline_admissions: u64,
    replay_attempts: u64,
    replay_denials_before_logical_decode: u64,
    handoff_admitted: u64,
}

impl S51CloseoutPerformanceReceipts {
    pub(crate) fn from_counter_matrix(
        matrix: &S51CloseoutCounterMatrix,
    ) -> Result<Self, S51CertificationCloseoutDenial> {
        let rows = S51CloseoutPerformanceRows::from_counter_matrix(matrix);
        let counter_backed = counter_receipt(
            "store.s5_1.certification_closeout",
            &[
                (
                    "store.s5_1.closeout.scenario_evidence_rows",
                    rows.scenario_evidence_rows,
                ),
                (
                    "store.s5_1.closeout.replay_transcripts",
                    rows.replay_transcripts,
                ),
                (
                    "store.s5_1.closeout.lower_store_requests",
                    rows.lower_store_requests,
                ),
                (
                    "store.s5_1.closeout.lower_store_current_authority_checks",
                    rows.lower_store_current_authority_checks,
                ),
                (
                    "store.s5_1.closeout.lower_store_witness_sets_issued",
                    rows.lower_store_witness_sets_issued,
                ),
                (
                    "store.s5_1.closeout.lower_store_denials",
                    rows.lower_store_denials,
                ),
                (
                    "store.s5_1.closeout.physical_scope_drift",
                    rows.physical_scope_drift,
                ),
                (
                    "store.s5_1.closeout.stale_key_posture",
                    rows.stale_key_posture,
                ),
                (
                    "store.s5_1.closeout.wrong_tenant_scope",
                    rows.wrong_tenant_scope,
                ),
                (
                    "store.s5_1.closeout.missing_authenticity_requirement",
                    rows.missing_authenticity_requirement,
                ),
                (
                    "store.s5_1.closeout.replayed_custody_posture",
                    rows.replayed_custody_posture,
                ),
                (
                    "store.s5_1.closeout.replay_wrong_tenant_scope",
                    rows.replay_wrong_tenant_scope,
                ),
                (
                    "store.s5_1.closeout.replay_stale_key_posture",
                    rows.replay_stale_key_posture,
                ),
                (
                    "store.s5_1.closeout.replay_missing_authenticity_requirement",
                    rows.replay_missing_authenticity_requirement,
                ),
                (
                    "store.s5_1.closeout.replay_baseline_admissions",
                    rows.replay_baseline_admissions,
                ),
                ("store.s5_1.closeout.replay_attempts", rows.replay_attempts),
                (
                    "store.s5_1.closeout.replay_denials_before_decode",
                    rows.replay_denials_before_logical_decode,
                ),
                (
                    "store.s5_1.closeout.handoff_admitted",
                    rows.handoff_admitted,
                ),
            ],
        )?;
        Ok(Self {
            counter_backed,
            rows,
        })
    }

    pub const fn counter_backed_receipt(&self) -> &Receipt {
        &self.counter_backed
    }

    pub const fn rows(&self) -> S51CloseoutPerformanceRows {
        self.rows
    }

    pub fn all_counter_backed(&self) -> bool {
        self.counter_backed.counter_rows().len() == REQUIRED_COUNTER_NAMES.len()
            && self
                .counter_backed
                .counter_rows()
                .iter()
                .all(|row| REQUIRED_COUNTER_NAMES.contains(&row.name().as_str()))
    }

    pub const fn required_counter_names() -> &'static [&'static str; 18] {
        &REQUIRED_COUNTER_NAMES
    }
}

impl S51CloseoutPerformanceRows {
    pub const fn from_counter_matrix(matrix: &S51CloseoutCounterMatrix) -> Self {
        Self {
            scenario_evidence_rows: matrix.scenario_evidence_rows(),
            replay_transcripts: matrix.replay_transcripts(),
            lower_store_requests: matrix.lower_store_requests(),
            lower_store_current_authority_checks: matrix.lower_store_current_authority_checks(),
            lower_store_witness_sets_issued: matrix.lower_store_witness_sets_issued(),
            lower_store_denials: matrix.lower_store_denials(),
            physical_scope_drift: matrix.physical_scope_drift(),
            stale_key_posture: matrix.stale_key_posture(),
            wrong_tenant_scope: matrix.wrong_tenant_scope(),
            missing_authenticity_requirement: matrix.missing_authenticity_requirement(),
            replayed_custody_posture: matrix.replayed_custody_posture(),
            replay_wrong_tenant_scope: matrix.replay_wrong_tenant_scope(),
            replay_stale_key_posture: matrix.replay_stale_key_posture(),
            replay_missing_authenticity_requirement: matrix
                .replay_missing_authenticity_requirement(),
            replay_baseline_admissions: matrix.replay_baseline_admissions(),
            replay_attempts: matrix.replay_attempts(),
            replay_denials_before_logical_decode: matrix.replay_denials_before_logical_decode(),
            handoff_admitted: matrix.handoff_admitted(),
        }
    }

    pub const fn scenario_evidence_rows(self) -> u64 {
        self.scenario_evidence_rows
    }

    pub const fn replay_transcripts(self) -> u64 {
        self.replay_transcripts
    }

    pub const fn lower_store_requests(self) -> u64 {
        self.lower_store_requests
    }

    pub const fn lower_store_current_authority_checks(self) -> u64 {
        self.lower_store_current_authority_checks
    }

    pub const fn lower_store_witness_sets_issued(self) -> u64 {
        self.lower_store_witness_sets_issued
    }

    pub const fn lower_store_denials(self) -> u64 {
        self.lower_store_denials
    }

    pub const fn physical_scope_drift(self) -> u64 {
        self.physical_scope_drift
    }

    pub const fn stale_key_posture(self) -> u64 {
        self.stale_key_posture
    }

    pub const fn wrong_tenant_scope(self) -> u64 {
        self.wrong_tenant_scope
    }

    pub const fn missing_authenticity_requirement(self) -> u64 {
        self.missing_authenticity_requirement
    }

    pub const fn replayed_custody_posture(self) -> u64 {
        self.replayed_custody_posture
    }

    pub const fn replay_wrong_tenant_scope(self) -> u64 {
        self.replay_wrong_tenant_scope
    }

    pub const fn replay_stale_key_posture(self) -> u64 {
        self.replay_stale_key_posture
    }

    pub const fn replay_missing_authenticity_requirement(self) -> u64 {
        self.replay_missing_authenticity_requirement
    }

    pub const fn replay_baseline_admissions(self) -> u64 {
        self.replay_baseline_admissions
    }

    pub const fn replay_attempts(self) -> u64 {
        self.replay_attempts
    }

    pub const fn replay_denials_before_logical_decode(self) -> u64 {
        self.replay_denials_before_logical_decode
    }

    pub const fn handoff_admitted(self) -> u64 {
        self.handoff_admitted
    }
}
