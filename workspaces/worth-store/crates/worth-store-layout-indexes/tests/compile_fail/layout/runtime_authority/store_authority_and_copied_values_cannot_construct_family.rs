use worth_store_authority::StoreCurrentAuthorityWitness;
use worth_store_layout_indexes::AdmittedPhysicalArtifactFamily;

fn worth(authority: StoreCurrentAuthorityWitness) -> AdmittedPhysicalArtifactFamily {
    AdmittedPhysicalArtifactFamily {
        lifecycle: panic!(),
        security_identity: panic!(),
        authority_identity: authority.authority_identity(),
    }
}

fn main() {}
