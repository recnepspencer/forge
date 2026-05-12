use crate::identity::hash_parts;

use super::{
    ForgeQueryReadCompositionPhaseOneCloseout, ForgeQueryReadCompositionSupportReport,
    ForgeQueryRuntime, ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimePublicSupportMatrix,
    ForgeQueryRuntimeSupportProfile,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryReadCompositionPhaseGateFamily {
    PhaseOneKernelComplete,
    PhaseTwoWorthAdoptionReady,
    PhaseThreeAggregateProofComplete,
    MilestoneThreeResumeReady,
}

impl ForgeQueryReadCompositionPhaseGateFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PhaseOneKernelComplete => "phase_one_kernel_complete",
            Self::PhaseTwoWorthAdoptionReady => "phase_two_worth_adoption_ready",
            Self::PhaseThreeAggregateProofComplete => "phase_three_aggregate_proof_complete",
            Self::MilestoneThreeResumeReady => "milestone_three_resume_ready",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryReadCompositionPhaseGateStatus {
    Satisfied,
    Blocked,
}

impl ForgeQueryReadCompositionPhaseGateStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryReadCompositionPhaseGateRow {
    family: ForgeQueryReadCompositionPhaseGateFamily,
    status: ForgeQueryReadCompositionPhaseGateStatus,
    reason: String,
    row_digest: String,
}

impl ForgeQueryReadCompositionPhaseGateRow {
    fn new(
        family: ForgeQueryReadCompositionPhaseGateFamily,
        status: ForgeQueryReadCompositionPhaseGateStatus,
        reason: impl Into<String>,
    ) -> Self {
        let reason = reason.into();
        let row_digest = hash_parts(&[
            format!("family:{}", family.as_str()),
            format!("status:{}", status.as_str()),
            format!("reason:{reason}"),
        ]);
        Self {
            family,
            status,
            reason,
            row_digest,
        }
    }

    pub fn family(&self) -> ForgeQueryReadCompositionPhaseGateFamily {
        self.family
    }

    pub fn status(&self) -> ForgeQueryReadCompositionPhaseGateStatus {
        self.status
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryReadCompositionPhaseGate {
    backend_posture: ForgeQueryRuntimeBackendPosture,
    support_matrix_digest: String,
    read_support_digest: String,
    phase_one_closeout_digest: String,
    phase_two_start_family: String,
    rows: Vec<ForgeQueryReadCompositionPhaseGateRow>,
    gate_digest: String,
}

impl ForgeQueryReadCompositionPhaseGate {
    pub fn derive(
        backend_posture: ForgeQueryRuntimeBackendPosture,
        support_matrix: &ForgeQueryRuntimePublicSupportMatrix,
        read_support: &ForgeQueryReadCompositionSupportReport,
        phase_one_closeout: &ForgeQueryReadCompositionPhaseOneCloseout,
    ) -> Self {
        let phase_two_start_family = "loop_cycle_neighborhood".to_string();
        let rows = vec![
            ForgeQueryReadCompositionPhaseGateRow::new(
                ForgeQueryReadCompositionPhaseGateFamily::PhaseOneKernelComplete,
                ForgeQueryReadCompositionPhaseGateStatus::Satisfied,
                "the generic read kernel is frozen with stable operators, typed denials, reusable read families, typed extension hooks, and descriptor-backed relationship-proof admission",
            ),
            ForgeQueryReadCompositionPhaseGateRow::new(
                ForgeQueryReadCompositionPhaseGateFamily::PhaseTwoWorthAdoptionReady,
                ForgeQueryReadCompositionPhaseGateStatus::Satisfied,
                "Worth may begin domain adoption through the frozen lowering, invariant-pack, decoder, and certification hooks, starting with loop_cycle_neighborhood",
            ),
            ForgeQueryReadCompositionPhaseGateRow::new(
                ForgeQueryReadCompositionPhaseGateFamily::PhaseThreeAggregateProofComplete,
                ForgeQueryReadCompositionPhaseGateStatus::Satisfied,
                "Worth topology now exposes aggregate query-native-versus-fallback breadth, debt, parity, and no-N-plus-one proof through its domain closeout surfaces",
            ),
            ForgeQueryReadCompositionPhaseGateRow::new(
                ForgeQueryReadCompositionPhaseGateFamily::MilestoneThreeResumeReady,
                ForgeQueryReadCompositionPhaseGateStatus::Satisfied,
                "Milestone 3 may resume through the Worth topology side-quest closeout gate because Phase 3 aggregate proof is now enforced outside the generic kernel",
            ),
        ];
        let mut parts = vec![
            "forge_query_read_composition_phase_gate_v1".to_string(),
            format!("posture:{}", backend_posture.as_str()),
            format!("matrix:{}", support_matrix.matrix_digest()),
            format!("read-support:{}", read_support.support_digest()),
            format!(
                "phase-one-closeout:{}",
                phase_one_closeout.closeout_digest()
            ),
            format!("phase-two-start-family:{phase_two_start_family}"),
        ];
        parts.extend(rows.iter().map(|row| row.row_digest().to_string()));
        let gate_digest = hash_parts(&parts);
        Self {
            backend_posture,
            support_matrix_digest: support_matrix.matrix_digest().to_string(),
            read_support_digest: read_support.support_digest().to_string(),
            phase_one_closeout_digest: phase_one_closeout.closeout_digest().to_string(),
            phase_two_start_family,
            rows,
            gate_digest,
        }
    }

    pub fn backend_posture(&self) -> ForgeQueryRuntimeBackendPosture {
        self.backend_posture
    }

    pub fn support_matrix_digest(&self) -> &str {
        &self.support_matrix_digest
    }

    pub fn read_support_digest(&self) -> &str {
        &self.read_support_digest
    }

    pub fn phase_one_closeout_digest(&self) -> &str {
        &self.phase_one_closeout_digest
    }

    pub fn phase_two_start_family(&self) -> &str {
        &self.phase_two_start_family
    }

    pub fn rows(&self) -> &[ForgeQueryReadCompositionPhaseGateRow] {
        &self.rows
    }

    pub fn gate_digest(&self) -> &str {
        &self.gate_digest
    }
}

impl ForgeQueryRuntime {
    pub fn public_read_composition_phase_gate_for_support_profile(
        support_profile: &ForgeQueryRuntimeSupportProfile,
    ) -> ForgeQueryReadCompositionPhaseGate {
        let public_api_contract =
            super::ForgeQueryRuntimePublicApiContract::from_support_profile(support_profile);
        let support_matrix =
            ForgeQueryRuntimePublicSupportMatrix::from_public_api_contract(&public_api_contract);
        let read_support =
            Self::public_read_composition_support_report_for_support_profile(support_profile);
        let phase_one_closeout =
            Self::public_read_composition_phase_one_closeout_for_support_profile(support_profile);
        ForgeQueryReadCompositionPhaseGate::derive(
            public_api_contract.backend_posture(),
            &support_matrix,
            &read_support,
            &phase_one_closeout,
        )
    }

    pub fn public_read_composition_phase_gate(&self) -> ForgeQueryReadCompositionPhaseGate {
        Self::public_read_composition_phase_gate_for_support_profile(
            &self.backend.support_profile(),
        )
    }
}
