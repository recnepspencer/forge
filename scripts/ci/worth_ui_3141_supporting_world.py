from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
from typing import Any

from worth_ui_ledger_dependency import require_proved_artifact
from worth_ui_3141_phase4_case_contracts import hostile_cases, positive_cases


MIXED_REQUIREMENT = "P3-DELTA-SOURCE-01"
MIXED_ARTIFACT = "_docs/worth-ui/milestone-3.14.1-evidence/p3-delta-source-01.json"
MIXED_TEST = "host_platform::mixed_carrier_successors_are_local_at_the_4096_command_ceiling"
ATLAS_REQUIREMENT = "P5-ATLAS-01"
ATLAS_ARTIFACT = "_docs/worth-ui/milestone-3.14.1-evidence/p5-atlas-01.json"


def validate_supporting_dependency(
    test: Any, revision: str, state_digest: str, root: Path
) -> dict[str, Any] | None:
    if test.requirement == "P5-ATLAS-PINNING-01":
        return validate_phase5_atlas_dependency(test, revision, state_digest, root)
    if test.requirement != "P3-HP02-WORLD-01":
        return None
    identity = os.environ.get("WORTH_UI_SUPPORTING_WORLD_ARTIFACT", MIXED_ARTIFACT)
    if identity not in test.sources:
        raise ValueError("HP-02 omits its mixed-carrier world artifact")
    path = (root / identity).resolve()
    artifact = json.loads(path.read_text(encoding="utf-8"))
    proved_digest = require_proved_artifact(root, MIXED_REQUIREMENT, identity, artifact)
    expected = {
        "schema_version": 5,
        "requirement": MIXED_REQUIREMENT,
        "package": "worth-ui-certification",
        "target_kind": "test",
        "target_name": "application_contracts",
        "test_name": MIXED_TEST,
        "matched_test_count": 1,
        "declared_ignored_test_count": 1,
        "expected_declared_ignored": True,
        "executed_test_count": 1,
        "passed_test_count": 1,
        "ignored_test_count": 0,
        "exit_posture": "passed",
        "test_exit_code": 0,
        "source_revision": revision,
        "source_state_digest": state_digest,
        "structural_counter": "source-rows=1",
        "construction_cost": (
            "main-tests=1;hostile-controls=1;product-processes=0;"
            "compile-sessions=0;courtroom-worlds=1"
        ),
        "execution_cost": "executed-tests=2;presentations=5",
    }
    for field, value in expected.items():
        if artifact.get(field) != value:
            raise ValueError(f"HP-02 mixed-carrier artifact has wrong {field}")
    output = artifact.get("test_stdout")
    if not isinstance(output, str) or "WORTH_UI_LEDGER_WORLD=1" not in output:
        raise ValueError("HP-02 mixed-carrier artifact omits its executed world observation")
    return {
        "artifact": identity,
        "artifact_digest": proved_digest,
        "requirement": MIXED_REQUIREMENT,
        "worlds": 1,
        "presentations": 5,
    }


def validate_phase3_hp02_support(
    test: Any, revision: str, state_digest: str, root: Path
) -> dict[str, Any] | None:
    return validate_supporting_dependency(test, revision, state_digest, root)


def validate_phase5_atlas_dependency(
    test: Any, revision: str, state_digest: str, root: Path
) -> dict[str, Any]:
    identity = os.environ.get("WORTH_UI_SUPPORTING_WORLD_ARTIFACT", ATLAS_ARTIFACT)
    if identity not in test.sources:
        raise ValueError("pinning proof omits its atlas producer artifact")
    artifact = json.loads((root / identity).read_text(encoding="utf-8"))
    digest = require_proved_artifact(root, ATLAS_REQUIREMENT, identity, artifact)
    required = {
        "schema_version": 5,
        "requirement": ATLAS_REQUIREMENT,
        "exit_posture": "passed",
        "executed_test_count": 1,
        "passed_test_count": 1,
        "ignored_test_count": 0,
        "source_revision": revision,
        "source_state_digest": state_digest,
    }
    for field, expected in required.items():
        if artifact.get(field) != expected:
            raise ValueError(f"pinning atlas dependency has wrong {field}")
    if artifact.get("structural_counter") != "physical-signal-runtimes=1":
        raise ValueError("pinning atlas dependency has wrong structural counter")
    if artifact.get("governed_cases") != list(positive_cases(ATLAS_REQUIREMENT) or ()):
        raise ValueError("pinning atlas dependency has wrong positive case inventory")
    control = artifact.get("hostile_control")
    if not isinstance(control, dict) or control.get("mutation_cases") != list(
        hostile_cases(ATLAS_REQUIREMENT) or ()
    ):
        raise ValueError("pinning atlas dependency has wrong hostile case inventory")
    return {
        "artifact": identity,
        "artifact_digest": digest,
        "requirement": ATLAS_REQUIREMENT,
        "worlds": 0,
        "presentations": 0,
    }
