use forge_store_authority::StoreCurrentAuthorityWitness;
use forge_store_layout_indexes::AdmittedPhysicalArtifactFamily;

fn forge(authority: StoreCurrentAuthorityWitness) -> AdmittedPhysicalArtifactFamily {
    AdmittedPhysicalArtifactFamily {
        lifecycle: panic!(),
        security_identity: panic!(),
        authority_identity: authority.authority_identity(),
    }
}

fn main() {}
