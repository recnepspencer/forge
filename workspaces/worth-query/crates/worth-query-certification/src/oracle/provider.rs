use crate::evidence::{WorthQueryCertificationDenialEvidence, WorthQueryCertificationObservation};
use crate::scenario::{WorthQueryCertificationHostileAttack, WorthQueryCertificationScenario};

/// Adapter implemented by an admitted provider's certification fixture.
///
/// The adapter reports provider-neutral semantics. The domain owns execution
/// and its independent oracle; Query certification owns comparison.
pub trait WorthQueryCertificationProvider {
    fn provider_identity(&self) -> &str;

    fn execute(
        &mut self,
        scenario: &WorthQueryCertificationScenario,
    ) -> Result<WorthQueryCertificationObservation, String>;
}

/// Query-owned hostile-world adapter. Downstream semantic providers do not
/// reproduce this generic matrix.
pub trait WorthQueryHostileCertificationProvider {
    fn provider_identity(&self) -> &str;

    fn attack(
        &mut self,
        attack: WorthQueryCertificationHostileAttack,
    ) -> Result<WorthQueryCertificationDenialEvidence, String>;
}
