//! Polygon hole-bridging helpers.
//!
//! DOMAIN: Pure geometric selection of bridge vertices between an outer polygon
//! and one or more hole polygons using extremal-vertex raycasting in a local
//! 2D frame derived from the face normal.

use worth_math::linalg::{compute_perpendicular_direction, cross, dot, norm_sq, sub};

/// Select a bridge between one outer polygon and one hole polygon.
///
/// Returns `(outer_vertex_index, hole_vertex_index)` where `hole_vertex_index`
/// is the +X extremal hole vertex in the local face frame and `outer_vertex_index`
/// is the chosen outer endpoint on the closest intersected edge.
pub fn bridge_polygon_hole(
    outer: &[[f64; 3]],
    hole: &[[f64; 3]],
    face_normal: [f64; 3],
    parallel_tol: f64,
    ray_hit_tol: f64,
) -> Option<(usize, usize)> {
    if outer.len() < 3 || hole.len() < 3 {
        return None;
    }

    let u_axis = compute_perpendicular_direction(face_normal);
    let v_axis = cross(face_normal, u_axis);

    let hole_index = find_extremal_vertex_index(hole, u_axis)?;
    let h_max_pos = hole[hole_index];
    let ray_origin_u = dot(h_max_pos, u_axis);
    let ray_origin_v = dot(h_max_pos, v_axis);

    let mut best_outer_edge = None;
    let mut best_t = f64::MAX;
    let mut i = 0usize;
    while i < outer.len() {
        let p_origin = outer[i];
        let p_dest = outer[(i + 1) % outer.len()];
        let o_u = dot(p_origin, u_axis);
        let o_v = dot(p_origin, v_axis);
        let d_u = dot(p_dest, u_axis);
        let d_v = dot(p_dest, v_axis);

        if let Some(t_val) = compute_ray_edge_intersection(
            ray_origin_u,
            ray_origin_v,
            o_u,
            o_v,
            d_u,
            d_v,
            parallel_tol,
            ray_hit_tol,
        ) {
            if t_val < best_t {
                best_t = t_val;
                best_outer_edge = Some(i);
            }
        }

        i += 1;
    }

    let hit_edge = best_outer_edge?;
    let outer_index = pick_closer_endpoint_index(outer, hit_edge, h_max_pos);
    Some((outer_index, hole_index))
}

/// Select bridges for multiple holes against a single outer polygon.
///
/// Returns one result per hole in input order.
pub fn bridge_polygon_holes(
    outer: &[[f64; 3]],
    holes: &[Vec<[f64; 3]>],
    face_normal: [f64; 3],
    parallel_tol: f64,
    ray_hit_tol: f64,
) -> Vec<Option<(usize, usize)>> {
    holes
        .iter()
        .map(|hole| bridge_polygon_hole(outer, hole, face_normal, parallel_tol, ray_hit_tol))
        .collect()
}

fn find_extremal_vertex_index(points: &[[f64; 3]], u_axis: [f64; 3]) -> Option<usize> {
    let mut best_index = None;
    let mut best_x = f64::NEG_INFINITY;
    let mut i = 0usize;
    while i < points.len() {
        let local_x = dot(points[i], u_axis);
        if local_x > best_x {
            best_x = local_x;
            best_index = Some(i);
        }
        i += 1;
    }
    best_index
}

fn compute_ray_edge_intersection(
    ray_u: f64,
    ray_v: f64,
    o_u: f64,
    o_v: f64,
    d_u: f64,
    d_v: f64,
    parallel_tol: f64,
    ray_hit_tol: f64,
) -> Option<f64> {
    let dv = d_v - o_v;
    if dv.abs() < parallel_tol {
        return None;
    }

    let s = (ray_v - o_v) / dv;
    if s < 0.0 || s > 1.0 {
        return None;
    }

    let intersect_u = o_u + s * (d_u - o_u);
    let t = intersect_u - ray_u;

    if t > ray_hit_tol {
        Some(t)
    } else {
        None
    }
}

fn pick_closer_endpoint_index(
    outer: &[[f64; 3]],
    hit_edge_index: usize,
    h_max_pos: [f64; 3],
) -> usize {
    let origin_index = hit_edge_index;
    let dest_index = (hit_edge_index + 1) % outer.len();
    let dist_o = norm_sq(sub(outer[origin_index], h_max_pos));
    let dist_d = norm_sq(sub(outer[dest_index], h_max_pos));
    if dist_o <= dist_d {
        origin_index
    } else {
        dest_index
    }
}

#[cfg(test)]
mod tests {
    use super::{bridge_polygon_hole, bridge_polygon_holes};

    #[test]
    fn selects_bridge_for_center_hole_in_square() {
        let outer = [
            [0.0, 0.0, 0.0],
            [4.0, 0.0, 0.0],
            [4.0, 4.0, 0.0],
            [0.0, 4.0, 0.0],
        ];
        let hole = [
            [1.0, 1.0, 0.0],
            [2.0, 1.0, 0.0],
            [2.0, 2.0, 0.0],
            [1.0, 2.0, 0.0],
        ];
        let result = bridge_polygon_hole(&outer, &hole, [0.0, 0.0, 1.0], 1e-15, 1e-15);
        assert!(result.is_some());
        let (outer_idx, hole_idx) = result.unwrap();
        assert_eq!(hole_idx, 1);
        assert!(outer_idx == 1 || outer_idx == 2);
    }

    #[test]
    fn returns_parallel_results_for_multiple_holes() {
        let outer = [
            [0.0, 0.0, 0.0],
            [5.0, 0.0, 0.0],
            [5.0, 5.0, 0.0],
            [0.0, 5.0, 0.0],
        ];
        let holes = vec![
            vec![
                [1.0, 1.0, 0.0],
                [2.0, 1.0, 0.0],
                [2.0, 2.0, 0.0],
                [1.0, 2.0, 0.0],
            ],
            vec![
                [3.0, 3.0, 0.0],
                [4.0, 3.0, 0.0],
                [4.0, 4.0, 0.0],
                [3.0, 4.0, 0.0],
            ],
        ];
        let results = bridge_polygon_holes(&outer, &holes, [0.0, 0.0, 1.0], 1e-15, 1e-15);
        assert_eq!(results.len(), 2);
        assert!(results[0].is_some());
        assert!(results[1].is_some());
    }
}
