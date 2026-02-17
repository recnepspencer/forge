"""Smoke tests for the OpenCASCADE / CadQuery environment.

Validates that CadQuery is correctly installed and can perform
basic Boolean operations. These tests serve as a foundation for
the differential testing harness in Milestone 2.4.
"""

import numpy as np
import cadquery as cq

from helpers import (
    classify_points_occ,
    compute_face_count_occ,
    compute_volume_occ,
    make_box_occ,
    sample_points_in_bbox,
)


def test_cadquery_import():
    """CadQuery library loads and reports a version."""
    assert hasattr(cq, "__version__")
    assert len(cq.__version__) > 0


def test_create_single_box():
    """A unit cube has volume 1.0 and 6 faces."""
    box = make_box_occ(1.0, 1.0, 1.0)
    volume = compute_volume_occ(box)
    faces = compute_face_count_occ(box)

    assert abs(volume - 1.0) < 1e-9, f"Expected volume 1.0, got {volume}"
    assert faces == 6, f"Expected 6 faces, got {faces}"


def test_boolean_union_two_cubes():
    """Union of two overlapping unit cubes produces correct volume.

    Box A: centered at (0, 0, 0), extent 1x1x1
    Box B: centered at (0.5, 0, 0), extent 1x1x1
    Overlap region: 0.5 x 1.0 x 1.0 = 0.5
    Union volume: 1.0 + 1.0 - 0.5 = 1.5
    """
    box_a = make_box_occ(1.0, 1.0, 1.0, center_x=0.0)
    box_b = make_box_occ(1.0, 1.0, 1.0, center_x=0.5)

    union = box_a.union(box_b)
    volume = compute_volume_occ(union)

    expected_volume = 1.5
    assert abs(volume - expected_volume) < 1e-6, (
        f"Union volume: expected {expected_volume}, got {volume}"
    )


def test_boolean_intersection_two_cubes():
    """Intersection of two overlapping unit cubes produces correct volume.

    Overlap region: 0.5 x 1.0 x 1.0 = 0.5
    """
    box_a = make_box_occ(1.0, 1.0, 1.0, center_x=0.0)
    box_b = make_box_occ(1.0, 1.0, 1.0, center_x=0.5)

    intersection = box_a.intersect(box_b)
    volume = compute_volume_occ(intersection)

    expected_volume = 0.5
    assert abs(volume - expected_volume) < 1e-6, (
        f"Intersection volume: expected {expected_volume}, got {volume}"
    )


def test_boolean_subtraction_two_cubes():
    """Subtraction of two overlapping unit cubes produces correct volume.

    A - B volume: 1.0 - 0.5 = 0.5
    """
    box_a = make_box_occ(1.0, 1.0, 1.0, center_x=0.0)
    box_b = make_box_occ(1.0, 1.0, 1.0, center_x=0.5)

    subtraction = box_a.cut(box_b)
    volume = compute_volume_occ(subtraction)

    expected_volume = 0.5
    assert abs(volume - expected_volume) < 1e-6, (
        f"Subtraction volume: expected {expected_volume}, got {volume}"
    )


def test_point_classification_inside_box():
    """Points inside a unit cube are classified as inside."""
    box = make_box_occ(1.0, 1.0, 1.0)

    inside_points = np.array([
        [0.0, 0.0, 0.0],
        [0.1, 0.2, 0.3],
        [-0.4, -0.4, -0.4],
    ])

    results = classify_points_occ(box, inside_points)
    assert all(results), f"Expected all inside, got {results}"


def test_point_classification_outside_box():
    """Points outside a unit cube are classified as outside."""
    box = make_box_occ(1.0, 1.0, 1.0)

    outside_points = np.array([
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0],
        [-1.0, -1.0, -1.0],
    ])

    results = classify_points_occ(box, outside_points)
    assert not any(results), f"Expected all outside, got {results}"


def test_volumetric_sampling_union(
    point_sample_count,
    point_sample_seed,
):
    """Volumetric point sampling agrees with analytical volume for union.

    Sample 1000 points in the bounding box of the union and verify
    the inside/outside ratio matches the expected volume ratio.
    """
    box_a = make_box_occ(1.0, 1.0, 1.0, center_x=0.0)
    box_b = make_box_occ(1.0, 1.0, 1.0, center_x=0.5)
    union = box_a.union(box_b)

    bbox_min = (-0.5, -0.5, -0.5)
    bbox_max = (1.0, 0.5, 0.5)
    bbox_volume = 1.5 * 1.0 * 1.0

    points = sample_points_in_bbox(
        bbox_min, bbox_max, point_sample_count, point_sample_seed
    )
    inside = classify_points_occ(union, points)
    sampled_volume = (np.sum(inside) / len(inside)) * bbox_volume

    expected_volume = 1.5
    relative_error = abs(sampled_volume - expected_volume) / expected_volume

    assert relative_error < 0.1, (
        f"Sampled volume {sampled_volume:.4f} vs expected {expected_volume:.4f} "
        f"(relative error: {relative_error:.4f})"
    )
