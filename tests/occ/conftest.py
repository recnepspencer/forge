"""Shared pytest fixtures for OpenCASCADE differential tests."""

import pytest


@pytest.fixture
def default_tolerance() -> float:
    """Standard tolerance for volumetric comparisons (0.1% relative error)."""
    return 1e-3


@pytest.fixture
def point_sample_count() -> int:
    """Number of points for volumetric point-in-solid sampling."""
    return 1_000


@pytest.fixture
def point_sample_seed() -> int:
    """Deterministic seed for point sampling (D1 compliance)."""
    return 42
