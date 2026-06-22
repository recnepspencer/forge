use worth_math::sign::TriSign;

use crate::planar_contracts::segment_segment_2d::CertifiedSegmentSegment2DReceipt;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PlanarBooleanPredicateBoundPairBasis {
    first_start_point_2d: [f64; 2],
    first_end_point_2d: [f64; 2],
    second_start_point_2d: [f64; 2],
    second_end_point_2d: [f64; 2],
    endpoint_source_identities: [String; 4],
    endpoint_projection_fact_digests: [String; 4],
    orientation_signs: [TriSign; 4],
    orientation_fact_digests: [String; 4],
}

impl PlanarBooleanPredicateBoundPairBasis {
    pub(crate) fn from_segment_receipt(receipt: &CertifiedSegmentSegment2DReceipt) -> Self {
        let basis = receipt.basis();
        let endpoint_source_identities = basis.endpoint_source_identities().map(str::to_string);
        let endpoint_projection_fact_digests =
            basis.endpoint_projection_fact_digests().map(str::to_string);
        let orientation_fact_digests = basis.orientation_fact_digests().map(str::to_string);
        Self {
            first_start_point_2d: basis.first_start_point_2d(),
            first_end_point_2d: basis.first_end_point_2d(),
            second_start_point_2d: basis.second_start_point_2d(),
            second_end_point_2d: basis.second_end_point_2d(),
            endpoint_source_identities,
            endpoint_projection_fact_digests,
            orientation_signs: basis.orientation_signs(),
            orientation_fact_digests,
        }
    }

    pub(crate) fn first_start_point_2d(&self) -> [f64; 2] {
        self.first_start_point_2d
    }

    pub(crate) fn first_end_point_2d(&self) -> [f64; 2] {
        self.first_end_point_2d
    }

    pub(crate) fn second_start_point_2d(&self) -> [f64; 2] {
        self.second_start_point_2d
    }

    pub(crate) fn second_end_point_2d(&self) -> [f64; 2] {
        self.second_end_point_2d
    }

    pub(crate) fn endpoint_source_identities(&self) -> [&str; 4] {
        self.endpoint_source_identities
            .each_ref()
            .map(String::as_str)
    }

    pub(crate) fn endpoint_projection_fact_digests(&self) -> [&str; 4] {
        self.endpoint_projection_fact_digests
            .each_ref()
            .map(String::as_str)
    }

    pub(crate) fn orientation_signs(&self) -> [TriSign; 4] {
        self.orientation_signs
    }

    pub(crate) fn orientation_fact_digests(&self) -> [&str; 4] {
        self.orientation_fact_digests.each_ref().map(String::as_str)
    }
}
