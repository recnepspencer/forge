use worth_spatial::facade::evidence_lookup_family_catalog::{
    EvidenceLookupFamilyQueryPosture, EvidenceLookupFamilyQueryPostureKind,
};

fn main() {
    let _posture = EvidenceLookupFamilyQueryPosture {
        kind: EvidenceLookupFamilyQueryPostureKind::NotRequired,
        imported_evidence: None,
    };
}
