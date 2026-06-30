use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::closeout_input::ReplayUndoMilestoneTwelvePublicCloseoutInput;
use super::counters::ReplayUndoMilestoneTwelvePublicCloseoutCounters;
use super::error::{
    ReplayUndoMilestoneTwelvePublicCloseoutError, ReplayUndoMilestoneTwelvePublicCloseoutErrorKind,
};
use super::inventory_classification::ReplayUndoPublicCloseoutInventoryRow;
use crate::replay_undo_consumer_cutover::ReplayUndoMilestoneThirteenSeed;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoMilestoneTwelvePublicCloseout {
    closeout_identity: String,
    transaction_packet_identity: String,
    replay_scope_identity: String,
    undo_scope_identity: String,
    inventory_rows: Vec<ReplayUndoPublicCloseoutInventoryRow>,
    counters: ReplayUndoMilestoneTwelvePublicCloseoutCounters,
    milestone_thirteen_seed: ReplayUndoMilestoneThirteenSeed,
}

impl ReplayUndoMilestoneTwelvePublicCloseout {
    pub fn publish(
        input: ReplayUndoMilestoneTwelvePublicCloseoutInput<'_>,
    ) -> Result<Self, ReplayUndoMilestoneTwelvePublicCloseoutError> {
        require_final_publication_proofs(&input)?;
        let inventory_rows =
            ReplayUndoPublicCloseoutInventoryRow::from_inventory(input.inventory())?;
        require_complete_classification(input.inventory().rows().len(), &inventory_rows)?;
        let counters = ReplayUndoMilestoneTwelvePublicCloseoutCounters::from_proof_products(
            &inventory_rows,
            input.consumer_cutover(),
            input.hard_deletion(),
        );
        let milestone_thirteen_seed = input.hard_deletion().milestone_thirteen_seed().clone();
        let closeout_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &closeout_identity_parts(&milestone_thirteen_seed, &input, &counters),
        );

        Ok(Self {
            closeout_identity,
            transaction_packet_identity: input
                .consumer_cutover()
                .transaction_packet_identity()
                .to_string(),
            replay_scope_identity: input.consumer_cutover().replay_scope_identity().to_string(),
            undo_scope_identity: input.consumer_cutover().undo_scope_identity().to_string(),
            inventory_rows,
            counters,
            milestone_thirteen_seed,
        })
    }

    pub fn closeout_identity(&self) -> &str {
        &self.closeout_identity
    }

    pub fn transaction_packet_identity(&self) -> &str {
        &self.transaction_packet_identity
    }

    pub fn replay_scope_identity(&self) -> &str {
        &self.replay_scope_identity
    }

    pub fn undo_scope_identity(&self) -> &str {
        &self.undo_scope_identity
    }

    pub fn inventory_rows(&self) -> &[ReplayUndoPublicCloseoutInventoryRow] {
        &self.inventory_rows
    }

    pub const fn counters(&self) -> ReplayUndoMilestoneTwelvePublicCloseoutCounters {
        self.counters
    }

    pub const fn milestone_thirteen_seed(&self) -> &ReplayUndoMilestoneThirteenSeed {
        &self.milestone_thirteen_seed
    }
}

fn require_final_publication_proofs(
    input: &ReplayUndoMilestoneTwelvePublicCloseoutInput<'_>,
) -> Result<(), ReplayUndoMilestoneTwelvePublicCloseoutError> {
    if input.hard_deletion().source_firewall().violation_count() != 0 {
        return Err(ReplayUndoMilestoneTwelvePublicCloseoutError::new(
            ReplayUndoMilestoneTwelvePublicCloseoutErrorKind::UncleanFirewall,
            "public replay/undo closeout requires a clean hard-deletion source firewall",
        ));
    }
    if input.hard_deletion().uncapped_residue_count() != 0 {
        return Err(ReplayUndoMilestoneTwelvePublicCloseoutError::new(
            ReplayUndoMilestoneTwelvePublicCloseoutErrorKind::UncappedResidue,
            "public replay/undo closeout requires capped non-ordinary residue",
        ));
    }
    let seed = input.hard_deletion().milestone_thirteen_seed();
    if seed.hard_deletion_ledger_digest().is_some()
        && seed.residue_cap_audit_digest().is_some()
        && seed.hard_deletion_source_firewall_digest().is_some()
    {
        Ok(())
    } else {
        Err(ReplayUndoMilestoneTwelvePublicCloseoutError::new(
            ReplayUndoMilestoneTwelvePublicCloseoutErrorKind::UnpublishedHardDeletionProof,
            "public replay/undo closeout cannot publish a pre-hard-deletion seed",
        ))
    }
}

fn require_complete_classification(
    expected_row_count: usize,
    inventory_rows: &[ReplayUndoPublicCloseoutInventoryRow],
) -> Result<(), ReplayUndoMilestoneTwelvePublicCloseoutError> {
    if inventory_rows.len() == expected_row_count {
        Ok(())
    } else {
        Err(ReplayUndoMilestoneTwelvePublicCloseoutError::new(
            ReplayUndoMilestoneTwelvePublicCloseoutErrorKind::IncompleteInventoryClassification,
            "public replay/undo closeout did not classify every in-scope seed surface",
        ))
    }
}

fn closeout_identity_parts(
    seed: &ReplayUndoMilestoneThirteenSeed,
    input: &ReplayUndoMilestoneTwelvePublicCloseoutInput<'_>,
    counters: &ReplayUndoMilestoneTwelvePublicCloseoutCounters,
) -> Vec<String> {
    vec![
        "worth-kernel:replay-undo-milestone-twelve-public-closeout:v1".to_string(),
        format!("seed:{}", seed.seed_identity()),
        format!(
            "hard-deletion-ledger:{}",
            seed.hard_deletion_ledger_digest()
                .expect("publication already required hard deletion ledger digest")
        ),
        format!(
            "residue-cap-audit:{}",
            seed.residue_cap_audit_digest()
                .expect("publication already required residue cap audit digest")
        ),
        format!(
            "hard-source-firewall:{}",
            seed.hard_deletion_source_firewall_digest()
                .expect("publication already required hard source firewall digest")
        ),
        format!(
            "forbidden-denials:{}",
            input
                .consumer_cutover()
                .forbidden_surface_denials()
                .row_count()
        ),
        format!("inventory-rows:{}", counters.inventory_row_count()),
    ]
}
