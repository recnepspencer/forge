use std::collections::HashMap;

use worth_store::physical_runtime::{PhysicalWorkDrainObservation, PhysicalWorkIdentity};

use super::PhysicalWorkTerminalFateEvidence;

pub(super) struct TerminalIdentityIndex {
    terminal: HashMap<PhysicalWorkIdentity, PhysicalWorkTerminalFateEvidence>,
}

impl TerminalIdentityIndex {
    pub(super) fn from_drain(drain: &PhysicalWorkDrainObservation) -> Result<Self, String> {
        if drain.evidence_overflow() != 0
            || drain.safe_evidence_elided() != 0
            || !drain.inspection_required().is_empty()
            || !drain.residual().is_empty()
            || !drain.derived_reconciliation_deferred().is_empty()
        {
            return Err("physical work shutdown retained incomplete terminal evidence".to_owned());
        }
        let mut terminal = HashMap::new();
        insert_all(
            &mut terminal,
            drain.settled(),
            PhysicalWorkTerminalFateEvidence::Settled,
        )?;
        insert_all(
            &mut terminal,
            drain.continued_after_consumer_cancellation(),
            PhysicalWorkTerminalFateEvidence::ContinuedAfterConsumerCancellation,
        )?;
        Ok(Self { terminal })
    }

    pub(super) fn take(
        &mut self,
        identity: PhysicalWorkIdentity,
    ) -> Result<PhysicalWorkTerminalFateEvidence, String> {
        self.terminal.remove(&identity).ok_or_else(|| {
            format!(
                "physical work {} omitted one exact terminal Store fate",
                identity.operation().get()
            )
        })
    }

    pub(super) fn require_consumed(self) -> Result<(), String> {
        if self.terminal.is_empty() {
            return Ok(());
        }
        Err(format!(
            "{} terminal Store fates omitted causal settlement records",
            self.terminal.len()
        ))
    }
}

fn insert_all(
    terminal: &mut HashMap<PhysicalWorkIdentity, PhysicalWorkTerminalFateEvidence>,
    identities: &[PhysicalWorkIdentity],
    fate: PhysicalWorkTerminalFateEvidence,
) -> Result<(), String> {
    for identity in identities {
        if terminal.insert(*identity, fate).is_some() {
            return Err(format!(
                "physical work {} retained duplicate terminal Store fates",
                identity.operation().get()
            ));
        }
    }
    Ok(())
}
