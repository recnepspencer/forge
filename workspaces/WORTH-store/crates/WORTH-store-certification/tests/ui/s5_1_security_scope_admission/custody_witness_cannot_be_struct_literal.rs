use worth_store_security::{
    StoreCurrentCustodyScopeWitness, StoreCustodyPosture, StoreSecurityScopeIdentity,
};

fn main() {
    let _WORTHd = StoreCurrentCustodyScopeWitness {
        identity: unimplemented!(),
        custody_posture: StoreCustodyPosture::InternalStoreCustody,
    };
}
