use crate::{CertifiedPhysicalScenario, PhysicalSimulationPlan, PhysicalSimulationProfile};
use worth_store_budgets::BlobHarnessEnvelopeProfile;

use super::CoverageRowDimension;

pub(super) fn append_blob_plan_dimensions(
    dimensions: &mut Vec<CoverageRowDimension>,
    plan: &PhysicalSimulationPlan,
) {
    if let Some(metadata) = plan.s7_blob_harness_metadata() {
        dimensions.push(CoverageRowDimension::BlobSizeClass(metadata.size_class()));
        dimensions.push(CoverageRowDimension::BlobChunkSizeClass(
            metadata.chunk_size_class(),
        ));
        dimensions.push(CoverageRowDimension::BlobPlacementClass(
            metadata.placement_class(),
        ));
        dimensions.push(CoverageRowDimension::BlobSecurityScopeClass(
            metadata.security_scope_class(),
        ));
        dimensions.push(CoverageRowDimension::BlobAccessMode(metadata.access_mode()));
        dimensions.push(CoverageRowDimension::BlobFailurePoint(
            metadata.failure_point(),
        ));
        dimensions.push(CoverageRowDimension::BlobActorMix(metadata.actor_mix()));
        dimensions.push(CoverageRowDimension::BlobMemoryEnvelopeProfile(
            blob_envelope_profile_for_plan(plan.profile()),
        ));
    }
    if let Some(topology) = plan.s7_blob_harness_topology() {
        dimensions.push(CoverageRowDimension::BlobChunkCount(topology.chunk_count()));
    }
}

pub(super) fn blob_scenario_dimensions(
    scenario: &CertifiedPhysicalScenario,
) -> impl Iterator<Item = CoverageRowDimension> + '_ {
    let metadata = scenario
        .definition()
        .expectation()
        .s7_blob_harness_metadata();
    let topology = scenario
        .definition()
        .expectation()
        .s7_blob_harness_topology();

    metadata.into_iter().flat_map(move |metadata| {
        let mut dimensions = vec![
            CoverageRowDimension::BlobSizeClass(metadata.size_class()),
            CoverageRowDimension::BlobChunkSizeClass(metadata.chunk_size_class()),
            CoverageRowDimension::BlobPlacementClass(metadata.placement_class()),
            CoverageRowDimension::BlobSecurityScopeClass(metadata.security_scope_class()),
            CoverageRowDimension::BlobAccessMode(metadata.access_mode()),
            CoverageRowDimension::BlobFailurePoint(metadata.failure_point()),
            CoverageRowDimension::BlobActorMix(metadata.actor_mix()),
        ];
        if let Some(topology) = topology {
            dimensions.push(CoverageRowDimension::BlobChunkCount(topology.chunk_count()));
        }
        dimensions.into_iter()
    })
}

const fn blob_envelope_profile_for_plan(
    profile: PhysicalSimulationProfile,
) -> BlobHarnessEnvelopeProfile {
    match profile {
        PhysicalSimulationProfile::DeveloperSmoke | PhysicalSimulationProfile::LocalSoak => {
            BlobHarnessEnvelopeProfile::Local
        }
        PhysicalSimulationProfile::CiCertification => {
            BlobHarnessEnvelopeProfile::CiMemoryEnvelopeExceeding
        }
        PhysicalSimulationProfile::ReleaseCertification
        | PhysicalSimulationProfile::HardwareQualification => {
            BlobHarnessEnvelopeProfile::HeavyMultiGb
        }
    }
}
