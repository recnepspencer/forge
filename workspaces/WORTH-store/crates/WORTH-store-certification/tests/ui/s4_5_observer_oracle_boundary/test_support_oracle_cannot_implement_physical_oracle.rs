use worth_store_physical_certification::{
    OracleDenial, OracleFamilyKind, OracleVerdictBasis, PhysicalProofOracle,
    PhysicalProofOracleKind, PhysicalProofOracleVerdict,
};

struct TestSupportOracle;

impl PhysicalProofOracle for TestSupportOracle {
    fn oracle_kind(&self) -> PhysicalProofOracleKind {
        PhysicalProofOracleKind::NoMixedRoot
    }

    fn family_kind(&self) -> OracleFamilyKind {
        OracleFamilyKind::S5ReadinessShape
    }

    fn judge_basis(
        &self,
        _basis: OracleVerdictBasis,
    ) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
        unreachable!()
    }
}

fn main() {}
