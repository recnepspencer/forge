use worth_foundational::facade::{
    AspectMaskLocator, AspectValueLocator, FoundationalBoundaryEvidenceProvenanceArtifact,
    ProjectionMask,
};
use worth_proof::{AuthorityMarker, AuthorityWitness};
use worth_query::facade::domain::{
    WorthQueryFoundationalInvalidationBoundaryArtifact, WorthQuerySharedLiveProjectionLease,
};
use worth_query::facade::foundation::ObservationLaneWitness;
use worth_query::facade::runtime::WorthQueryWorkspace;

struct ForgedAuthority;
impl AuthorityMarker for ForgedAuthority {}

type Lease = WorthQuerySharedLiveProjectionLease<(), (), (), ObservationLaneWitness>;

fn copied_evidence_cannot_admit(
    lease: &Lease,
    workspace: &WorthQueryWorkspace,
    locator: AspectValueLocator,
    mask: AspectMaskLocator<ProjectionMask>,
    provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    materialized: WorthQueryFoundationalInvalidationBoundaryArtifact,
    generic_proof: AuthorityWitness<ForgedAuthority>,
) {
    lease.admit_consumer_invalidation_delta(locator, workspace);
    lease.admit_consumer_invalidation_delta(mask, workspace);
    lease.admit_consumer_invalidation_delta(provenance, workspace);
    lease.admit_consumer_invalidation_delta(materialized, workspace);
    lease.admit_consumer_invalidation_delta(generic_proof, workspace);
}

fn main() {}
