use crate::domain_artifacts::digest_basis::HadwigerArtifactPayloadEntry;

use super::motif_artifacts::{MotifGeometryTemplateReference, MotifProofSupportPosture};
use super::motif_identity::{
    MotifForbiddenSameColorPair, MotifParameterBinding, MotifTerminal, MotifUnitEdge, MotifVertex,
};

pub(crate) fn motif_payload_entries(
    motif_id: &str,
    source_family: Option<&str>,
    novelty_signature: Option<&str>,
    geometry_template: Option<&MotifGeometryTemplateReference>,
    proof_support_posture: MotifProofSupportPosture,
    motif_index: &MotifCanonicalIndex<'_>,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut entries = vec![
        HadwigerArtifactPayloadEntry::text("motif_id", motif_id),
        HadwigerArtifactPayloadEntry::text("proof_support_posture", proof_support_posture.as_str()),
    ];
    push_optional(&mut entries, "source_family", source_family);
    push_optional(&mut entries, "novelty_signature", novelty_signature);
    if let Some(geometry_template) = geometry_template {
        entries.push(HadwigerArtifactPayloadEntry::text(
            "geometry_template",
            geometry_template.stable_token(),
        ));
    }
    for vertex in motif_index.vertices {
        entries.push(HadwigerArtifactPayloadEntry::text("vertex", vertex.label()));
    }
    for parameter in motif_index.parameters {
        entries.push(HadwigerArtifactPayloadEntry::text(
            "parameter",
            parameter.stable_token(),
        ));
    }
    for terminal in motif_index.terminals {
        entries.push(HadwigerArtifactPayloadEntry::text(
            "terminal",
            terminal.label(),
        ));
    }
    for edge in motif_index.unit_edges {
        entries.push(HadwigerArtifactPayloadEntry::text(
            "unit_edge",
            edge.stable_token(),
        ));
    }
    for pair in motif_index.forbidden_pairs {
        entries.push(HadwigerArtifactPayloadEntry::text(
            "forbidden_same_color_pair",
            pair.stable_token(),
        ));
    }
    entries
}

pub(crate) struct MotifCanonicalIndex<'a> {
    pub(crate) vertices: &'a [MotifVertex],
    pub(crate) parameters: &'a [MotifParameterBinding],
    pub(crate) terminals: &'a [MotifTerminal],
    pub(crate) unit_edges: &'a [MotifUnitEdge],
    pub(crate) forbidden_pairs: &'a [MotifForbiddenSameColorPair],
}

fn push_optional(
    entries: &mut Vec<HadwigerArtifactPayloadEntry>,
    locus: &'static str,
    value: Option<&str>,
) {
    if let Some(value) = value {
        entries.push(HadwigerArtifactPayloadEntry::text(locus, value));
    }
}
