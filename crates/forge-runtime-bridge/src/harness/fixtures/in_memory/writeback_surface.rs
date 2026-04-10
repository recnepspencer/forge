use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use crate::adapter::{
    TruthWritebackAuthority, TruthWritebackAuthorityError, TruthWritebackReceipt,
    TruthWritebackRequest,
};
use crate::routing::canonicalization::digest_string;
use crate::writeback::BridgeWritebackOutcomeClass;

#[derive(Debug, Clone, Default)]
struct RecordingTruthWritebackState {
    first_commit_by_family_and_causality: BTreeMap<String, String>,
    request_digests: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RecordingTruthWritebackAuthority {
    state: Arc<RwLock<RecordingTruthWritebackState>>,
}

impl RecordingTruthWritebackAuthority {
    pub fn committed_causality_count(&self) -> usize {
        self.state
            .read()
            .expect("writeback authority lock poisoned")
            .first_commit_by_family_and_causality
            .len()
    }
}

impl TruthWritebackAuthority for RecordingTruthWritebackAuthority {
    fn execute_writeback(
        &self,
        request: TruthWritebackRequest,
    ) -> Result<TruthWritebackReceipt, TruthWritebackAuthorityError> {
        let mut state = self
            .state
            .write()
            .expect("writeback authority lock poisoned");
        state.request_digests.push(request.digest().to_string());

        let authority_key = format!(
            "family:{:?}|causality={}",
            request.family_kind(),
            request.causality_digest()
        );

        if let Some(existing_artifact) = state
            .first_commit_by_family_and_causality
            .get(authority_key.as_str())
            .cloned()
        {
            return Ok(TruthWritebackReceipt::new(
                BridgeWritebackOutcomeClass::CanonicalNoop,
                existing_artifact,
                &request,
            ));
        }

        let authoritative_artifact_digest = digest_string(
            "recording-truth-writeback-authority",
            &format!(
                "candidate={}|causality={}|effect={}|family:{:?}|effect-class:{:?}|strategy-class:{:?}|mapper-witness={}|loop-prevention={}|loop-disposition:{:?}|strategy-compatibility={}|idempotence={}|idempotence-class:{:?}",
                request.candidate_digest(),
                request.causality_digest(),
                request.proposed_effect_digest(),
                request.family_kind(),
                request.effect_class(),
                request.strategy_class(),
                request.mapper_witness_digest(),
                request.loop_prevention_digest(),
                request.loop_prevention_disposition(),
                request.strategy_compatibility_digest(),
                request.idempotence_digest(),
                request.idempotence_class(),
            ),
        )
        .to_string();
        state.first_commit_by_family_and_causality.insert(
            authority_key,
            authoritative_artifact_digest.clone(),
        );

        Ok(TruthWritebackReceipt::new(
            BridgeWritebackOutcomeClass::AuthoritativeCommit,
            authoritative_artifact_digest,
            &request,
        ))
    }
}
