use worth_spatial::facade::evidence_lookup_index_product::{
    EvidenceLookupIndexLifecyclePosture, EvidenceLookupIndexLifecyclePostureKind,
};

fn main() {
    let _ = EvidenceLookupIndexLifecyclePosture {
        kind: EvidenceLookupIndexLifecyclePostureKind::SparseLookupOnly,
    };
}
