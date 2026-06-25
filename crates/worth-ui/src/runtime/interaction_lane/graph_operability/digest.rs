use crate::runtime::{
    WorthUiInteractionKind, WorthUiMountedInteractionGesture, WorthUiRuntimeFactId,
};

use super::activation_receipts::WorthUiMountedInteractionActivation;
use super::{WorthUiInteractionOperabilityBasis, WorthUiInteractionOperabilityPosture};

pub(super) fn mounted_plan_digest(
    surface_id: &str,
    activation: &WorthUiMountedInteractionActivation,
) -> u64 {
    let activation_digest = match activation {
        WorthUiMountedInteractionActivation::Eligible(receipt) => receipt.receipt_digest(),
        WorthUiMountedInteractionActivation::Denied(receipt) => receipt.receipt_digest(),
    };
    fold_basis(0xcbf2_9ce4_8422_2325, ["mounted-plan", surface_id])
        ^ activation_digest.wrapping_mul(0x0000_0100_0000_01b3)
}

pub(super) fn operability_digest(
    posture: WorthUiInteractionOperabilityPosture,
    basis: WorthUiInteractionOperabilityBasis,
    query_graph_digest: u64,
    consumed_facts: &[WorthUiRuntimeFactId],
) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325;
    digest = fold(digest, &query_graph_digest.to_le_bytes());
    digest = fold(digest, format!("{posture:?}").as_bytes());
    digest = fold(digest, format!("{basis:?}").as_bytes());
    for fact in consumed_facts {
        digest = fold(digest, fact.family().token().as_bytes());
        digest = fold(digest, fact.identity().as_bytes());
    }
    digest
}

pub(super) fn activation_digest(
    surface_id: &str,
    interaction_id: &str,
    kind: WorthUiInteractionKind,
    gesture: WorthUiMountedInteractionGesture,
    operability_digest: u64,
    emitted_digest: u64,
) -> u64 {
    let digest = fold_basis(
        operability_digest,
        [
            "mounted-activation",
            surface_id,
            interaction_id,
            kind.token(),
            gesture.token(),
        ],
    );
    digest ^ emitted_digest.wrapping_mul(0x0000_0100_0000_01b3)
}

fn fold_basis<const N: usize>(mut digest: u64, values: [&str; N]) -> u64 {
    for value in values {
        digest = fold(digest, value.as_bytes());
    }
    digest
}

fn fold(mut digest: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
    }
    digest
}
