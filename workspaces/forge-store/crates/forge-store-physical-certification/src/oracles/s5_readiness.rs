use crate::OracleFamilyKind;

use super::{
    OracleDenial, OracleVerdictBasis, PhysicalOracleNonClaim, PhysicalProofOracle,
    PhysicalProofOracleKind, PhysicalProofOracleVerdict,
};

macro_rules! readiness_oracle {
    ($name:ident, $kind:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name;

        impl PhysicalProofOracle for $name {
            fn oracle_kind(&self) -> PhysicalProofOracleKind {
                PhysicalProofOracleKind::$kind
            }

            fn family_kind(&self) -> OracleFamilyKind {
                OracleFamilyKind::S5ReadinessShape
            }

            fn judge_basis(
                &self,
                basis: OracleVerdictBasis,
            ) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
                Ok(PhysicalProofOracleVerdict::satisfied(
                    OracleFamilyKind::S5ReadinessShape,
                    self.oracle_kind(),
                    basis,
                    [PhysicalOracleNonClaim::S5PhysicalIsolationCorrectness],
                ))
            }
        }
    };
}

readiness_oracle!(NoMixedRootOracle, NoMixedRoot);
readiness_oracle!(OldReaderSeesOldRootOracle, OldReaderSeesOldRoot);
readiness_oracle!(PostSwapReaderSeesNewRootOracle, PostSwapReaderSeesNewRoot);
readiness_oracle!(BlockedReclaimUntilReleaseOracle, BlockedReclaimUntilRelease);
