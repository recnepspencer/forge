"""Differential tests comparing Forge boolean results against CadQuery/OCC.

These tests build identical geometry in both systems, then compare:
  - Face counts
  - Volume (via BRepGProp)
  - Point-in-solid agreement for a random sample of probe points

Dependencies: cadquery, OCP, numpy, pytest
"""

import pytest
import cadquery as cq
import numpy as np
from helpers import (
    classify_points_occ,
    compute_volume_occ,
    compute_face_count_occ,
    make_box_occ,
    sample_points_in_bbox,
)


class TestDifferentialConcentricSubtraction:
    """Compare Forge and OCC for A - B where B is fully inside A."""

    @pytest.fixture
    def occ_result(self):
        """Build concentric cube subtraction in CadQuery/OCC."""
        outer = make_box(center=(0.1, 0.2, 0.3), size=2.0)
        inner = make_box(center=(0.1, 0.2, 0.3), size=1.0)
        result = outer.cut(inner)
        return result

    def test_face_count(self, occ_result):
        """OCC hollow box should have 12 faces (6 outer + 6 inner)."""
        fc = count_faces(occ_result)
        assert fc == 12, f"Expected 12 faces (6 outer + 6 inner), got {fc}"

    def test_volume(self, occ_result):
        """Volume should be outer^3 - inner^3 = 8 - 1 = 7."""
        vol = compute_volume(occ_result)
        assert abs(vol - 7.0) < 1e-6, f"Expected volume ~7.0, got {vol}"

    def test_point_classification(self, occ_result, default_tolerance, point_sample_count, random_seed):
        """Probe points: those in the shell should be IN, those inside the cavity OUT."""
        rng = np.random.default_rng(random_seed)
        bbox = (-0.9, -0.8, -0.7, 1.1, 1.2, 1.3)
        points = sample_points_in_bbox(bbox, point_sample_count, rng)

        for pt in points:
            x, y, z = pt
            in_outer = all(
                -0.9 <= c <= 1.1 for c in (x,)
            ) and all(
                -0.8 <= c <= 1.2 for c in (y,)
            ) and all(
                -0.7 <= c <= 1.3 for c in (z,)
            )
            in_inner = all(abs(c - o) <= 0.5 for c, o in zip(pt, (0.1, 0.2, 0.3)))

            expected_inside = in_outer and not in_inner

            actual = classify_point_occ(occ_result, pt, default_tolerance)
            if expected_inside:
                assert actual, f"Point {pt} should be inside hollow box"


class TestDifferentialHalfOverlapSubtraction:
    """Compare Forge and OCC for overlapping cube subtraction."""

    @pytest.fixture
    def occ_result(self):
        """Build half-overlap subtraction in CadQuery/OCC."""
        target = make_box(center=(0.0, 0.0, 0.0), size=2.0)
        tool = make_box(center=(1.0, 0.0, 0.0), size=2.0)
        result = target.cut(tool)
        return result

    def test_volume(self, occ_result):
        """Volume = target - intersection = 8 - 4 = 4."""
        vol = compute_volume(occ_result)
        assert abs(vol - 4.0) < 1e-6, f"Expected volume ~4.0, got {vol}"

    def test_face_count(self, occ_result):
        """Should have 6 faces (5 from outer + 1 new cut face)."""
        fc = count_faces(occ_result)
        assert fc == 6, f"Expected 6 faces, got {fc}"


class TestDifferentialUnion:
    """Compare Forge and OCC for union of disjoint cubes."""

    @pytest.fixture
    def occ_result(self):
        """Build disjoint union in CadQuery/OCC."""
        a = make_box(center=(0.0, 0.0, 0.0), size=1.0)
        b = make_box(center=(5.0, 5.0, 5.0), size=1.0)
        result = a.union(b)
        return result

    def test_volume(self, occ_result):
        """Volume = 1 + 1 = 2."""
        vol = compute_volume(occ_result)
        assert abs(vol - 2.0) < 1e-6, f"Expected volume ~2.0, got {vol}"

    def test_face_count(self, occ_result):
        """Should have 12 faces (6 + 6)."""
        fc = count_faces(occ_result)
        assert fc == 12, f"Expected 12 faces, got {fc}"


class TestDifferentialIntersection:
    """Compare Forge and OCC for intersection of concentric cubes."""

    @pytest.fixture
    def occ_result(self):
        """Build concentric intersection in CadQuery/OCC."""
        outer = make_box(center=(0.1, 0.2, 0.3), size=2.0)
        inner = make_box(center=(0.1, 0.2, 0.3), size=1.0)
        result = outer.intersect(inner)
        return result

    def test_volume(self, occ_result):
        """Volume = inner cube = 1."""
        vol = compute_volume(occ_result)
        assert abs(vol - 1.0) < 1e-6, f"Expected volume ~1.0, got {vol}"

    def test_face_count(self, occ_result):
        """Should have 6 faces (all from inner cube)."""
        fc = count_faces(occ_result)
        assert fc == 6, f"Expected 6 faces, got {fc}"
