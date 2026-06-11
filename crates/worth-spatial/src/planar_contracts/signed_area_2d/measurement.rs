use worth_math::arithmetic::{Interval, PrecisionBudget, Rational};
use worth_math::sign::TriSign;

use crate::planar_contracts::polygon_winding_2d::CertifiedLoopContainment;

use super::scale_comparison::LocalScaleAreaThresholds;
use super::{
    AreaDegeneracyClass, CertifiedSignedArea2DBasis, CertifiedSignedArea2DDenial,
    CertifiedSignedArea2DDenialKind, SignedAreaDegeneracyCause, SignedAreaOrientation,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CertifiedSignedAreaMeasurement {
    pub(crate) orientation: SignedAreaOrientation,
    pub(crate) degeneracy: AreaDegeneracyClass,
    pub(crate) signed_area_twice_decimal: String,
    pub(crate) localized_cause: Option<SignedAreaDegeneracyCause>,
    pub(crate) loop_edges_walked: usize,
    pub(crate) area_terms_evaluated: usize,
    pub(crate) precision_escalations: usize,
    pub(crate) local_scale_comparisons: usize,
    pub(crate) degeneracy_localization_breadth: usize,
}

pub(crate) fn certify_signed_area(
    basis: &CertifiedSignedArea2DBasis,
) -> Result<CertifiedSignedAreaMeasurement, CertifiedSignedArea2DDenial> {
    let thresholds = LocalScaleAreaThresholds::from_normalization_scale(
        basis.precision_receipt().basis().normalization_scale(),
    )
    .map_err(|_| {
        CertifiedSignedArea2DDenial::new(
            CertifiedSignedArea2DDenialKind::PrecisionBudgetExceeded,
            "signed area requires a finite positive local normalization scale",
        )
    })?;
    let mut budget = PrecisionBudget::default();
    let mut net = Rational::zero();
    let loop_edges_walked = basis
        .loops()
        .iter()
        .map(|loop_summary| loop_summary.vertices().len())
        .sum();
    let mut terms = 0;
    let mut tiny_hole = false;
    let mut policy_required_cause = None;
    let mut interval_escalations = 0;
    for loop_summary in basis.loops() {
        if loop_summary.containment_identity() == CertifiedLoopContainment::Outside.as_str() {
            policy_required_cause = Some(SignedAreaDegeneracyCause::ContainmentPolicyRequired {
                loop_identity: loop_summary.loop_identity().to_string(),
                containment: "outside".to_string(),
                policy: basis.degeneracy_policy().as_str().to_string(),
            });
            continue;
        }
        if loop_interval_signed_area_twice(loop_summary.vertices())
            .sign()
            .is_none()
        {
            interval_escalations += 1;
        }
        let signed = loop_signed_area_twice(loop_summary.vertices(), &mut budget)?;
        terms += loop_summary.vertices().len();
        let magnitude = signed.clone().abs();
        if loop_summary.containment_identity() == CertifiedLoopContainment::ContainedHole.as_str()
            && magnitude > *thresholds.zero_area()
            && magnitude <= *thresholds.tiny_hole_area()
        {
            tiny_hole = true;
        }
        net = if loop_summary.containment_identity()
            == CertifiedLoopContainment::ContainedHole.as_str()
        {
            &net - &magnitude
        } else {
            &net + &signed
        };
    }
    let absolute = net.clone().abs();
    let orientation = match net.sign() {
        TriSign::Pos => SignedAreaOrientation::CounterClockwise,
        TriSign::Neg => SignedAreaOrientation::Clockwise,
        TriSign::Zero => SignedAreaOrientation::Zero,
    };
    let degeneracy = if policy_required_cause.is_some() {
        AreaDegeneracyClass::PolicyRequired
    } else if absolute == *thresholds.zero_area() {
        AreaDegeneracyClass::ZeroArea
    } else if absolute <= *thresholds.sliver_area() {
        AreaDegeneracyClass::Sliver
    } else if has_needle_loop(basis) {
        AreaDegeneracyClass::Needle
    } else if tiny_hole {
        AreaDegeneracyClass::TinyHole
    } else {
        AreaDegeneracyClass::WellFormed
    };
    Ok(CertifiedSignedAreaMeasurement {
        orientation,
        degeneracy,
        signed_area_twice_decimal: net.to_string(),
        localized_cause: policy_required_cause.or_else(|| localized_cause_for(degeneracy, basis)),
        loop_edges_walked,
        area_terms_evaluated: terms,
        precision_escalations: usize::from(
            basis
                .precision_receipt()
                .precision_escalation()
                .get_expansion_length()
                .is_some(),
        ) + interval_escalations,
        local_scale_comparisons: 3,
        degeneracy_localization_breadth: basis.loops().len(),
    })
}

fn loop_interval_signed_area_twice(
    vertices: &[crate::planar_contracts::polygon_winding_2d::ProjectedLoopVertexSnapshot],
) -> Interval {
    let mut sum = Interval::from_f64(0.0);
    for edge in vertices.windows(2) {
        sum = sum + interval_area_term(edge[0].point_2d, edge[1].point_2d);
    }
    let first = vertices.first().expect("validated loop has vertices");
    let last = vertices.last().expect("validated loop has vertices");
    sum + interval_area_term(last.point_2d, first.point_2d)
}

fn interval_area_term(a: [f64; 2], b: [f64; 2]) -> Interval {
    Interval::from_f64(a[0]) * Interval::from_f64(b[1])
        - Interval::from_f64(a[1]) * Interval::from_f64(b[0])
}

fn loop_signed_area_twice(
    vertices: &[crate::planar_contracts::polygon_winding_2d::ProjectedLoopVertexSnapshot],
    budget: &mut PrecisionBudget,
) -> Result<Rational, CertifiedSignedArea2DDenial> {
    let mut sum = Rational::zero();
    for edge in vertices.windows(2) {
        sum = &sum + &area_term(edge[0].point_2d, edge[1].point_2d, budget)?;
    }
    let first = vertices.first().expect("validated loop has vertices");
    let last = vertices.last().expect("validated loop has vertices");
    sum = &sum + &area_term(last.point_2d, first.point_2d, budget)?;
    Ok(sum)
}

fn area_term(
    a: [f64; 2],
    b: [f64; 2],
    budget: &mut PrecisionBudget,
) -> Result<Rational, CertifiedSignedArea2DDenial> {
    let ax = Rational::try_from_f64(a[0]).map_err(|_| precision_denial())?;
    let ay = Rational::try_from_f64(a[1]).map_err(|_| precision_denial())?;
    let bx = Rational::try_from_f64(b[0]).map_err(|_| precision_denial())?;
    let by = Rational::try_from_f64(b[1]).map_err(|_| precision_denial())?;
    Ok(budget.enforce(&ax * &by) - budget.enforce(&ay * &bx))
}

fn has_needle_loop(basis: &CertifiedSignedArea2DBasis) -> bool {
    needle_loop_edge(basis).is_some()
}

fn needle_loop_edge(basis: &CertifiedSignedArea2DBasis) -> Option<(String, usize)> {
    for loop_summary in basis.loops() {
        let vertices = loop_summary.vertices();
        for (edge_index, edge) in vertices.windows(2).enumerate() {
            if same_point(edge[0].point_2d, edge[1].point_2d) {
                return Some((loop_summary.loop_identity().to_string(), edge_index));
            }
        }
        if same_point(
            vertices.last().expect("validated loop").point_2d,
            vertices.first().expect("validated loop").point_2d,
        ) {
            return Some((loop_summary.loop_identity().to_string(), vertices.len() - 1));
        }
    }
    None
}

fn same_point(a: [f64; 2], b: [f64; 2]) -> bool {
    a[0].to_bits() == b[0].to_bits() && a[1].to_bits() == b[1].to_bits()
}

fn localized_cause_for(
    degeneracy: AreaDegeneracyClass,
    basis: &CertifiedSignedArea2DBasis,
) -> Option<SignedAreaDegeneracyCause> {
    match degeneracy {
        AreaDegeneracyClass::WellFormed => None,
        AreaDegeneracyClass::Needle => {
            needle_loop_edge(basis).map(|(loop_identity, edge_index)| {
                SignedAreaDegeneracyCause::NeedleEdge {
                    loop_identity,
                    edge_index,
                    frame_identity: basis.frame_identity().to_string(),
                    precision_fact_digest: basis.precision_receipt().fact_digest().to_string(),
                }
            })
        }
        _ => Some(SignedAreaDegeneracyCause::AreaSum {
            loop_identity: basis.primary_loop_identity().to_string(),
            frame_identity: basis.frame_identity().to_string(),
            precision_fact_digest: basis.precision_receipt().fact_digest().to_string(),
        }),
    }
}

fn precision_denial() -> CertifiedSignedArea2DDenial {
    CertifiedSignedArea2DDenial::new(
        CertifiedSignedArea2DDenialKind::PrecisionBudgetExceeded,
        "signed area exact rational conversion exceeded the declared precision budget",
    )
}
