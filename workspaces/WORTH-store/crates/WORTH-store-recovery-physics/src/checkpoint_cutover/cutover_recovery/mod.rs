mod recovered_checkpoint_evidence;
mod recovered_cutover_state;
mod recovery_selection;

use super::{CheckpointCutoverReceipt, CheckpointValidationDenial, CheckpointValidationDenialKind};

pub use recovered_checkpoint_evidence::{
    RecoveredCheckpointManifestMedia, RecoveredCheckpointRoot, RecoveredCheckpointSelector,
};
pub use recovered_cutover_state::{CheckpointCutoverCrashStage, RecoveredCheckpointCutoverState};
pub use recovery_selection::{
    CheckpointCutoverRecoverySelection, CheckpointCutoverRecoverySelectionKind,
};

fn recovered_mismatch(receipt: &CheckpointCutoverReceipt) -> CheckpointValidationDenial {
    CheckpointValidationDenial::new(
        CheckpointValidationDenialKind::RecoveredCheckpointEvidenceMismatch,
        receipt.counters().with_cutover_decision(),
    )
}
