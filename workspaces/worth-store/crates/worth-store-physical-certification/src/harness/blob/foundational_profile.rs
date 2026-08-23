use worth_foundational::canonicalization_api::lower_lane::basis::CanonicalizationRuleVersion;
use worth_foundational::{
    derive_foundational_profile_identity, profiles, AdmissionReadinessProfile,
    CertificationPostureProfile, CompatibilityPostureProfile, DiagnosticRichnessProfile,
    ExecutionObjectiveProfile, FoundationalProfileIdentity, FoundationalProfileSet,
    MaterializedFoundationalProfileArtifact, ObservationActivationProfile,
    RetentionDeliveryProfile, SupportPostureProfile,
};
use worth_proof::TransitionOutcome;

use super::profile::BlobHarnessProfile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobHarnessMaterializedProfile {
    blob_profile: BlobHarnessProfile,
    foundational_identity: FoundationalProfileIdentity,
    materialized: MaterializedFoundationalProfileArtifact,
}

impl BlobHarnessMaterializedProfile {
    pub fn for_blob_profile(blob_profile: BlobHarnessProfile) -> Self {
        let requested = profiles()
            .set()
            .diagnostic_richness(diagnostic_richness(blob_profile))
            .support_posture(support_posture(blob_profile))
            .compatibility_posture(CompatibilityPostureProfile::NativeOnly)
            .admission_readiness(admission_readiness(blob_profile))
            .retention_delivery(retention_delivery(blob_profile))
            .certification_posture(CertificationPostureProfile::EvidenceBacked)
            .execution_objective(execution_objective(blob_profile))
            .observation_activation(observation_activation(blob_profile))
            .request()
            .expect("S.7 blob harness profile families are complete");
        let admitted_profile = *requested.payload().requested();
        let admitted = match profiles().progression().admit_same(requested) {
            TransitionOutcome::Success(admitted) => admitted,
            outcome => panic!("S.7 blob harness profile admission should succeed: {outcome:?}"),
        };
        let foundational_identity =
            match derive_foundational_profile_identity(profile_identity_version(), &admitted) {
                TransitionOutcome::Success(identity) => identity,
                outcome => {
                    panic!("S.7 blob harness profile identity should derive: {outcome:?}")
                }
            };
        let materialized = materialize_admitted_profile(admitted, admitted_profile);
        Self {
            blob_profile,
            foundational_identity,
            materialized,
        }
    }

    pub const fn blob_profile(&self) -> BlobHarnessProfile {
        self.blob_profile
    }

    pub const fn foundational_identity(&self) -> &FoundationalProfileIdentity {
        &self.foundational_identity
    }

    pub const fn materialized(&self) -> &MaterializedFoundationalProfileArtifact {
        &self.materialized
    }
}

fn materialize_admitted_profile(
    admitted: worth_foundational::AdmittedFoundationalProfileArtifact,
    admitted_profile: FoundationalProfileSet,
) -> MaterializedFoundationalProfileArtifact {
    match profiles()
        .progression()
        .materialize_as(admitted, admitted_profile, None)
    {
        TransitionOutcome::Success(materialized) => materialized,
        outcome => panic!("S.7 blob harness profile materialization should succeed: {outcome:?}"),
    }
}

fn profile_identity_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("s7.blob-harness.profile.identity")
        .expect("static S.7 profile identity version is valid")
}

const fn diagnostic_richness(profile: BlobHarnessProfile) -> DiagnosticRichnessProfile {
    match profile {
        BlobHarnessProfile::Local => DiagnosticRichnessProfile::Standard,
        BlobHarnessProfile::CiMemoryEnvelopeExceeding | BlobHarnessProfile::HeavyMultiGb => {
            DiagnosticRichnessProfile::Forensic
        }
    }
}

const fn support_posture(profile: BlobHarnessProfile) -> SupportPostureProfile {
    match profile {
        BlobHarnessProfile::Local => SupportPostureProfile::SupportReady,
        BlobHarnessProfile::CiMemoryEnvelopeExceeding | BlobHarnessProfile::HeavyMultiGb => {
            SupportPostureProfile::CertificationReady
        }
    }
}

const fn admission_readiness(profile: BlobHarnessProfile) -> AdmissionReadinessProfile {
    match profile {
        BlobHarnessProfile::Local => AdmissionReadinessProfile::Admitted,
        BlobHarnessProfile::CiMemoryEnvelopeExceeding | BlobHarnessProfile::HeavyMultiGb => {
            AdmissionReadinessProfile::ProductionGateReady
        }
    }
}

const fn retention_delivery(profile: BlobHarnessProfile) -> RetentionDeliveryProfile {
    match profile {
        BlobHarnessProfile::Local => RetentionDeliveryProfile::Retained,
        BlobHarnessProfile::CiMemoryEnvelopeExceeding => RetentionDeliveryProfile::Retained,
        BlobHarnessProfile::HeavyMultiGb => RetentionDeliveryProfile::Durable,
    }
}

const fn execution_objective(profile: BlobHarnessProfile) -> ExecutionObjectiveProfile {
    match profile {
        BlobHarnessProfile::Local | BlobHarnessProfile::CiMemoryEnvelopeExceeding => {
            ExecutionObjectiveProfile::Balanced
        }
        BlobHarnessProfile::HeavyMultiGb => ExecutionObjectiveProfile::Throughput,
    }
}

const fn observation_activation(profile: BlobHarnessProfile) -> ObservationActivationProfile {
    match profile {
        BlobHarnessProfile::Local
        | BlobHarnessProfile::CiMemoryEnvelopeExceeding
        | BlobHarnessProfile::HeavyMultiGb => ObservationActivationProfile::Continuous,
    }
}
