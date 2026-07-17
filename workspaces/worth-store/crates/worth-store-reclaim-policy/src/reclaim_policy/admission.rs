use worth_store_physical_format::ReclaimedByteInterpretation;

use super::{
    AdmittedReclaimPolicy, ReclaimPolicyCounterSnapshot, ReclaimPolicyDenial,
    ReclaimPolicyDenialKind, ReclaimPolicyProofAuthority, ReclaimPolicyRequest,
};

pub struct ReclaimPolicyAdmission;

impl ReclaimPolicyAdmission {
    pub fn admit(
        authority: ReclaimPolicyProofAuthority,
        request: ReclaimPolicyRequest,
    ) -> Result<AdmittedReclaimPolicy, ReclaimPolicyDenial> {
        let mut counters = ReclaimPolicyCounterSnapshot::start_request();
        let region = request.region().ok_or_else(|| {
            ReclaimPolicyDenial::new(
                ReclaimPolicyDenialKind::MissingPhysicalRegion,
                counters.with_denied(),
            )
        })?;
        let posture = request.posture().ok_or_else(|| {
            ReclaimPolicyDenial::new(
                ReclaimPolicyDenialKind::MissingPosture,
                counters.with_denied(),
            )
        })?;
        if posture.backend_profile() != authority.backend().profile()
            || posture.evidence_class() != authority.backend().evidence_class()
            || posture.media_assumptions() != authority.backend().media_assumptions()
        {
            return Err(ReclaimPolicyDenial::new(
                ReclaimPolicyDenialKind::UnsupportedBackendPosture,
                counters.with_denied(),
            ));
        }
        if posture.interpretation() == ReclaimedByteInterpretation::PlatformGradeDenied {
            return Err(ReclaimPolicyDenial::new(
                ReclaimPolicyDenialKind::PlatformGradeDenied,
                counters.with_denied(),
            ));
        }
        let reachability = request.reachability().ok_or_else(|| {
            ReclaimPolicyDenial::new(
                ReclaimPolicyDenialKind::MissingProtectedReachability,
                counters.with_denied(),
            )
        })?;
        counters = counters.with_protected_reachability_check();
        if !reachability.is_eligible() {
            return Err(ReclaimPolicyDenial::new(
                ReclaimPolicyDenialKind::ProtectedReachabilityBlocked,
                counters.with_denied(),
            ));
        }
        let security_scope = request.security_scope().ok_or_else(|| {
            ReclaimPolicyDenial::new(
                ReclaimPolicyDenialKind::MissingSecurityScope,
                counters.with_denied(),
            )
        })?;
        counters = counters.with_security_scope_check();
        let permit = request.permit().ok_or_else(|| {
            ReclaimPolicyDenial::new(
                ReclaimPolicyDenialKind::MissingReclaimPermit,
                counters.with_denied(),
            )
        })?;
        if !request.handoff_policy().is_non_claim() {
            return Err(ReclaimPolicyDenial::new(
                ReclaimPolicyDenialKind::LaterLifecycleClaimAttempted,
                counters.with_denied(),
            ));
        }
        counters = counters.with_non_claim_handoff().with_admitted();
        Ok(AdmittedReclaimPolicy::new(
            super::AdmittedReclaimPolicyBasis {
                backend: authority.backend(),
                region,
                posture,
                reachability: reachability.clone(),
                security_scope,
                permit,
                handoff_policy: request.handoff_policy(),
            },
            counters,
        ))
    }
}
