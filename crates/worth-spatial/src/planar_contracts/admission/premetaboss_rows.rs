use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::{
    PlanarAdmissionClass, PlanarAdmissionFamily, PlanarAdmissionMatrix, PlanarAdmissionReason,
    PlanarPremetabossInputFamily, PlanarRuntimeConcern,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarPremetabossAdmissionRow {
    input_family: PlanarPremetabossInputFamily,
    family: PlanarAdmissionFamily,
    concern: PlanarRuntimeConcern,
    class: PlanarAdmissionClass,
    reason: PlanarAdmissionReason,
    movement_rotation_posture_class: PlanarAdmissionClass,
    row_digest: String,
}

impl PlanarPremetabossAdmissionRow {
    pub(super) fn new(
        input_family: PlanarPremetabossInputFamily,
        matrix: &PlanarAdmissionMatrix,
    ) -> Self {
        let (family, concern) = premetaboss_basis(input_family);
        let admission_row = matrix
            .row(family, concern)
            .expect("pre-MetaBoss admission rows must point at covered matrix rows");
        let movement_rotation_row = matrix
            .row(
                PlanarAdmissionFamily::MovementRotationPosture,
                PlanarRuntimeConcern::MovementRotationPosture,
            )
            .expect("pre-MetaBoss admission rows must point at movement posture row");
        let row_digest = hash_parts(&[
            format!("input_family:{}", input_family.as_str()),
            format!("matrix_row_digest:{}", admission_row.row_digest()),
            format!(
                "movement_rotation_row_digest:{}",
                movement_rotation_row.row_digest()
            ),
            format!("class:{}", admission_row.class().as_str()),
            format!("reason:{}", admission_row.reason().as_str()),
        ]);
        Self {
            input_family,
            family,
            concern,
            class: admission_row.class(),
            reason: admission_row.reason(),
            movement_rotation_posture_class: movement_rotation_row.class(),
            row_digest,
        }
    }

    pub fn input_family(&self) -> PlanarPremetabossInputFamily {
        self.input_family
    }

    pub fn family(&self) -> PlanarAdmissionFamily {
        self.family
    }

    pub fn concern(&self) -> PlanarRuntimeConcern {
        self.concern
    }

    pub fn class(&self) -> PlanarAdmissionClass {
        self.class
    }

    pub fn reason(&self) -> PlanarAdmissionReason {
        self.reason
    }

    pub fn movement_rotation_posture_class(&self) -> PlanarAdmissionClass {
        self.movement_rotation_posture_class
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

pub(super) fn premetaboss_admission_rows(
    matrix: &PlanarAdmissionMatrix,
) -> Vec<PlanarPremetabossAdmissionRow> {
    PlanarPremetabossInputFamily::all()
        .into_iter()
        .map(|input_family| PlanarPremetabossAdmissionRow::new(input_family, matrix))
        .collect()
}

fn premetaboss_basis(
    input_family: PlanarPremetabossInputFamily,
) -> (PlanarAdmissionFamily, PlanarRuntimeConcern) {
    match input_family {
        PlanarPremetabossInputFamily::CoplanarOverlapContractStorm => (
            PlanarAdmissionFamily::CoplanarOverlapContract,
            PlanarRuntimeConcern::CoplanarOverlapExtraction,
        ),
        PlanarPremetabossInputFamily::HighValencePlanarSingularityContract => (
            PlanarAdmissionFamily::ExactPlanarPredicateAuthority,
            PlanarRuntimeConcern::PredicateClassification,
        ),
        PlanarPremetabossInputFamily::ThinFeatureScaleSeparationContract => (
            PlanarAdmissionFamily::CertifiedSignedArea2d,
            PlanarRuntimeConcern::SignedAreaDegeneracy,
        ),
        PlanarPremetabossInputFamily::RetainedPlanarHistoryCancellationChain => (
            PlanarAdmissionFamily::RetainedPlanarFact,
            PlanarRuntimeConcern::RetainedFactReplay,
        ),
        PlanarPremetabossInputFamily::DirtyPlanarInputCleanFailLocalization => (
            PlanarAdmissionFamily::DirtyPlanarInput,
            PlanarRuntimeConcern::DiagnosticsLocalization,
        ),
        PlanarPremetabossInputFamily::UnboundedHalfSpacePlanarPosture => (
            PlanarAdmissionFamily::UnboundedPlanarDomain,
            PlanarRuntimeConcern::DiagnosticsLocalization,
        ),
        PlanarPremetabossInputFamily::ProjectionConsumedPlanarFactParity => (
            PlanarAdmissionFamily::ProjectionConsumedPlanarFact,
            PlanarRuntimeConcern::ProjectionConsumption,
        ),
        PlanarPremetabossInputFamily::BooleanReadinessFinalBoss => (
            PlanarAdmissionFamily::PlanarContractBundle,
            PlanarRuntimeConcern::BooleanReadinessBundle,
        ),
        PlanarPremetabossInputFamily::OpenRadialFanNmtTopology => (
            PlanarAdmissionFamily::ProjectionConsumedPlanarFact,
            PlanarRuntimeConcern::ProjectionConsumption,
        ),
        PlanarPremetabossInputFamily::MixedSurfaceKillBox => (
            PlanarAdmissionFamily::CertifiedPlaneProjection2d,
            PlanarRuntimeConcern::SupportMatrixAdmission,
        ),
        PlanarPremetabossInputFamily::OpenClassTriadParity => (
            PlanarAdmissionFamily::ProjectionConsumedPlanarFact,
            PlanarRuntimeConcern::ProjectionConsumption,
        ),
        PlanarPremetabossInputFamily::GrazingBasketStack => (
            PlanarAdmissionFamily::ProjectionConsumedPlanarFact,
            PlanarRuntimeConcern::ProjectionConsumption,
        ),
    }
}

fn hash_parts(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
