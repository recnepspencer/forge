use forge_store_authority::StoreCurrentAuthorityWitness;
use forge_store_security::StoreSecurityWitnessVocabulary;

fn require_security_witness(_: StoreSecurityWitnessVocabulary) {}

fn main() {
    let authority: StoreCurrentAuthorityWitness = unimplemented!();
    require_security_witness(authority);
}
