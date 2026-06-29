use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::family_declaration::{
    BatchAdmissionAdvisoryWitnessShape, BatchAdmissionFamilyIdentity, BatchAdmissionFamilyPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BatchAdmissionPlanDenialKind {
    SelectedPlanDenied,
    MissingExplicitIndependenceProof,
    DeclaredDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchAdmissionPlanDenial {
    kind: BatchAdmissionPlanDenialKind,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchAdmissionPlanAdvisory {
    witness_shape: BatchAdmissionAdvisoryWitnessShape,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchAdmissionSelectedFamilyRow {
    identity: BatchAdmissionFamilyIdentity,
    declaration_digest: String,
    posture: BatchAdmissionFamilyPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchAdmissionPlanEdge {
    left_participant_identity: String,
    right_participant_identity: String,
    proof_digest: String,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum BatchAdmissionSupportingConflictLane {
    Topology,
    Spatial,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchAdmissionSupportingConflictFamilyRow {
    participant_identity: String,
    conflict_lane: BatchAdmissionSupportingConflictLane,
    conflict_family_identity: String,
    declaration_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedBatchAdmissionPlan {
    posture: BatchAdmissionFamilyPosture,
    authority_digests: Vec<String>,
    selected_conflict_plan_digests: Vec<String>,
    overlap_identity_digests: Vec<String>,
    locality_footprint_digests: Vec<String>,
    participant_identities: Vec<String>,
    selected_family_rows: Vec<BatchAdmissionSelectedFamilyRow>,
    supporting_conflict_family_rows: Vec<BatchAdmissionSupportingConflictFamilyRow>,
    parallel_admission_edges: Vec<BatchAdmissionPlanEdge>,
    serial_admission_edges: Vec<BatchAdmissionPlanEdge>,
    denied_proof_identities: Vec<String>,
    advisory: Option<BatchAdmissionPlanAdvisory>,
    denial: Option<BatchAdmissionPlanDenial>,
    selected_plan_digest: String,
}

impl BatchAdmissionPlanDenial {
    pub(crate) fn new(kind: BatchAdmissionPlanDenialKind, detail: &str) -> Self {
        Self {
            kind,
            detail: detail.to_string(),
        }
    }

    pub const fn kind(&self) -> BatchAdmissionPlanDenialKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl BatchAdmissionPlanAdvisory {
    pub(crate) fn new(witness_shape: BatchAdmissionAdvisoryWitnessShape, detail: &str) -> Self {
        Self {
            witness_shape,
            detail: detail.to_string(),
        }
    }

    pub const fn witness_shape(&self) -> BatchAdmissionAdvisoryWitnessShape {
        self.witness_shape
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl BatchAdmissionSelectedFamilyRow {
    pub(crate) fn new(
        identity: BatchAdmissionFamilyIdentity,
        declaration_digest: &str,
        posture: BatchAdmissionFamilyPosture,
    ) -> Self {
        Self {
            identity,
            declaration_digest: declaration_digest.to_string(),
            posture,
        }
    }

    pub const fn identity(&self) -> BatchAdmissionFamilyIdentity {
        self.identity
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub const fn posture(&self) -> BatchAdmissionFamilyPosture {
        self.posture
    }
}

impl BatchAdmissionPlanEdge {
    pub(crate) fn new(left: &str, right: &str, proof_digest: &str) -> Self {
        Self {
            left_participant_identity: left.to_string(),
            right_participant_identity: right.to_string(),
            proof_digest: proof_digest.to_string(),
        }
    }

    pub fn left_participant_identity(&self) -> &str {
        &self.left_participant_identity
    }

    pub fn right_participant_identity(&self) -> &str {
        &self.right_participant_identity
    }

    pub fn proof_digest(&self) -> &str {
        &self.proof_digest
    }
}

impl BatchAdmissionSupportingConflictLane {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Topology => "topology",
            Self::Spatial => "spatial",
        }
    }
}

impl BatchAdmissionSupportingConflictFamilyRow {
    pub(crate) fn new(
        participant_identity: &str,
        conflict_lane: BatchAdmissionSupportingConflictLane,
        conflict_family_identity: &str,
        declaration_digest: &str,
    ) -> Self {
        Self {
            participant_identity: participant_identity.to_string(),
            conflict_lane,
            conflict_family_identity: conflict_family_identity.to_string(),
            declaration_digest: declaration_digest.to_string(),
        }
    }

    pub fn participant_identity(&self) -> &str {
        &self.participant_identity
    }

    pub const fn conflict_lane(&self) -> BatchAdmissionSupportingConflictLane {
        self.conflict_lane
    }

    pub fn conflict_family_identity(&self) -> &str {
        &self.conflict_family_identity
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }
}

impl SelectedBatchAdmissionPlan {
    pub(crate) fn new(
        posture: BatchAdmissionFamilyPosture,
        mut authority_digests: Vec<String>,
        mut selected_conflict_plan_digests: Vec<String>,
        mut overlap_identity_digests: Vec<String>,
        mut locality_footprint_digests: Vec<String>,
        mut participant_identities: Vec<String>,
        mut selected_family_rows: Vec<BatchAdmissionSelectedFamilyRow>,
        mut supporting_conflict_family_rows: Vec<BatchAdmissionSupportingConflictFamilyRow>,
        mut parallel_admission_edges: Vec<BatchAdmissionPlanEdge>,
        mut serial_admission_edges: Vec<BatchAdmissionPlanEdge>,
        mut denied_proof_identities: Vec<String>,
        advisory: Option<BatchAdmissionPlanAdvisory>,
        denial: Option<BatchAdmissionPlanDenial>,
        grouped_input_digest: &str,
    ) -> Self {
        authority_digests.sort();
        authority_digests.dedup();
        selected_conflict_plan_digests.sort();
        selected_conflict_plan_digests.dedup();
        overlap_identity_digests.sort();
        overlap_identity_digests.dedup();
        locality_footprint_digests.sort();
        locality_footprint_digests.dedup();
        participant_identities.sort();
        selected_family_rows
            .sort_by_key(|row| format!("{}:{}", row.identity().as_str(), row.declaration_digest()));
        supporting_conflict_family_rows.sort_by_key(|row| {
            format!(
                "{}:{}:{}:{}",
                row.participant_identity(),
                row.conflict_lane().as_str(),
                row.conflict_family_identity(),
                row.declaration_digest()
            )
        });
        parallel_admission_edges.sort_by_key(|edge| {
            format!(
                "{}:{}:{}",
                edge.left_participant_identity(),
                edge.right_participant_identity(),
                edge.proof_digest()
            )
        });
        serial_admission_edges.sort_by_key(|edge| {
            format!(
                "{}:{}:{}",
                edge.left_participant_identity(),
                edge.right_participant_identity(),
                edge.proof_digest()
            )
        });
        denied_proof_identities.sort();
        let selected_plan_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &participant_identities
                .iter()
                .map(|identity| format!("participant:{identity}"))
                .chain(
                    authority_digests
                        .iter()
                        .map(|digest| format!("authority:{digest}")),
                )
                .chain(
                    selected_conflict_plan_digests
                        .iter()
                        .map(|digest| format!("selected-conflict:{digest}")),
                )
                .chain(
                    overlap_identity_digests
                        .iter()
                        .map(|digest| format!("overlap:{digest}")),
                )
                .chain(
                    locality_footprint_digests
                        .iter()
                        .map(|digest| format!("locality:{digest}")),
                )
                .chain(selected_family_rows.iter().map(|row| {
                    format!(
                        "family:{}:{}",
                        row.identity().as_str(),
                        row.declaration_digest()
                    )
                }))
                .chain(supporting_conflict_family_rows.iter().map(|row| {
                    format!(
                        "support:{}:{}:{}:{}",
                        row.participant_identity(),
                        row.conflict_lane().as_str(),
                        row.conflict_family_identity(),
                        row.declaration_digest()
                    )
                }))
                .chain(parallel_admission_edges.iter().map(|edge| {
                    format!(
                        "parallel:{}:{}:{}",
                        edge.left_participant_identity(),
                        edge.right_participant_identity(),
                        edge.proof_digest()
                    )
                }))
                .chain(serial_admission_edges.iter().map(|edge| {
                    format!(
                        "serial:{}:{}:{}",
                        edge.left_participant_identity(),
                        edge.right_participant_identity(),
                        edge.proof_digest()
                    )
                }))
                .chain(
                    denied_proof_identities
                        .iter()
                        .map(|proof_digest| format!("denied-proof:{proof_digest}")),
                )
                .chain(std::iter::once(format!("grouped:{grouped_input_digest}")))
                .chain(std::iter::once(format!("posture:{}", posture.as_str())))
                .chain(std::iter::once(format!(
                    "advisory:{}",
                    advisory
                        .as_ref()
                        .map(|row| row.witness_shape().as_str())
                        .unwrap_or("none")
                )))
                .chain(std::iter::once(format!(
                    "denial:{}",
                    denial
                        .as_ref()
                        .map(|row| match row.kind() {
                            BatchAdmissionPlanDenialKind::SelectedPlanDenied =>
                                "selected-plan-denied",
                            BatchAdmissionPlanDenialKind::MissingExplicitIndependenceProof =>
                                "missing-explicit-independence-proof",
                            BatchAdmissionPlanDenialKind::DeclaredDenied => "declared-denied",
                        })
                        .unwrap_or("none")
                )))
                .chain(std::iter::once(
                    "worth-kernel:selected-batch-admission-plan:v1".to_string(),
                ))
                .collect::<Vec<_>>(),
        );
        Self {
            posture,
            authority_digests,
            selected_conflict_plan_digests,
            overlap_identity_digests,
            locality_footprint_digests,
            participant_identities,
            selected_family_rows,
            supporting_conflict_family_rows,
            parallel_admission_edges,
            serial_admission_edges,
            denied_proof_identities,
            advisory,
            denial,
            selected_plan_digest,
        }
    }

    pub const fn posture(&self) -> BatchAdmissionFamilyPosture {
        self.posture
    }

    pub fn authority_digests(&self) -> &[String] {
        &self.authority_digests
    }

    pub fn selected_conflict_plan_digests(&self) -> &[String] {
        &self.selected_conflict_plan_digests
    }

    pub fn overlap_identity_digests(&self) -> &[String] {
        &self.overlap_identity_digests
    }

    pub fn locality_footprint_digests(&self) -> &[String] {
        &self.locality_footprint_digests
    }

    pub fn participant_identities(&self) -> &[String] {
        &self.participant_identities
    }

    pub fn selected_family_rows(&self) -> &[BatchAdmissionSelectedFamilyRow] {
        &self.selected_family_rows
    }

    pub fn supporting_conflict_family_rows(&self) -> &[BatchAdmissionSupportingConflictFamilyRow] {
        &self.supporting_conflict_family_rows
    }

    pub fn parallel_admission_edges(&self) -> &[BatchAdmissionPlanEdge] {
        &self.parallel_admission_edges
    }

    pub fn serial_admission_edges(&self) -> &[BatchAdmissionPlanEdge] {
        &self.serial_admission_edges
    }

    pub fn denied_proof_identities(&self) -> &[String] {
        &self.denied_proof_identities
    }

    pub fn advisory(&self) -> Option<&BatchAdmissionPlanAdvisory> {
        self.advisory.as_ref()
    }

    pub fn denial(&self) -> Option<&BatchAdmissionPlanDenial> {
        self.denial.as_ref()
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }
}
