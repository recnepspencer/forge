use worth_foundational::facade::{
    AspectMaskLocator, AspectValueLocator, FoundationalBoundaryEvidenceProvenanceArtifact,
    ProjectionMask,
};
use worth_proof::{AuthorityMarker, AuthorityWitness};
use worth_query::facade::domain::WorthQueryFoundationalInvalidationBoundaryArtifact;

struct ForgedAuthority;
impl AuthorityMarker for ForgedAuthority {}

fn locator_consequence(value: AspectValueLocator) {
    value.attach_consumer_authored_consequence(())
}

fn mask_consequence(value: AspectMaskLocator<ProjectionMask>) {
    value.attach_consumer_authored_consequence(())
}

fn provenance_consequence(value: FoundationalBoundaryEvidenceProvenanceArtifact) {
    value.attach_consumer_authored_consequence(())
}

fn materialized_consequence(value: WorthQueryFoundationalInvalidationBoundaryArtifact) {
    value.attach_consumer_authored_consequence(())
}

fn generic_proof_consequence(value: AuthorityWitness<ForgedAuthority>) {
    value.attach_consumer_authored_consequence(())
}

fn main() {}
