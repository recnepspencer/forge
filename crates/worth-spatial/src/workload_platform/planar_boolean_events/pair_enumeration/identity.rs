use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::counters::PlanarBooleanSegmentPairEnumerationCounters;
use super::work_item::PlanarBooleanSegmentPairWorkItem;
use crate::workload_platform::planar_boolean_events::segment_identity::PlanarBooleanCanonicalSegment;

pub(crate) fn canonical_segment_set_identity(
    left: &[PlanarBooleanCanonicalSegment],
    right: &[PlanarBooleanCanonicalSegment],
) -> String {
    let mut parts = vec!["planar-boolean-canonical-segment-set".to_string()];
    parts.push("left".to_string());
    parts.extend(sorted_segment_identity_parts(left));
    parts.push("right".to_string());
    parts.extend(sorted_segment_identity_parts(right));
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn pair_work_item_identity(
    left: &PlanarBooleanCanonicalSegment,
    right: &PlanarBooleanCanonicalSegment,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-segment-pair-work-item".to_string(),
            format!("left:{}", left.canonical_segment_identity()),
            format!("right:{}", right.canonical_segment_identity()),
            format!("left-carrier:{}", left.carrier_identity()),
            format!("right-carrier:{}", right.carrier_identity()),
            format!("left-local-frame:{}", left.local_frame_identity()),
            format!("right-local-frame:{}", right.local_frame_identity()),
            format!("left-precision-basis:{}", left.precision_basis_identity()),
            format!("right-precision-basis:{}", right.precision_basis_identity()),
        ],
    )
}

pub(crate) fn pair_enumeration_identity(
    canonical_segment_set_identity: &str,
    query_index_identity: &str,
    query_index_declaration_digest: &str,
    query_index_plan_digest: &str,
    query_index_envelope_digest: &str,
    counters: PlanarBooleanSegmentPairEnumerationCounters,
    work_items: &[PlanarBooleanSegmentPairWorkItem],
) -> String {
    let mut parts = vec![
        "planar-boolean-segment-pair-enumeration".to_string(),
        format!("canonical-segment-set:{canonical_segment_set_identity}"),
        format!("query-index:{query_index_identity}"),
        format!("query-index-declaration:{query_index_declaration_digest}"),
        format!("query-index-plan:{query_index_plan_digest}"),
        format!("query-index-envelope:{query_index_envelope_digest}"),
        format!("left-count:{}", counters.left_segment_count()),
        format!("right-count:{}", counters.right_segment_count()),
        format!("expected-breadth:{}", counters.expected_pair_breadth()),
        format!("emitted-breadth:{}", counters.emitted_pair_breadth()),
        format!("skipped-pairs:{}", counters.skipped_pair_count()),
        format!(
            "query-index-candidates:{}",
            counters.query_index_candidate_count()
        ),
        format!(
            "query-index-culled-pairs:{}",
            counters.query_index_culled_pair_count()
        ),
    ];
    parts.extend(
        work_items
            .iter()
            .map(|work_item| format!("pair:{}", work_item.segment_pair_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

fn sorted_segment_identity_parts(segments: &[PlanarBooleanCanonicalSegment]) -> Vec<String> {
    let mut identities = segments
        .iter()
        .map(|segment| segment.canonical_segment_identity().to_string())
        .collect::<Vec<_>>();
    identities.sort();
    identities
}
