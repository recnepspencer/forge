use forge_store_security::{
    StoreCurrentCustodyScopeWitness, StoreCustodyPosture, StoreSecurityScopeIdentity,
};

fn main() {
    let _forged = StoreCurrentCustodyScopeWitness {
        identity: unimplemented!(),
        custody_posture: StoreCustodyPosture::InternalStoreCustody,
    };
}
