"""Compatibility facade for the phase-ledger contracts.

The requirement, platform, and execution-cost authorities live in separate
modules so each contract family has one semantic responsibility. Existing
ledger tooling imports this facade to keep the public script boundary stable.
"""

from worth_ui_3141_execution_cost_contracts import (
    construction_cost,
    execution_cost,
    platform_versions,
)
from worth_ui_3141_platform_contracts import (
    BASIC_PLATFORM_VERSIONS,
    MOUNTED_BASELINE_REQUIREMENTS,
    NATIVE_PHASE6_PLATFORM_VERSIONS,
    NATIVE_PLATFORM_VERSIONS,
    P3_NATIVE_REQUIREMENTS,
    PROFILE_PLATFORM_VERSIONS,
    TEXT_PLATFORM_VERSIONS,
    baseline_path,
)
from worth_ui_3141_requirement_contracts import COUNTERS, EXPECTED_IGNORED, MUTATIONS
from worth_ui_3141_fault_boundaries import fault_boundaries


FAULT_BOUNDARIES = fault_boundaries(COUNTERS)
