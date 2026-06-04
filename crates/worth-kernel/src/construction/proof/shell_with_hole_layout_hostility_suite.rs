use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use worth_primitives::{
    derive_shell_with_hole_layout, ShellWithHoleWitnessLayoutError,
    ShellWithHoleWitnessLayoutPolicy,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlanarWitnessContainmentReport {
    maximum_center_radius: f64,
    farthest_center_radius: f64,
}

impl PlanarWitnessContainmentReport {
    pub fn containment_verified(&self) -> bool {
        self.farthest_center_radius <= self.maximum_center_radius
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlanarWitnessNonOverlapReport {
    minimum_center_spacing: f64,
    nearest_center_spacing: f64,
}

impl PlanarWitnessNonOverlapReport {
    pub fn non_overlap_verified(&self) -> bool {
        self.nearest_center_spacing >= self.minimum_center_spacing
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShellWithHoleLayoutHostilitySuite {
    containment: PlanarWitnessContainmentReport,
    non_overlap: PlanarWitnessNonOverlapReport,
    rejected_missing_hole_loop: bool,
    report_digest: String,
}

impl ShellWithHoleLayoutHostilitySuite {
    pub fn containment(&self) -> PlanarWitnessContainmentReport {
        self.containment
    }

    pub fn non_overlap(&self) -> PlanarWitnessNonOverlapReport {
        self.non_overlap
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }

    pub fn rejected_missing_hole_loop(&self) -> bool {
        self.rejected_missing_hole_loop
    }
}

pub fn prepare_shell_with_hole_layout_hostility_suite() -> ShellWithHoleLayoutHostilitySuite {
    let (layout, legality) = derive_shell_with_hole_layout(
        12,
        &[5, 5, 5, 5, 5, 5],
        ShellWithHoleWitnessLayoutPolicy::default(),
    )
    .expect("layout");
    let farthest_center_radius = layout
        .hole_centers()
        .iter()
        .map(|center| center[0].hypot(center[1]))
        .fold(0.0, f64::max);
    let nearest_center_spacing = layout
        .hole_centers()
        .iter()
        .enumerate()
        .flat_map(|(left_index, left)| {
            layout
                .hole_centers()
                .iter()
                .skip(left_index + 1)
                .map(move |right| (left[0] - right[0]).hypot(left[1] - right[1]))
        })
        .fold(f64::INFINITY, f64::min);
    let containment = PlanarWitnessContainmentReport {
        maximum_center_radius: legality.maximum_center_radius(),
        farthest_center_radius,
    };
    let non_overlap = PlanarWitnessNonOverlapReport {
        minimum_center_spacing: legality.minimum_center_spacing(),
        nearest_center_spacing,
    };
    let rejected_missing_hole_loop = matches!(
        derive_shell_with_hole_layout(12, &[], ShellWithHoleWitnessLayoutPolicy::default()),
        Err(ShellWithHoleWitnessLayoutError::MissingHoleLoop)
    );
    let report_digest = digest_owned_parts_with_scope(
        ConstructionDigestScope::ArtifactIdentity,
        &[
            containment.maximum_center_radius.to_bits().to_string(),
            containment.farthest_center_radius.to_bits().to_string(),
            non_overlap.minimum_center_spacing.to_bits().to_string(),
            non_overlap.nearest_center_spacing.to_bits().to_string(),
            rejected_missing_hole_loop.to_string(),
        ],
    );
    ShellWithHoleLayoutHostilitySuite {
        containment,
        non_overlap,
        rejected_missing_hole_loop,
        report_digest,
    }
}
