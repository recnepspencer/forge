use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_w_circles_exact_geometry_support::{WExactPoint, K4};

const UNIT_DISTANCE_FLOAT_TOLERANCE: f64 = 1e-9;

pub(super) fn g27_points(
    coefficients: &[[i32; 4]],
) -> Result<Vec<WExactPoint>, G27GeometricFractionalError> {
    if coefficients.len() != 27 {
        return Err(G27GeometricFractionalError::MalformedData {
            source: "g27_point_coefficients",
        });
    }
    Ok(coefficients
        .iter()
        .map(|row| {
            row.iter().enumerate().fold(
                WExactPoint {
                    x: K4::zero(),
                    y: K4::zero(),
                },
                |sum, (index, value)| sum.add(g27_basis_point(index).scale(*value as i128)),
            )
        })
        .collect())
}

pub(super) fn approx_unit_distance(left: WExactPoint, right: WExactPoint) -> bool {
    let (left_x, left_y) = left.approx();
    let (right_x, right_y) = right.approx();
    let dx = left_x - right_x;
    let dy = left_y - right_y;
    (dx.mul_add(dx, dy * dy) - 1.0).abs() <= UNIT_DISTANCE_FLOAT_TOLERANCE
}

fn g27_basis_point(index: usize) -> WExactPoint {
    match index {
        0 => WExactPoint {
            x: K4::rational(1, 1),
            y: K4::zero(),
        },
        1 => WExactPoint {
            x: K4::rational(1, 2),
            y: K4([rat(0, 1), rat(1, 2), rat(0, 1), rat(0, 1)]),
        },
        2 => WExactPoint {
            x: K4::rational(5, 6),
            y: K4([rat(0, 1), rat(0, 1), rat(1, 6), rat(0, 1)]),
        },
        3 => WExactPoint {
            x: K4([rat(5, 12), rat(0, 1), rat(0, 1), rat(-1, 12)]),
            y: K4([rat(0, 1), rat(5, 12), rat(1, 12), rat(0, 1)]),
        },
        _ => WExactPoint {
            x: K4::zero(),
            y: K4::zero(),
        },
    }
}

fn rat(numerator: i128, denominator: i128) -> super::g27_w_circles_exact_geometry_support::Rat {
    super::g27_w_circles_exact_geometry_support::Rat::new(numerator, denominator)
        .expect("literal denominator")
}
