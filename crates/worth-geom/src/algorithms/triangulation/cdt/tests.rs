//! CDT triangulation tests.

use super::geometry::{point_in_polygon_2d, triangle_centroid};
use super::{triangulate_face_with_cut, triangulate_polygon_2d};

#[test]
fn triangulate_square() {
    let verts = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    let boundary = vec![0, 1, 2, 3];
    let constraints: Vec<[usize; 2]> = vec![[0, 1], [1, 2], [2, 3], [3, 0]];

    let result = triangulate_polygon_2d(&verts, &constraints, &boundary).unwrap();
    assert_eq!(
        result.triangles.len(),
        2,
        "Square should produce 2 triangles, got {}",
        result.triangles.len()
    );
}

#[test]
fn triangulate_square_with_diagonal_cut() {
    let verts = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    let result = triangulate_face_with_cut(&verts, 0, 2).unwrap();
    assert_eq!(result.triangles.len(), 2);

    let has_diagonal = result
        .triangles
        .iter()
        .any(|t| t.contains(&0) && t.contains(&2));
    assert!(
        has_diagonal,
        "Cut diagonal 0-2 should appear as triangle edge"
    );
}

#[test]
fn triangulate_l_shape() {
    let verts = [
        [0.0, 0.0],
        [2.0, 0.0],
        [2.0, 1.0],
        [1.0, 1.0],
        [1.0, 2.0],
        [0.0, 2.0],
    ];
    let boundary = vec![0, 1, 2, 3, 4, 5];
    let constraints: Vec<[usize; 2]> = (0..6).map(|i| [i, (i + 1) % 6]).collect();

    let result = triangulate_polygon_2d(&verts, &constraints, &boundary).unwrap();
    assert!(
        result.triangles.len() >= 4,
        "L-shape needs at least 4 triangles, got {}",
        result.triangles.len()
    );

    for tri in &result.triangles {
        let centroid = triangle_centroid(verts[tri[0]], verts[tri[1]], verts[tri[2]]);
        assert!(
            point_in_polygon_2d(&centroid, &verts, &boundary).unwrap(),
            "Triangle centroid should be inside L-shape"
        );
    }
}

#[test]
fn concave_polygon_with_reentrant_cut() {
    let verts = [
        [0.0, 0.0],
        [3.0, 0.0],
        [3.0, 3.0],
        [2.0, 1.0],
        [1.0, 3.0],
        [0.0, 3.0],
    ];
    let result = triangulate_face_with_cut(&verts, 0, 2).unwrap();

    assert!(
        result.triangles.len() >= 4,
        "Concave polygon with cut needs at least 4 triangles, got {}",
        result.triangles.len()
    );

    let has_cut = result
        .triangles
        .iter()
        .any(|t| t.contains(&0) && t.contains(&2));
    assert!(has_cut, "Cut edge 0-2 should appear as triangle edge");
}
