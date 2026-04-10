use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{BridgeIdentity, WritebackLoopPreventionIdentityTag};

use super::{
    BridgeDerivedWritebackEffect, BridgeWritebackFeedbackProvenance, BridgeWritebackIdempotenceBasis,
    BridgeWritebackLoopDisposition,
};

pub type BridgeWritebackLoopPreventionIdentity =
    BridgeIdentity<WritebackLoopPreventionIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackLoopPreventionReport {
    loop_prevention_identity: BridgeWritebackLoopPreventionIdentity,
    current_feedback_provenance_digest: Arc<str>,
    current_causality_digest: Arc<str>,
    incoming_feedback_provenance_digest: Option<Arc<str>>,
    incoming_feedback_causality_digest: Option<Arc<str>>,
    idempotence_digest: Arc<str>,
    disposition: BridgeWritebackLoopDisposition,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackLoopPreventionReport {
    pub fn classify(
        effect: &BridgeDerivedWritebackEffect,
        idempotence: &BridgeWritebackIdempotenceBasis,
        incoming_feedback_provenance_digest: Option<impl Into<Arc<str>>>,
        incoming_feedback_causality_digest: Option<impl Into<Arc<str>>>,
    ) -> Self {
        let current_feedback_provenance = BridgeWritebackFeedbackProvenance::new(effect);
        let incoming_feedback_provenance_digest =
            incoming_feedback_provenance_digest.map(Into::into);
        let incoming_feedback_causality_digest =
            incoming_feedback_causality_digest.map(Into::into);
        let disposition = classify_disposition(
            &current_feedback_provenance,
            idempotence,
            incoming_feedback_provenance_digest.as_deref(),
            incoming_feedback_causality_digest.as_deref(),
        );
        let current_feedback_provenance_digest =
            Arc::<str>::from(current_feedback_provenance.digest().to_owned());
        let current_causality_digest = Arc::<str>::from(effect.causality_digest().to_owned());
        let idempotence_digest = Arc::<str>::from(idempotence.digest().to_owned());
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-loop-prevention|feedback-provenance={}|causality={}|incoming-feedback={}|incoming-causality={}|idempotence={}|disposition:{disposition:?}",
            current_feedback_provenance_digest.as_ref(),
            current_causality_digest.as_ref(),
            incoming_feedback_provenance_digest
                .as_deref()
                .unwrap_or("none"),
            incoming_feedback_causality_digest
                .as_deref()
                .unwrap_or("none"),
            idempotence_digest.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            loop_prevention_identity: BridgeWritebackLoopPreventionIdentity::new(format!(
                "bridge-writeback-loop-prevention:sha256:{digest:x}"
            )),
            current_feedback_provenance_digest,
            current_causality_digest,
            incoming_feedback_provenance_digest,
            incoming_feedback_causality_digest,
            idempotence_digest,
            disposition,
            canonical_basis,
            digest: Arc::from(format!("bridge-writeback-loop-prevention:sha256:{digest:x}")),
        }
    }

    pub fn loop_prevention_identity(&self) -> &BridgeWritebackLoopPreventionIdentity {
        &self.loop_prevention_identity
    }

    pub fn current_feedback_provenance_digest(&self) -> &str {
        self.current_feedback_provenance_digest.as_ref()
    }

    pub fn current_causality_digest(&self) -> &str {
        self.current_causality_digest.as_ref()
    }

    pub fn incoming_feedback_provenance_digest(&self) -> Option<&str> {
        self.incoming_feedback_provenance_digest.as_deref()
    }

    pub fn incoming_feedback_causality_digest(&self) -> Option<&str> {
        self.incoming_feedback_causality_digest.as_deref()
    }

    pub fn idempotence_digest(&self) -> &str {
        self.idempotence_digest.as_ref()
    }

    pub fn disposition(&self) -> BridgeWritebackLoopDisposition {
        self.disposition
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

fn classify_disposition(
    current_feedback_provenance: &BridgeWritebackFeedbackProvenance,
    idempotence: &BridgeWritebackIdempotenceBasis,
    incoming_feedback_provenance_digest: Option<&str>,
    incoming_feedback_causality_digest: Option<&str>,
) -> BridgeWritebackLoopDisposition {
    match (
        incoming_feedback_provenance_digest,
        incoming_feedback_causality_digest,
    ) {
        (None, None) => BridgeWritebackLoopDisposition::AllowAuthoritativeAttempt,
        (Some(_), None) | (None, Some(_)) => BridgeWritebackLoopDisposition::RejectAsUnsafeFeedback,
        (Some(incoming_feedback_provenance_digest), Some(incoming_feedback_causality_digest)) => {
            let same_feedback = incoming_feedback_provenance_digest == current_feedback_provenance.digest();
            let same_causality = incoming_feedback_causality_digest == current_feedback_provenance.causality_digest();
            if same_feedback && same_causality {
                match idempotence.idempotence_class() {
                    crate::writeback::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression => {
                        BridgeWritebackLoopDisposition::CanonicalNoop
                    }
                    crate::writeback::BridgeWritebackIdempotenceClass::AllowRepeatedAuthorityAttempt => {
                        BridgeWritebackLoopDisposition::RejectAsUnsafeFeedback
                    }
                }
            } else if same_feedback || same_causality {
                BridgeWritebackLoopDisposition::RejectAsUnsafeFeedback
            } else {
                BridgeWritebackLoopDisposition::AllowAuthoritativeAttempt
            }
        }
    }
}
