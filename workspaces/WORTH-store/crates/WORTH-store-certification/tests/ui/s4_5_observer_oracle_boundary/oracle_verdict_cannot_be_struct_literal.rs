use worth_store_physical_certification::{
    OracleFamilyKind, PhysicalProofOracleKind, PhysicalProofOracleVerdict,
    PhysicalProofOracleVerdictKind,
};

fn main() {
    let _verdict = PhysicalProofOracleVerdict {
        family: OracleFamilyKind::S5ReadinessShape,
        oracle: PhysicalProofOracleKind::NoMixedRoot,
        kind: PhysicalProofOracleVerdictKind::Satisfied,
    };
}
