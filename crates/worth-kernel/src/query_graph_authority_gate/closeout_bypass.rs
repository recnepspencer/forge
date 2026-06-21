use std::collections::HashSet;
use std::path::Path;

use super::closeout_report::WorthGraphAuthorityCloseoutViolation;
use super::closeout_types::{
    WorthGraphAuthorityCloseoutBypassClass, WorthGraphAuthorityCloseoutBypassEvidence,
};
use super::gate_report_types::WorthGraphAuthorityGateReport;
use super::types::{WorthLowerAuthorityPromotionCase, WorthLowerAuthorityPromotionGuardPlan};

const BYPASS_GUARD_BINDINGS: [(
    WorthGraphAuthorityCloseoutBypassClass,
    WorthLowerAuthorityPromotionCase,
); 6] = [
    (
        WorthGraphAuthorityCloseoutBypassClass::SyntheticProof,
        WorthLowerAuthorityPromotionCase::SyntheticFixtureNotProductionProof,
    ),
    (
        WorthGraphAuthorityCloseoutBypassClass::LocalSupportPin,
        WorthLowerAuthorityPromotionCase::SupportReportNotGraphAuthority,
    ),
    (
        WorthGraphAuthorityCloseoutBypassClass::CopiedRows,
        WorthLowerAuthorityPromotionCase::SplitDigestNotLoopDigest,
    ),
    (
        WorthGraphAuthorityCloseoutBypassClass::HandoffOnlyReceipt,
        WorthLowerAuthorityPromotionCase::HandoffNotExecutedBirth,
    ),
    (
        WorthGraphAuthorityCloseoutBypassClass::RawEvidenceVector,
        WorthLowerAuthorityPromotionCase::RawEvidenceNotStageIndex,
    ),
    (
        WorthGraphAuthorityCloseoutBypassClass::StringStageLink,
        WorthLowerAuthorityPromotionCase::StringPrefixNotTypedStageLink,
    ),
];

pub(crate) fn closeout_bypass_evidence_from_gate(
    gate: &WorthGraphAuthorityGateReport,
) -> Result<Vec<WorthGraphAuthorityCloseoutBypassEvidence>, WorthGraphAuthorityCloseoutViolation> {
    let mut evidence = Vec::with_capacity(BYPASS_GUARD_BINDINGS.len());
    for (bypass_class, promotion_case) in BYPASS_GUARD_BINDINGS {
        let guard = guard_plan_for(gate.lower_authority_guard_plan(), promotion_case)?;
        validate_compile_fail_fixture(guard)?;
        evidence.push(WorthGraphAuthorityCloseoutBypassEvidence::new(
            bypass_class,
            promotion_case,
            guard.planned_compile_fail_path(),
            guard.lower_authority_surface(),
            guard.required_authority_type(),
        ));
    }
    validate_bypass_evidence_complete(&evidence)?;
    Ok(evidence)
}

pub(crate) fn validate_bypass_evidence_complete(
    evidence: &[WorthGraphAuthorityCloseoutBypassEvidence],
) -> Result<(), WorthGraphAuthorityCloseoutViolation> {
    let classes: HashSet<_> = evidence.iter().map(|row| row.bypass_class()).collect();
    for bypass_class in WorthGraphAuthorityCloseoutBypassClass::ALL {
        if !classes.contains(&bypass_class) {
            return Err(WorthGraphAuthorityCloseoutViolation::MissingBypassRejection(bypass_class));
        }
    }
    Ok(())
}

fn guard_plan_for(
    guards: &[WorthLowerAuthorityPromotionGuardPlan],
    promotion_case: WorthLowerAuthorityPromotionCase,
) -> Result<&WorthLowerAuthorityPromotionGuardPlan, WorthGraphAuthorityCloseoutViolation> {
    guards
        .iter()
        .find(|guard| guard.promotion_case() == promotion_case)
        .ok_or(WorthGraphAuthorityCloseoutViolation::MissingLowerAuthorityGuard(promotion_case))
}

fn validate_compile_fail_fixture(
    guard: &WorthLowerAuthorityPromotionGuardPlan,
) -> Result<(), WorthGraphAuthorityCloseoutViolation> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(guard.planned_compile_fail_path());
    if path.is_file() {
        Ok(())
    } else {
        Err(
            WorthGraphAuthorityCloseoutViolation::MissingLowerAuthorityFixture(
                guard.planned_compile_fail_path(),
            ),
        )
    }
}
