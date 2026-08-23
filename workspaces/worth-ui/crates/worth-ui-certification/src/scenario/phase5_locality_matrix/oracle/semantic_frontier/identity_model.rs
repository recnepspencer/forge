//! Separately authored canonical identity model for Query semantic evidence.

use sha2::{Digest, Sha256};
use worth_ui_host_native::{
    UiNativeClientPresentationSemanticChange as SemanticChange,
    UiNativeClientPresentationSemanticSubscriberObservation as Subscriber,
};

pub(super) fn require_exact(subscriber: Subscriber, change: SemanticChange) -> Result<(), String> {
    let source = modeled_source_digest(subscriber)?;
    if subscriber.source_digest() != source {
        return Err("semantic source identity disagrees with the canonical model".to_owned());
    }
    if subscriber.immediate_dependency_digest()
        != modeled_dependency_digest(subscriber, source, change)
    {
        return Err("immediate dependency identity disagrees with the canonical model".to_owned());
    }
    Ok(())
}

fn modeled_source_digest(subscriber: Subscriber) -> Result<[u8; 32], String> {
    let mechanic = match (subscriber.mounted_instance(), subscriber.semantic_slot()) {
        (Some(instance), Some(slot)) => Some((instance, slot, subscriber.collection_row())),
        (None, None) => None,
        _ => return Err("semantic subscriber retained a partial mechanic identity".to_owned()),
    };
    let mut digest = Sha256::new();
    digest.update(b"worth-ui-presentation-source-v1");
    for identity in [
        subscriber.attempt(),
        subscriber.semantic_surface(),
        subscriber.host_surface(),
        subscriber.binding(),
        subscriber.host_lineage(),
        subscriber.mounted_frame(),
    ] {
        digest.update(identity.to_le_bytes());
    }
    digest.update([u8::from(subscriber.removal())]);
    append_mechanic(&mut digest, subscriber, mechanic);
    Ok(digest.finalize().into())
}

fn append_mechanic(
    digest: &mut Sha256,
    subscriber: Subscriber,
    mechanic: Option<(u64, u16, Option<[u8; 32]>)>,
) {
    if let Some((instance, slot, row)) = mechanic {
        digest.update([1]);
        digest.update(instance.to_le_bytes());
        digest.update(slot.to_le_bytes());
        digest.update([u8::from(row.is_some())]);
        digest.update(row.unwrap_or([0; 32]));
        for evidence in [
            subscriber.content_digest(),
            subscriber.layout_digest(),
            subscriber.foreground_digest(),
            subscriber.raster_key_set_digest(),
        ] {
            digest.update(evidence);
        }
    } else {
        digest.update([0]);
        digest.update(0_u64.to_le_bytes());
        digest.update(0_u16.to_le_bytes());
        digest.update([0]);
        for _ in 0..5 {
            digest.update([0; 32]);
        }
    }
}

fn modeled_dependency_digest(
    subscriber: Subscriber,
    source: [u8; 32],
    change: SemanticChange,
) -> [u8; 32] {
    let ordinal = match change {
        SemanticChange::Content => 0_u64,
        SemanticChange::Width => 1,
        SemanticChange::PaintValue => 2,
        SemanticChange::PaintBoundary => 3,
        SemanticChange::Dpi => 4,
        SemanticChange::UploadCompletion => 5,
        SemanticChange::PinRelease => 6,
        SemanticChange::Currentness => 7,
    };
    let mut digest = Sha256::new();
    digest.update(b"worth-ui-presentation-dependency-v1");
    digest.update(ordinal.to_le_bytes());
    match change {
        SemanticChange::Dpi => {
            digest.update(subscriber.mounted_frame().to_le_bytes());
            digest.update(subscriber.semantic_surface().to_le_bytes());
            digest.update(subscriber.host_lineage().to_le_bytes());
        }
        SemanticChange::UploadCompletion => {
            digest.update(subscriber.raster_key_set_digest());
            digest.update(subscriber.mounted_frame().to_le_bytes());
            digest.update(subscriber.semantic_surface().to_le_bytes());
            digest.update(subscriber.host_lineage().to_le_bytes());
        }
        SemanticChange::Currentness => {
            for identity in [
                subscriber.attempt(),
                subscriber.semantic_surface(),
                subscriber.host_surface(),
                subscriber.binding(),
                subscriber.host_lineage(),
                subscriber.mounted_frame(),
            ] {
                digest.update(identity.to_le_bytes());
            }
        }
        _ => digest.update(source),
    }
    digest.finalize().into()
}
