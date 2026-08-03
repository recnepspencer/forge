use sha2::{Digest, Sha256};

use super::{ScheduleDecision, ScheduleSeed};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) struct ScheduleDecisionTrace {
    decisions: [ScheduleDecision; 4],
    digest: [u8; 32],
}

impl ScheduleDecisionTrace {
    pub(in crate::courtroom_campaign::bounded_residency_siege) fn new(
        seed: ScheduleSeed,
        decisions: [ScheduleDecision; 4],
    ) -> Self {
        let mut digest = Sha256::new();
        digest.update(b"worth-store-c6-schedule-trace-v1");
        digest.update(seed.value().to_le_bytes());
        for decision in decisions {
            update_text(&mut digest, decision.family());
            update_text(&mut digest, decision.choice());
        }
        Self {
            decisions,
            digest: digest.finalize().into(),
        }
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn decisions(
        &self,
    ) -> &[ScheduleDecision; 4] {
        &self.decisions
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn digest(&self) -> [u8; 32] {
        self.digest
    }
}

fn update_text(digest: &mut Sha256, text: &str) {
    digest.update((text.len() as u64).to_le_bytes());
    digest.update(text.as_bytes());
}
