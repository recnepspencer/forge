//! Bitwise and stage-visible evidence for the adaptive incircle cascade.

use super::exact_stage;
use super::stage_b::{self, StageBResult};
use super::stage_c::{self, StageCResult};
use super::translated_geometry::CoordinateTails;

#[derive(Clone)]
struct IncircleCase {
    label: String,
    points: [[f64; 2]; 4],
}

impl IncircleCase {
    fn new(label: impl Into<String>, points: [[f64; 2]; 4]) -> Self {
        Self {
            label: label.into(),
            points,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ResolutionClass {
    StageBResolved,
    StageCZeroTailResolved,
    StageCScalarCorrectedResolved,
    ExactTailResolved,
}

impl ResolutionClass {
    fn index(self) -> usize {
        match self {
            Self::StageBResolved => 0,
            Self::StageCZeroTailResolved => 1,
            Self::StageCScalarCorrectedResolved => 2,
            Self::ExactTailResolved => 3,
        }
    }
}

#[derive(Clone, Copy)]
struct TailSnapshot {
    adxtail: f64,
    adytail: f64,
    bdxtail: f64,
    bdytail: f64,
    cdxtail: f64,
    cdytail: f64,
}

impl TailSnapshot {
    fn from_tails(tails: &CoordinateTails) -> Self {
        Self {
            adxtail: tails.adxtail,
            adytail: tails.adytail,
            bdxtail: tails.bdxtail,
            bdytail: tails.bdytail,
            cdxtail: tails.cdxtail,
            cdytail: tails.cdytail,
        }
    }

    fn all_vertex_families_nonzero(self) -> bool {
        (self.adxtail != 0.0 || self.adytail != 0.0)
            && (self.bdxtail != 0.0 || self.bdytail != 0.0)
            && (self.cdxtail != 0.0 || self.cdytail != 0.0)
    }
}

#[derive(Clone)]
struct Observation {
    case: IncircleCase,
    stage: ResolutionClass,
    permanent: f64,
    adaptive_result: f64,
    tails: Option<TailSnapshot>,
}

fn permanent(points: [[f64; 2]; 4]) -> f64 {
    let pa = points[0];
    let pb = points[1];
    let pc = points[2];
    let pd = points[3];
    let adx = pa[0] - pd[0];
    let bdx = pb[0] - pd[0];
    let cdx = pc[0] - pd[0];
    let ady = pa[1] - pd[1];
    let bdy = pb[1] - pd[1];
    let cdy = pc[1] - pd[1];
    let bdxcdy = bdx * cdy;
    let cdxbdy = cdx * bdy;
    let alift = adx * adx + ady * ady;
    let cdxady = cdx * ady;
    let adxcdy = adx * cdy;
    let blift = bdx * bdx + bdy * bdy;
    let adxbdy = adx * bdy;
    let bdxady = bdx * ady;
    let clift = cdx * cdx + cdy * cdy;
    (bdxcdy.abs() + cdxbdy.abs()) * alift
        + (cdxady.abs() + adxcdy.abs()) * blift
        + (adxbdy.abs() + bdxady.abs()) * clift
}

fn point_bits(points: [[f64; 2]; 4]) -> [[u64; 2]; 4] {
    [
        [points[0][0].to_bits(), points[0][1].to_bits()],
        [points[1][0].to_bits(), points[1][1].to_bits()],
        [points[2][0].to_bits(), points[2][1].to_bits()],
        [points[3][0].to_bits(), points[3][1].to_bits()],
    ]
}

fn observe(case: IncircleCase) -> Observation {
    let points = case.points;
    let permanent = permanent(points);
    let (stage, staged_result, tails) =
        match stage_b::run(points[0], points[1], points[2], points[3], permanent) {
            StageBResult::Resolved(det) => (ResolutionClass::StageBResolved, det, None),
            StageBResult::Continue(state) => {
                let tails = state.geometry.coordinate_tails(&state.input);
                let tail_snapshot = TailSnapshot::from_tails(&tails);
                let all_zero = tails.all_zero();
                match stage_c::run(state) {
                    StageCResult::Resolved(det) => {
                        let class = if all_zero {
                            ResolutionClass::StageCZeroTailResolved
                        } else {
                            ResolutionClass::StageCScalarCorrectedResolved
                        };
                        (class, det, Some(tail_snapshot))
                    }
                    StageCResult::Continue(state) => (
                        ResolutionClass::ExactTailResolved,
                        exact_stage::run(state),
                        Some(tail_snapshot),
                    ),
                }
            }
        };
    let adaptive_result =
        super::incircleadapt(points[0], points[1], points[2], points[3], permanent);
    assert_eq!(
        staged_result.to_bits(),
        adaptive_result.to_bits(),
        "stage result mismatch case={} points_bits={:?} stage={:?}",
        case.label,
        point_bits(points),
        stage
    );
    Observation {
        case,
        stage,
        permanent,
        adaptive_result,
        tails,
    }
}

fn assert_oracle(observation: &Observation) {
    let points = observation.case.points;
    let upstream_adaptive = geometry_predicates::predicates::incircleadapt(
        points[0],
        points[1],
        points[2],
        points[3],
        observation.permanent,
    );
    let local_canonical =
        super::super::evaluation::incircle(points[0], points[1], points[2], points[3]);
    let upstream_canonical =
        geometry_predicates::incircle(points[0], points[1], points[2], points[3]);
    let bits = point_bits(points);
    assert_eq!(
        observation.adaptive_result.to_bits(),
        upstream_adaptive.to_bits(),
        "adaptive oracle mismatch case={} points_bits={:?} stage={:?}",
        observation.case.label,
        bits,
        observation.stage
    );
    assert_eq!(
        local_canonical.to_bits(),
        upstream_canonical.to_bits(),
        "canonical oracle mismatch case={} points_bits={:?} stage={:?}",
        observation.case.label,
        bits,
        observation.stage
    );
}

fn awkward_rectangle() -> [[f64; 2]; 4] {
    [[0.1, 0.2], [1.3, 0.2], [1.3, 2.2], [0.1, 2.2]]
}

fn permuted_case(label: &str, points: [[f64; 2]; 4], order: [usize; 4]) -> IncircleCase {
    IncircleCase::new(
        label,
        [
            points[order[0]],
            points[order[1]],
            points[order[2]],
            points[order[3]],
        ],
    )
}

fn explicit_cases() -> Vec<(IncircleCase, ResolutionClass)> {
    let awkward = awkward_rectangle();
    vec![
        (
            IncircleCase::new(
                "stage-b-inside",
                [[0.0, 0.0], [4.0, 0.0], [0.0, 4.0], [2.0, 2.0]],
            ),
            ResolutionClass::StageBResolved,
        ),
        (
            IncircleCase::new(
                "stage-b-outside",
                [[0.0, 0.0], [4.0, 0.0], [0.0, 4.0], [5.0, 5.0]],
            ),
            ResolutionClass::StageBResolved,
        ),
        (
            IncircleCase::new(
                "integer-square-zero-tail",
                [[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0]],
            ),
            ResolutionClass::StageCZeroTailResolved,
        ),
        (
            IncircleCase::new("awkward-rectangle", awkward),
            ResolutionClass::ExactTailResolved,
        ),
        (
            permuted_case("awkward-cycle-1", awkward, [1, 2, 3, 0]),
            ResolutionClass::ExactTailResolved,
        ),
        (
            permuted_case("awkward-cycle-2", awkward, [2, 3, 0, 1]),
            ResolutionClass::ExactTailResolved,
        ),
        (
            permuted_case("awkward-reversal", awkward, [0, 3, 2, 1]),
            ResolutionClass::ExactTailResolved,
        ),
    ]
}

fn ulp_case(base: [[f64; 2]; 4], axis: usize, direction: i8, steps: u64) -> [[f64; 2]; 4] {
    let mut points = base;
    let bits = points[0][axis].to_bits();
    let shifted = if direction < 0 {
        bits - steps
    } else {
        bits + steps
    };
    points[0][axis] = f64::from_bits(shifted);
    points
}

fn note_class(class: ResolutionClass, counts: &mut [usize; 4], seen: &mut [bool; 4]) {
    let index = class.index();
    counts[index] += 1;
    seen[index] = true;
}

#[test]
fn adaptive_stage_world_matches_upstream_bitwise() {
    const ULP_STEPS: u64 = 128;
    let base = awkward_rectangle();
    let mut counts = [0; 4];
    let mut seen = [false; 4];
    let mut checked = Vec::new();
    let base_observation = observe(IncircleCase::new("awkward-rectangle", base));
    assert_eq!(
        base_observation.stage,
        ResolutionClass::ExactTailResolved,
        "unexpected awkward center stage points_bits={:?}",
        point_bits(base)
    );
    assert!(base_observation.permanent > 0.0);
    assert_oracle(&base_observation);
    note_class(base_observation.stage, &mut counts, &mut seen);
    checked.push(base_observation);

    for (case, expected) in explicit_cases()
        .into_iter()
        .filter(|(case, _)| case.label != "awkward-rectangle")
    {
        let observation = observe(case);
        assert_eq!(
            observation.stage,
            expected,
            "unexpected explicit stage case={} points_bits={:?}",
            observation.case.label,
            point_bits(observation.case.points)
        );
        assert_oracle(&observation);
        note_class(observation.stage, &mut counts, &mut seen);
        checked.push(observation);
    }

    let mut candidate_count = 0;
    let mut retained_ulp = 0;
    let mut retained_scalar = false;
    let mut retained_exact = 0;
    for axis in 0..2 {
        for (direction, direction_name) in [(-1_i8, "down"), (1_i8, "up")] {
            for steps in 1..=ULP_STEPS {
                candidate_count += 1;
                let label = format!("awkward-ulp-axis{}-{}-{}", axis, direction_name, steps);
                let observation = observe(IncircleCase::new(
                    label,
                    ulp_case(base, axis, direction, steps),
                ));
                note_class(observation.stage, &mut counts, &mut seen);
                let retain = match observation.stage {
                    ResolutionClass::StageCScalarCorrectedResolved if !retained_scalar => {
                        retained_scalar = true;
                        true
                    }
                    ResolutionClass::ExactTailResolved if retained_exact < 2 => {
                        retained_exact += 1;
                        true
                    }
                    _ => false,
                };
                if retain {
                    assert_oracle(&observation);
                    checked.push(observation);
                    retained_ulp += 1;
                }
            }
        }
    }

    assert!(
        seen.iter().all(|seen_class| *seen_class),
        "resolution classes missing: counts={counts:?}"
    );
    assert!(
        retained_scalar,
        "ULP neighborhood found no Stage C scalar-correction case"
    );
    assert!(
        checked.iter().any(|observation| {
            observation.stage == ResolutionClass::ExactTailResolved
                && observation
                    .tails
                    .map(TailSnapshot::all_vertex_families_nonzero)
                    .unwrap_or(false)
        }),
        "no exact-tail case exercised nonzero A/B/C coordinate-tail families"
    );
    println!(
        "adaptive oracle worlds: checked={}, explicit=7, ulp_candidates={}, retained_ulp={}, classes={counts:?}",
        checked.len(),
        candidate_count,
        retained_ulp,
    );
}
