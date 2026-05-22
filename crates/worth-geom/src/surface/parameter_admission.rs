use std::fmt;

use serde::{Deserialize, Serialize};

use crate::algorithms::point_strictly_inside_polygon;
use crate::primitives::parameter_space::ParameterSpacePoint;

use super::schema::ParameterDomain;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParameterAxis {
    U,
    V,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParameterDomainError {
    OutsideDomain {
        point: ParameterSpacePoint,
    },
    NonPeriodicCoordinateOutsideDomain {
        axis: ParameterAxis,
        value: f64,
        min: f64,
        max: f64,
    },
    InvalidPeriodicSpan {
        axis: ParameterAxis,
        min: f64,
        max: f64,
    },
    TrimBoundaryTooShort {
        boundary_kind: &'static str,
        vertex_count: usize,
    },
    PointOutsideTrimmedRegion {
        point: ParameterSpacePoint,
    },
}

impl fmt::Display for ParameterDomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutsideDomain { point } => {
                write!(
                    f,
                    "parameter point [{}, {}] lies outside the surface parameter domain",
                    point.u(),
                    point.v()
                )
            }
            Self::NonPeriodicCoordinateOutsideDomain {
                axis,
                value,
                min,
                max,
            } => {
                write!(
                    f,
                    "non-periodic {} coordinate {} lies outside [{}, {}]",
                    axis, value, min, max
                )
            }
            Self::InvalidPeriodicSpan { axis, min, max } => {
                write!(
                    f,
                    "periodic {} domain requires a finite positive span, got [{}, {}]",
                    axis, min, max
                )
            }
            Self::TrimBoundaryTooShort {
                boundary_kind,
                vertex_count,
            } => {
                write!(
                    f,
                    "{} boundary requires at least 3 vertices, got {}",
                    boundary_kind, vertex_count
                )
            }
            Self::PointOutsideTrimmedRegion { point } => {
                write!(
                    f,
                    "parameter point [{}, {}] lies outside the trimmed parameter region",
                    point.u(),
                    point.v()
                )
            }
        }
    }
}

impl std::error::Error for ParameterDomainError {}

impl fmt::Display for ParameterAxis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::U => write!(f, "u"),
            Self::V => write!(f, "v"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainParameterPoint {
    domain: ParameterDomain,
    point: ParameterSpacePoint,
}

impl DomainParameterPoint {
    pub fn domain(&self) -> &ParameterDomain {
        &self.domain
    }

    pub fn point(&self) -> ParameterSpacePoint {
        self.point
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalParameterPoint {
    domain: ParameterDomain,
    point: ParameterSpacePoint,
}

impl CanonicalParameterPoint {
    pub fn domain(&self) -> &ParameterDomain {
        &self.domain
    }

    pub fn point(&self) -> ParameterSpacePoint {
        self.point
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolygonalTrimmedParameterRegion {
    domain: ParameterDomain,
    outer_boundary: Vec<ParameterSpacePoint>,
    holes: Vec<Vec<ParameterSpacePoint>>,
    outer_boundary_arrays: Vec<[f64; 2]>,
    hole_boundary_arrays: Vec<Vec<[f64; 2]>>,
}

impl PolygonalTrimmedParameterRegion {
    pub fn new(
        domain: ParameterDomain,
        outer_boundary: Vec<ParameterSpacePoint>,
        holes: Vec<Vec<ParameterSpacePoint>>,
    ) -> Result<Self, ParameterDomainError> {
        let outer_boundary = canonicalize_boundary(&domain, outer_boundary, "outer")?;
        let holes = holes
            .into_iter()
            .map(|boundary| canonicalize_boundary(&domain, boundary, "hole"))
            .collect::<Result<Vec<_>, _>>()?;
        let outer_boundary_arrays = boundary_arrays(&outer_boundary);
        let hole_boundary_arrays = holes.iter().map(|hole| boundary_arrays(hole)).collect();
        Ok(Self {
            domain,
            outer_boundary,
            holes,
            outer_boundary_arrays,
            hole_boundary_arrays,
        })
    }

    pub fn domain(&self) -> &ParameterDomain {
        &self.domain
    }

    pub fn admit(
        &self,
        point: CanonicalParameterPoint,
    ) -> Result<PolygonalTrimmedParameterPoint, ParameterDomainError> {
        if point.domain() != &self.domain || !self.contains(point.point()) {
            return Err(ParameterDomainError::PointOutsideTrimmedRegion {
                point: point.point(),
            });
        }
        Ok(PolygonalTrimmedParameterPoint {
            region: self.clone(),
            point,
        })
    }

    pub fn contains(&self, point: ParameterSpacePoint) -> bool {
        let raw = point.as_array();
        if !point_in_or_on_polygon(&raw, &self.outer_boundary_arrays) {
            return false;
        }
        !self.hole_boundary_arrays.iter().any(|hole| {
            point_strictly_inside_polygon(&raw, hole) || point_on_polygon_boundary(&raw, hole)
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolygonalTrimmedParameterPoint {
    region: PolygonalTrimmedParameterRegion,
    point: CanonicalParameterPoint,
}

impl PolygonalTrimmedParameterPoint {
    pub fn region(&self) -> &PolygonalTrimmedParameterRegion {
        &self.region
    }

    pub fn point(&self) -> &CanonicalParameterPoint {
        &self.point
    }
}

impl ParameterDomain {
    pub fn contains(&self, point: ParameterSpacePoint) -> bool {
        let [u, v] = point.as_array();
        self.u_min <= u && u <= self.u_max && self.v_min <= v && v <= self.v_max
    }

    pub fn admit(
        &self,
        point: ParameterSpacePoint,
    ) -> Result<DomainParameterPoint, ParameterDomainError> {
        if !self.contains(point) {
            return Err(ParameterDomainError::OutsideDomain { point });
        }
        Ok(DomainParameterPoint {
            domain: self.clone(),
            point,
        })
    }

    pub fn canonicalize(
        &self,
        point: ParameterSpacePoint,
    ) -> Result<CanonicalParameterPoint, ParameterDomainError> {
        let [u, v] = point.as_array();
        let canonical = ParameterSpacePoint::try_new([
            canonicalize_coordinate(u, self.u_min, self.u_max, self.u_periodic, ParameterAxis::U)?,
            canonicalize_coordinate(v, self.v_min, self.v_max, self.v_periodic, ParameterAxis::V)?,
        ])
        .expect("canonicalized parameter coordinates must remain finite");
        self.admit(canonical)?;
        Ok(CanonicalParameterPoint {
            domain: self.clone(),
            point: canonical,
        })
    }
}

fn canonicalize_boundary(
    domain: &ParameterDomain,
    boundary: Vec<ParameterSpacePoint>,
    boundary_kind: &'static str,
) -> Result<Vec<ParameterSpacePoint>, ParameterDomainError> {
    if boundary.len() < 3 {
        return Err(ParameterDomainError::TrimBoundaryTooShort {
            boundary_kind,
            vertex_count: boundary.len(),
        });
    }
    boundary
        .into_iter()
        .map(|point| {
            domain
                .canonicalize(point)
                .map(|canonical| canonical.point())
        })
        .collect()
}

fn boundary_arrays(boundary: &[ParameterSpacePoint]) -> Vec<[f64; 2]> {
    boundary.iter().map(|point| point.as_array()).collect()
}

fn point_in_or_on_polygon(point: &[f64; 2], polygon: &[[f64; 2]]) -> bool {
    point_strictly_inside_polygon(point, polygon) || point_on_polygon_boundary(point, polygon)
}

fn point_on_polygon_boundary(point: &[f64; 2], polygon: &[[f64; 2]]) -> bool {
    const TOLERANCE_SQ: f64 = 1.0e-20;
    polygon
        .iter()
        .zip(polygon.iter().cycle().skip(1))
        .take(polygon.len())
        .any(|(start, end)| point_near_segment_sq(point, start, end) <= TOLERANCE_SQ)
}

fn point_near_segment_sq(point: &[f64; 2], start: &[f64; 2], end: &[f64; 2]) -> f64 {
    let dx = end[0] - start[0];
    let dy = end[1] - start[1];
    let len_sq = dx * dx + dy * dy;

    if len_sq <= 0.0 {
        let px = point[0] - start[0];
        let py = point[1] - start[1];
        return px * px + py * py;
    }

    let px = point[0] - start[0];
    let py = point[1] - start[1];
    let t = ((px * dx) + (py * dy)) / len_sq;
    let clamped_t = t.clamp(0.0, 1.0);
    let proj_x = start[0] + clamped_t * dx;
    let proj_y = start[1] + clamped_t * dy;
    let ex = point[0] - proj_x;
    let ey = point[1] - proj_y;
    ex * ex + ey * ey
}

fn canonicalize_coordinate(
    value: f64,
    min: f64,
    max: f64,
    periodic: bool,
    axis: ParameterAxis,
) -> Result<f64, ParameterDomainError> {
    if !periodic {
        if value < min || value > max {
            return Err(ParameterDomainError::NonPeriodicCoordinateOutsideDomain {
                axis,
                value,
                min,
                max,
            });
        }
        return Ok(value);
    }

    let span = max - min;
    if !span.is_finite() || span <= 0.0 {
        return Err(ParameterDomainError::InvalidPeriodicSpan { axis, min, max });
    }

    let wrapped = min + (value - min).rem_euclid(span);
    Ok(if wrapped == max { min } else { wrapped })
}

#[cfg(test)]
mod tests {
    use super::{ParameterDomain, ParameterDomainError, PolygonalTrimmedParameterRegion};
    use crate::primitives::parameter_space::ParameterSpacePoint;

    #[test]
    fn canonicalize_wraps_periodic_coordinates() {
        let domain = ParameterDomain::cylinder();
        let point = ParameterSpacePoint::try_new([std::f64::consts::TAU + 0.25, 3.0]).unwrap();
        let canonical = domain.canonicalize(point).unwrap();
        assert_eq!(canonical.point().as_array(), [0.25, 3.0]);
    }

    #[test]
    fn admit_rejects_out_of_domain_coordinates() {
        let domain = ParameterDomain::sphere();
        let point = ParameterSpacePoint::try_new([1.0, std::f64::consts::PI]).unwrap();
        assert!(matches!(
            domain.admit(point),
            Err(ParameterDomainError::OutsideDomain { .. })
        ));
    }

    #[test]
    fn polygonal_trim_region_admits_interior_points() {
        let domain = ParameterDomain::plane();
        let region = PolygonalTrimmedParameterRegion::new(
            domain.clone(),
            vec![
                ParameterSpacePoint::try_new([0.0, 0.0]).unwrap(),
                ParameterSpacePoint::try_new([2.0, 0.0]).unwrap(),
                ParameterSpacePoint::try_new([2.0, 2.0]).unwrap(),
                ParameterSpacePoint::try_new([0.0, 2.0]).unwrap(),
            ],
            vec![],
        )
        .unwrap();
        let point = domain
            .canonicalize(ParameterSpacePoint::try_new([1.0, 1.0]).unwrap())
            .unwrap();
        assert!(region.admit(point).is_ok());
    }

    #[test]
    fn polygonal_trim_region_accepts_outer_boundary_points() {
        let domain = ParameterDomain::plane();
        let region = PolygonalTrimmedParameterRegion::new(
            domain.clone(),
            vec![
                ParameterSpacePoint::try_new([0.0, 0.0]).unwrap(),
                ParameterSpacePoint::try_new([2.0, 0.0]).unwrap(),
                ParameterSpacePoint::try_new([2.0, 2.0]).unwrap(),
                ParameterSpacePoint::try_new([0.0, 2.0]).unwrap(),
            ],
            vec![],
        )
        .unwrap();
        let point = domain
            .canonicalize(ParameterSpacePoint::try_new([1.0, 0.0]).unwrap())
            .unwrap();
        assert!(region.admit(point).is_ok());
    }
}
