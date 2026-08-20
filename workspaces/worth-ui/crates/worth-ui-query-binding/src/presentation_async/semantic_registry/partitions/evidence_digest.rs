use super::{WorthUiPresentationMechanicBasis, WorthUiPresentationRequestBasis};
use sha2::{Digest, Sha256};

pub(super) fn raster_key_set_digest(mechanic: &WorthUiPresentationMechanicBasis) -> [u8; 32] {
    let mut rows = mechanic
        .raster_key_set()
        .keys()
        .iter()
        .map(|key| key.canonical_evidence_bytes())
        .collect::<Vec<_>>();
    rows.sort_unstable();
    let mut digest = Sha256::new();
    digest.update((rows.len() as u64).to_le_bytes());
    for row in rows {
        digest.update((row.len() as u64).to_le_bytes());
        digest.update(row);
    }
    digest.finalize().into()
}

pub(super) fn subscriber_evidence_digests(
    basis: &WorthUiPresentationRequestBasis,
    mechanic: Option<&WorthUiPresentationMechanicBasis>,
    removal: bool,
) -> ([u8; 32], [[u8; 32]; super::super::DEPENDENCY_COUNT]) {
    let mut digest = Sha256::new();
    digest.update(b"worth-ui-presentation-source-v1");
    digest.update(basis.attempt().diagnostic_value().to_le_bytes());
    digest.update(basis.semantic_surface().diagnostic_value().to_le_bytes());
    digest.update(basis.host_surface().diagnostic_value().to_le_bytes());
    digest.update(basis.binding().diagnostic_value().to_le_bytes());
    digest.update(basis.host_lineage().diagnostic_value().to_le_bytes());
    digest.update(basis.mounted_frame().diagnostic_value().to_le_bytes());
    digest.update([u8::from(removal)]);
    if let Some(mechanic) = mechanic {
        digest.update([1]);
        digest.update(mechanic.mounted_instance().diagnostic_value().to_le_bytes());
        let (slot, row) = mechanic
            .mechanic()
            .semantic_text_identity_parts()
            .expect("presentation subscriber retains semantic-text mechanics");
        digest.update(slot.to_le_bytes());
        digest.update([u8::from(row.is_some())]);
        digest.update(row.unwrap_or([0; 32]));
        digest.update(content_digest(mechanic));
        digest.update(mechanic.layout().digest());
        digest.update(foreground_digest(mechanic));
        digest.update(raster_key_set_digest(mechanic));
    } else {
        digest.update([0]);
        digest.update(0_u64.to_le_bytes());
        digest.update(0_u16.to_le_bytes());
        digest.update([0]);
        digest.update([0; 32]);
        for _ in 0..4 {
            digest.update([0; 32]);
        }
    }
    let source: [u8; 32] = digest.finalize().into();
    let dependencies = std::array::from_fn(|ordinal| {
        let mut digest = Sha256::new();
        digest.update(b"worth-ui-presentation-dependency-v1");
        digest.update((ordinal as u64).to_le_bytes());
        match ordinal {
            4 => {
                digest.update(basis.mounted_frame().diagnostic_value().to_le_bytes());
                digest.update(basis.semantic_surface().diagnostic_value().to_le_bytes());
                digest.update(basis.host_lineage().diagnostic_value().to_le_bytes());
            }
            5 => {
                digest.update(mechanic.map(raster_key_set_digest).unwrap_or([0; 32]));
                digest.update(basis.mounted_frame().diagnostic_value().to_le_bytes());
                digest.update(basis.semantic_surface().diagnostic_value().to_le_bytes());
                digest.update(basis.host_lineage().diagnostic_value().to_le_bytes());
            }
            7 => {
                digest.update(basis.attempt().diagnostic_value().to_le_bytes());
                digest.update(basis.semantic_surface().diagnostic_value().to_le_bytes());
                digest.update(basis.host_surface().diagnostic_value().to_le_bytes());
                digest.update(basis.binding().diagnostic_value().to_le_bytes());
                digest.update(basis.host_lineage().diagnostic_value().to_le_bytes());
                digest.update(basis.mounted_frame().diagnostic_value().to_le_bytes());
            }
            _ => digest.update(source),
        }
        digest.finalize().into()
    });
    (source, dependencies)
}

pub(super) fn content_digest(mechanic: &WorthUiPresentationMechanicBasis) -> [u8; 32] {
    Sha256::digest(mechanic.content().as_bytes()).into()
}

pub(super) fn foreground_digest(mechanic: &WorthUiPresentationMechanicBasis) -> [u8; 32] {
    let mut digest = Sha256::new();
    for span in mechanic.paint_spans() {
        digest.update(span.foreground());
    }
    digest.finalize().into()
}
