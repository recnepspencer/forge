"""Utility functions for OpenCASCADE differential testing.

Provides point sampling, volume computation, and point classification
helpers that wrap CadQuery/OCC operations for use in differential
comparison tests against the Forge kernel.
"""

import numpy as np
import cadquery as cq


def sample_points_in_bbox(
    bbox_min: tuple[float, float, float],
    bbox_max: tuple[float, float, float],
    count: int,
    seed: int = 42,
) -> np.ndarray:
    """Generate uniformly distributed random points within an axis-aligned bounding box.

    Args:
        bbox_min: Minimum corner (x, y, z).
        bbox_max: Maximum corner (x, y, z).
        count: Number of points to generate.
        seed: RNG seed for deterministic sampling.

    Returns:
        Array of shape (count, 3) with point coordinates.
    """
    rng = np.random.default_rng(seed)
    mins = np.array(bbox_min)
    maxs = np.array(bbox_max)
    return rng.uniform(mins, maxs, size=(count, 3))


def classify_points_occ(
    solid: cq.Workplane,
    points: np.ndarray,
) -> np.ndarray:
    """Classify each point as inside (True) or outside (False) an OCC solid.

    Uses BRepClass3d_SolidClassifier under the hood via CadQuery.

    Args:
        solid: CadQuery workplane containing a single solid.
        points: Array of shape (N, 3) with point coordinates.

    Returns:
        Boolean array of shape (N,).
    """
    from OCP.BRepClass3d import BRepClass3d_SolidClassifier
    from OCP.gp import gp_Pnt
    from OCP.TopAbs import TopAbs_IN

    occ_solid = solid.val().wrapped
    classifier = BRepClass3d_SolidClassifier(occ_solid)
    tolerance = 1e-6

    results = np.empty(len(points), dtype=bool)
    for i, (x, y, z) in enumerate(points):
        classifier.Perform(gp_Pnt(float(x), float(y), float(z)), tolerance)
        results[i] = classifier.State() == TopAbs_IN

    return results


def compute_volume_occ(solid: cq.Workplane) -> float:
    """Compute the volume of an OCC solid.

    Args:
        solid: CadQuery workplane containing a single solid.

    Returns:
        Volume as a float.
    """
    from OCP.GProp import GProp_GProps
    from OCP.BRepGProp import BRepGProp

    props = GProp_GProps()
    BRepGProp.VolumeProperties_s(solid.val().wrapped, props)
    return props.Mass()


def compute_face_count_occ(solid: cq.Workplane) -> int:
    """Count the number of faces on an OCC solid.

    Args:
        solid: CadQuery workplane containing a single solid.

    Returns:
        Number of faces.
    """
    from OCP.TopExp import TopExp_Explorer
    from OCP.TopAbs import TopAbs_FACE

    explorer = TopExp_Explorer(solid.val().wrapped, TopAbs_FACE)
    count = 0
    while explorer.More():
        count += 1
        explorer.Next()
    return count


def make_box_occ(
    length: float,
    width: float,
    height: float,
    center_x: float = 0.0,
    center_y: float = 0.0,
    center_z: float = 0.0,
) -> cq.Workplane:
    """Create an axis-aligned box solid centered at the given position.

    Args:
        length: Box extent along X.
        width: Box extent along Y.
        height: Box extent along Z.
        center_x: Center X coordinate.
        center_y: Center Y coordinate.
        center_z: Center Z coordinate.

    Returns:
        CadQuery workplane containing the box solid.
    """
    return (
        cq.Workplane("XY")
        .transformed(offset=(center_x, center_y, center_z))
        .box(length, width, height)
    )
