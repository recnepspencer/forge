use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::facade::{
    BridgeExecutionPolicyClass, BridgePolicyDeclaration, BridgePolicyDeclarationIdentity,
    BridgeRequestKind, BridgeWritebackDeclaration, BridgeWritebackDeclarationIdentity,
    BridgeWritebackEffectClass, BridgeWritebackEffectIntent, BridgeWritebackFamilyKind,
    BridgeWritebackIdempotenceClass, BridgeWritebackStrategyClass,
};
use crate::identity::{AsyncWritebackAdmissionIdentityTag, BridgeIdentity};

use super::super::{
    AdmittedBridgeAsyncCompletion, AdmittedBridgeAsyncRequestIdentity,
    BridgeAsyncRequestTruthViewBasis, BridgeAsyncRequestTruthViewBasisKind,
};
use super::{
    BridgeAsyncWritebackCounters, BridgeAsyncWritebackFamily, BridgeAsyncWritebackRejection,
    BridgeAsyncWritebackRejectionKind,
};

pub type BridgeAsyncWritebackAdmissionIdentity = BridgeIdentity<AsyncWritebackAdmissionIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAsyncWritebackAdmissionRequest {
    completion: AdmittedBridgeAsyncCompletion,
    family: BridgeAsyncWritebackFamily,
    effect_intent: BridgeWritebackEffectIntent,
    current_authoritative_truth_view_basis: BridgeAsyncRequestTruthViewBasis,
    current_authoritative_request: Option<AdmittedBridgeAsyncRequestIdentity>,
}

impl BridgeAsyncWritebackAdmissionRequest {
    pub fn authoritative_commit(
        completion: &AdmittedBridgeAsyncCompletion,
        effect_intent: BridgeWritebackEffectIntent,
        current_authoritative_truth_view_basis: BridgeAsyncRequestTruthViewBasis,
    ) -> Self {
        Self {
            completion: completion.clone(),
            family: BridgeAsyncWritebackFamily::AuthoritativeCommit,
            effect_intent,
            current_authoritative_truth_view_basis,
            current_authoritative_request: None,
        }
    }

    pub fn with_current_authoritative_request(
        mut self,
        request_identity: AdmittedBridgeAsyncRequestIdentity,
    ) -> Self {
        self.current_authoritative_request = Some(request_identity);
        self
    }

    pub fn completion(&self) -> &AdmittedBridgeAsyncCompletion {
        &self.completion
    }

    pub fn family(&self) -> BridgeAsyncWritebackFamily {
        self.family
    }

    pub fn effect_intent(&self) -> &BridgeWritebackEffectIntent {
        &self.effect_intent
    }

    pub fn current_authoritative_truth_view_basis(&self) -> &BridgeAsyncRequestTruthViewBasis {
        &self.current_authoritative_truth_view_basis
    }

    pub fn current_authoritative_request(&self) -> Option<&AdmittedBridgeAsyncRequestIdentity> {
        self.current_authoritative_request.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedBridgeAsyncWriteback {
    admission_identity: BridgeAsyncWritebackAdmissionIdentity,
    request: BridgeAsyncWritebackAdmissionRequest,
    writeback_declaration: BridgeWritebackDeclaration,
    policy_declaration: BridgePolicyDeclaration,
    counters: BridgeAsyncWritebackCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl AdmittedBridgeAsyncWriteback {
    pub(crate) fn admit(
        request: BridgeAsyncWritebackAdmissionRequest,
        diagnostics_tier: crate::policy::BridgeDiagnosticsTier,
        allow_replay_artifacts: bool,
        record_route_artifacts: bool,
    ) -> Result<Self, BridgeAsyncWritebackRejection> {
        let completion = request.completion();
        let truth_view_basis = completion
            .request_identity()
            .basis_binding()
            .truth_view_basis();
        let current_truth_view_basis = request.current_authoritative_truth_view_basis();
        match truth_view_basis.kind() {
            BridgeAsyncRequestTruthViewBasisKind::Preview => {
                return Err(BridgeAsyncWritebackRejection::new(
                    BridgeAsyncWritebackRejectionKind::PreviewCompletionForbidden,
                    format!(
                        "bridge async writeback cannot publish preview completion `{}` into authoritative truth",
                        completion.completion_identity()
                    ),
                ));
            }
            BridgeAsyncRequestTruthViewBasisKind::Authoritative => {}
            _ => {
                return Err(BridgeAsyncWritebackRejection::new(
                    BridgeAsyncWritebackRejectionKind::CompletionMustBeAuthoritative,
                    format!(
                        "bridge async writeback requires an authoritative truth-view basis for completion `{}`",
                        completion.completion_identity()
                    ),
                ));
            }
        }
        if current_truth_view_basis.kind() != BridgeAsyncRequestTruthViewBasisKind::Authoritative {
            return Err(BridgeAsyncWritebackRejection::new(
                BridgeAsyncWritebackRejectionKind::CompletionMustBeAuthoritative,
                format!(
                    "bridge async writeback requires an authoritative current truth-view basis for completion `{}`",
                    completion.completion_identity()
                ),
            ));
        }
        if current_truth_view_basis.digest() != truth_view_basis.digest() {
            return Err(BridgeAsyncWritebackRejection::new(
                BridgeAsyncWritebackRejectionKind::CurrentAuthorityDrifted,
                format!(
                    "bridge async writeback completion `{}` is no longer current because truth-view basis `{}` displaced `{}`",
                    completion.completion_identity(),
                    current_truth_view_basis.digest(),
                    truth_view_basis.digest(),
                ),
            ));
        }
        if let Some(current) = request.current_authoritative_request() {
            if current.basis_binding().truth_view_basis().digest()
                != current_truth_view_basis.digest()
            {
                return Err(BridgeAsyncWritebackRejection::new(
                    BridgeAsyncWritebackRejectionKind::CurrentAuthorityDrifted,
                    format!(
                        "bridge async writeback current authority proof mismatched truth-view basis for completion `{}`",
                        completion.completion_identity(),
                    ),
                ));
            }
            if current.request_identity() != completion.request_identity().request_identity() {
                return Err(BridgeAsyncWritebackRejection::new(
                    BridgeAsyncWritebackRejectionKind::CurrentAuthorityDrifted,
                    format!(
                        "bridge async writeback completion `{}` is no longer current because request `{}` displaced `{}`",
                        completion.completion_identity(),
                        current.request_identity().as_str(),
                        completion.request_identity().request_identity().as_str(),
                    ),
                ));
            }
        }
        if request.effect_intent().effect_class() != BridgeWritebackEffectClass::ProjectedStateDiff
        {
            return Err(BridgeAsyncWritebackRejection::new(
                BridgeAsyncWritebackRejectionKind::MapperEffectClassUnsupported,
                format!(
                    "bridge async writeback family `{:?}` only admits `ProjectedStateDiff` effect intents in phase 10",
                    request.family()
                ),
            ));
        }
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-async-writeback-admission|completion={}|request={}|family={:?}|effect-intent={}|effect-class={:?}|current-authority={}",
            completion.completion_identity(),
            completion.request_identity().request_identity().as_str(),
            request.family(),
            request.effect_intent().digest(),
            request.effect_intent().effect_class(),
            request.current_authoritative_truth_view_basis().digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        let admission_identity = BridgeAsyncWritebackAdmissionIdentity::new(format!(
            "bridge-async-writeback-admission-id:sha256:{digest:x}"
        ));
        let writeback_declaration = BridgeWritebackDeclaration::writeback_capable(
            BridgeWritebackDeclarationIdentity::new(format!(
                "bridge-async-writeback-declaration:{}",
                admission_identity.as_str()
            )),
            BridgeRequestKind::Authoritative,
            BridgeWritebackFamilyKind::ProjectedStateDiff,
            BridgeWritebackEffectClass::ProjectedStateDiff,
            BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
            BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
        );
        let policy_declaration = BridgePolicyDeclaration::new(
            BridgePolicyDeclarationIdentity::new(format!(
                "bridge-async-writeback-policy:{}",
                admission_identity.as_str()
            )),
            BridgeRequestKind::Authoritative,
            BridgeExecutionPolicyClass::DeterministicCanonical,
            diagnostics_tier,
            allow_replay_artifacts,
            record_route_artifacts,
        );

        Ok(Self {
            admission_identity,
            request,
            writeback_declaration,
            policy_declaration,
            counters: BridgeAsyncWritebackCounters::admitted(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-async-writeback-admission:sha256:{digest:x}"
            )),
        })
    }

    pub fn admission_identity(&self) -> &BridgeAsyncWritebackAdmissionIdentity {
        &self.admission_identity
    }

    pub fn completion(&self) -> &AdmittedBridgeAsyncCompletion {
        self.request.completion()
    }

    pub fn request_identity(&self) -> &AdmittedBridgeAsyncRequestIdentity {
        self.request.completion().request_identity()
    }

    pub fn family(&self) -> BridgeAsyncWritebackFamily {
        self.request.family()
    }

    pub fn effect_intent(&self) -> &BridgeWritebackEffectIntent {
        self.request.effect_intent()
    }

    pub fn current_authoritative_request(&self) -> Option<&AdmittedBridgeAsyncRequestIdentity> {
        self.request.current_authoritative_request()
    }

    pub fn current_authoritative_truth_view_basis(&self) -> &BridgeAsyncRequestTruthViewBasis {
        self.request.current_authoritative_truth_view_basis()
    }

    pub fn writeback_declaration(&self) -> &BridgeWritebackDeclaration {
        &self.writeback_declaration
    }

    pub fn policy_declaration(&self) -> &BridgePolicyDeclaration {
        &self.policy_declaration
    }

    pub fn counters(&self) -> &BridgeAsyncWritebackCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
