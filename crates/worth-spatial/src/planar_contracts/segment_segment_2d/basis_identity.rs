use super::CertifiedSegmentSegment2DBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CertifiedSegmentSegment2DIdentityEntry {
    locus: &'static str,
    value: String,
}

impl CertifiedSegmentSegment2DIdentityEntry {
    fn new(locus: &'static str, value: impl Into<String>) -> Self {
        Self {
            locus,
            value: value.into(),
        }
    }

    pub(crate) fn locus(&self) -> &'static str {
        self.locus
    }

    pub(crate) fn value(&self) -> &str {
        &self.value
    }
}

pub(crate) fn certified_segment_segment_2d_identity_entries(
    basis: &CertifiedSegmentSegment2DBasis,
) -> Vec<CertifiedSegmentSegment2DIdentityEntry> {
    let mut entries = vec![
        entry(
            "geometry.segment_segment_2d.first_segment_identity",
            basis.first_segment_identity(),
        ),
        entry(
            "geometry.segment_segment_2d.second_segment_identity",
            basis.second_segment_identity(),
        ),
        entry(
            "geometry.segment_segment_2d.topology_basis",
            basis.topology_basis_identity(),
        ),
        entry(
            "geometry.segment_segment_2d.contact_policy",
            basis.contact_policy_identity(),
        ),
        entry(
            "geometry.segment_segment_2d.frame_identity",
            basis.frame_identity(),
        ),
        entry(
            "geometry.segment_segment_2d.local_frame_fact",
            basis.local_frame_fact_digest(),
        ),
        entry(
            "geometry.segment_segment_2d.local_frame_declaration",
            basis.local_frame_declaration_digest(),
        ),
        entry(
            "geometry.segment_segment_2d.local_frame_envelope",
            basis.local_frame_envelope_digest(),
        ),
        entry(
            "geometry.segment_segment_2d.transform_chain",
            basis.transform_chain_digest(),
        ),
        entry(
            "geometry.segment_segment_2d.movement_rotation",
            basis.movement_rotation_posture_identity(),
        ),
        entry(
            "geometry.segment_segment_2d.tolerance_policy",
            basis.tolerance_policy_identity(),
        ),
        entry(
            "geometry.segment_segment_2d.classification",
            basis.classification().as_str(),
        ),
    ];
    for (index, source_identity) in basis.endpoint_source_identities().iter().enumerate() {
        entries.push(entry(
            endpoint_locus(index, "source_identity"),
            *source_identity,
        ));
    }
    for (index, digest) in basis.endpoint_projection_fact_digests().iter().enumerate() {
        entries.push(entry(endpoint_locus(index, "projection_fact"), *digest));
    }
    for (index, orientation) in basis.orientations().iter().enumerate() {
        entries.push(entry(
            orientation_locus(index, "predicate_fact"),
            orientation.fact_digest.as_str(),
        ));
        entries.push(entry(
            orientation_locus(index, "sign"),
            format!("{:?}", orientation.sign),
        ));
        entries.push(entry(
            orientation_locus(index, "precision"),
            orientation.precision_escalation.as_str(),
        ));
    }
    entries
}

fn entry(locus: &'static str, value: impl Into<String>) -> CertifiedSegmentSegment2DIdentityEntry {
    CertifiedSegmentSegment2DIdentityEntry::new(locus, value)
}

fn endpoint_locus(index: usize, suffix: &str) -> &'static str {
    match (index, suffix) {
        (0, "source_identity") => "geometry.segment_segment_2d.endpoint.0.source_identity",
        (1, "source_identity") => "geometry.segment_segment_2d.endpoint.1.source_identity",
        (2, "source_identity") => "geometry.segment_segment_2d.endpoint.2.source_identity",
        (3, "source_identity") => "geometry.segment_segment_2d.endpoint.3.source_identity",
        (0, "projection_fact") => "geometry.segment_segment_2d.endpoint.0.projection_fact",
        (1, "projection_fact") => "geometry.segment_segment_2d.endpoint.1.projection_fact",
        (2, "projection_fact") => "geometry.segment_segment_2d.endpoint.2.projection_fact",
        (3, "projection_fact") => "geometry.segment_segment_2d.endpoint.3.projection_fact",
        _ => "geometry.segment_segment_2d.endpoint.unknown",
    }
}

fn orientation_locus(index: usize, suffix: &str) -> &'static str {
    match (index, suffix) {
        (0, "predicate_fact") => "geometry.segment_segment_2d.orientation.0.predicate_fact",
        (1, "predicate_fact") => "geometry.segment_segment_2d.orientation.1.predicate_fact",
        (2, "predicate_fact") => "geometry.segment_segment_2d.orientation.2.predicate_fact",
        (3, "predicate_fact") => "geometry.segment_segment_2d.orientation.3.predicate_fact",
        (0, "sign") => "geometry.segment_segment_2d.orientation.0.sign",
        (1, "sign") => "geometry.segment_segment_2d.orientation.1.sign",
        (2, "sign") => "geometry.segment_segment_2d.orientation.2.sign",
        (3, "sign") => "geometry.segment_segment_2d.orientation.3.sign",
        (0, "precision") => "geometry.segment_segment_2d.orientation.0.precision",
        (1, "precision") => "geometry.segment_segment_2d.orientation.1.precision",
        (2, "precision") => "geometry.segment_segment_2d.orientation.2.precision",
        (3, "precision") => "geometry.segment_segment_2d.orientation.3.precision",
        _ => "geometry.segment_segment_2d.orientation.unknown",
    }
}
