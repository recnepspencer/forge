use std::collections::BTreeMap;

use worth_math::arithmetic::Rational;

use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanSplitEdgeFragment, PlanarBooleanSplitEdgeFragmentEndpointKind,
    PlanarBooleanSplitEdgeFragmentEndpointRef, PlanarBooleanSplitEdgeFragmentSet,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopSourceCarrierRow, PlanarBooleanLoopSourceCarrierSet,
};

pub(super) struct DegenerateLoopGeometryIndex<'a> {
    fragment_index: BTreeMap<String, &'a PlanarBooleanSplitEdgeFragment>,
    carrier_index: BTreeMap<String, &'a PlanarBooleanLoopSourceCarrierRow>,
}

impl<'a> DegenerateLoopGeometryIndex<'a> {
    pub(super) fn new(
        source_loop_carriers: &'a PlanarBooleanLoopSourceCarrierSet,
        split_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
    ) -> Self {
        Self {
            fragment_index: split_fragments
                .fragments()
                .map(|row| (row.fragment_identity().to_string(), row))
                .collect(),
            carrier_index: source_loop_carriers
                .rows()
                .iter()
                .map(|row| (row.carrier_identity().to_string(), row))
                .collect(),
        }
    }

    pub(super) fn classify_zero_area(
        &self,
        loop_identity: &str,
        fragment_identities: &[String],
    ) -> Result<Option<String>, String> {
        let Some(vertices) = self.oriented_loop_vertices(fragment_identities) else {
            return Err(
                "loop degeneracy classification requires source-loop carrier endpoint evidence and fragment geometry that reconstruct a continuous loop walk before identity minting"
                    .to_string(),
            );
        };
        let signed_area_twice = signed_area_twice(&vertices).ok_or_else(|| {
            "loop degeneracy classification requires finite projected local coordinates to classify loop area before identity minting"
                .to_string()
        })?;
        Ok((signed_area_twice == Rational::zero()).then(|| {
            format!(
                "loop degeneracy classification denies zero-area loop walks before identity minting because the reconstructed fragment walk for {loop_identity} collapses to an exact shoelace area of zero"
            )
        }))
    }

    fn oriented_loop_vertices(&self, fragment_identities: &[String]) -> Option<Vec<[f64; 2]>> {
        let fragments = fragment_identities
            .iter()
            .map(|identity| self.fragment_geometry(identity))
            .collect::<Option<Vec<_>>>()?;
        orient_loop_vertices(&fragments)
    }

    fn fragment_geometry(&self, fragment_identity: &str) -> Option<FragmentGeometry<'a>> {
        let fragment = self.fragment_index.get(fragment_identity).copied()?;
        let carrier = self
            .carrier_index
            .get(fragment.carrier_identity())
            .copied()?;
        Some(FragmentGeometry {
            start_endpoint: fragment.start_endpoint(),
            end_endpoint: fragment.end_endpoint(),
            carrier,
        })
    }
}

fn orient_loop_vertices(fragments: &[FragmentGeometry<'_>]) -> Option<Vec<[f64; 2]>> {
    if fragments.is_empty() {
        return Some(Vec::new());
    }

    for reverse_first in [false, true] {
        let mut oriented = Vec::with_capacity(fragments.len());
        oriented.push(oriented_fragment(&fragments[0], reverse_first));

        for fragment in &fragments[1..] {
            let natural = oriented_fragment(fragment, false);
            let reversed = oriented_fragment(fragment, true);
            let Some(last) = oriented.last() else {
                return None;
            };
            if last.end_identity == natural.start_identity {
                oriented.push(natural);
            } else if last.end_identity == reversed.start_identity {
                oriented.push(reversed);
            } else {
                oriented.clear();
                break;
            }
        }

        if oriented.is_empty() {
            continue;
        }

        let first = oriented.first()?;
        let last = oriented.last()?;
        if last.end_identity == first.start_identity {
            return Some(oriented.into_iter().map(|row| row.start_point).collect());
        }
    }

    None
}

fn oriented_fragment(fragment: &FragmentGeometry<'_>, reversed: bool) -> OrientedFragment {
    let natural = OrientedFragment {
        start_identity: endpoint_walk_identity(fragment.start_endpoint, fragment.carrier),
        end_identity: endpoint_walk_identity(fragment.end_endpoint, fragment.carrier),
        start_point: endpoint_point(fragment.start_endpoint, fragment.carrier),
        end_point: endpoint_point(fragment.end_endpoint, fragment.carrier),
    };
    if reversed {
        OrientedFragment {
            start_identity: natural.end_identity,
            end_identity: natural.start_identity,
            start_point: natural.end_point,
            end_point: natural.start_point,
        }
    } else {
        natural
    }
}

fn endpoint_walk_identity(
    endpoint: &PlanarBooleanSplitEdgeFragmentEndpointRef,
    carrier: &PlanarBooleanLoopSourceCarrierRow,
) -> String {
    match endpoint.endpoint_kind() {
        PlanarBooleanSplitEdgeFragmentEndpointKind::SplitVertex => {
            endpoint.endpoint_identity().to_string()
        }
        PlanarBooleanSplitEdgeFragmentEndpointKind::OriginalSourceStart => {
            carrier.start_source_endpoint_identity().to_string()
        }
        PlanarBooleanSplitEdgeFragmentEndpointKind::OriginalSourceEnd => {
            carrier.end_source_endpoint_identity().to_string()
        }
    }
}

fn endpoint_point(
    endpoint: &PlanarBooleanSplitEdgeFragmentEndpointRef,
    carrier: &PlanarBooleanLoopSourceCarrierRow,
) -> [f64; 2] {
    let start = carrier.start_point_2d();
    let end = carrier.end_point_2d();
    let parameter = f64::from_bits(endpoint.parameter_bits());
    [
        start[0] + ((end[0] - start[0]) * parameter),
        start[1] + ((end[1] - start[1]) * parameter),
    ]
}

fn signed_area_twice(vertices: &[[f64; 2]]) -> Option<Rational> {
    let mut sum = Rational::zero();
    for edge in vertices.windows(2) {
        sum = &sum + &area_term(edge[0], edge[1])?;
    }
    let first = *vertices.first()?;
    let last = *vertices.last()?;
    Some(&sum + &area_term(last, first)?)
}

fn area_term(a: [f64; 2], b: [f64; 2]) -> Option<Rational> {
    let ax = Rational::try_from_f64(a[0]).ok()?;
    let ay = Rational::try_from_f64(a[1]).ok()?;
    let bx = Rational::try_from_f64(b[0]).ok()?;
    let by = Rational::try_from_f64(b[1]).ok()?;
    Some((&ax * &by) - (&ay * &bx))
}

struct FragmentGeometry<'a> {
    start_endpoint: &'a PlanarBooleanSplitEdgeFragmentEndpointRef,
    end_endpoint: &'a PlanarBooleanSplitEdgeFragmentEndpointRef,
    carrier: &'a PlanarBooleanLoopSourceCarrierRow,
}

struct OrientedFragment {
    start_identity: String,
    end_identity: String,
    start_point: [f64; 2],
    end_point: [f64; 2],
}
