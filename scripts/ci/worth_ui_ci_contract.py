from pathlib import Path
from typing import Any

from worth_ui_test_topology_config import (
    Violation,
    required_string,
    required_string_list,
)


def ci_contract_violations(root: Path, config: dict[str, Any]) -> list[Violation]:
    contract = config.get("ci_contract")
    if not isinstance(contract, dict):
        raise ValueError("ci_contract must be an object")
    workflow = root / required_string(contract, "workflow")
    setup_action = root / required_string(contract, "setup_action")
    return [
        *ci_source_violations(
            root,
            workflow,
            "workflow",
            required_string_list(contract, "required_workflow_fragments"),
            required_string_list(contract, "forbidden_workflow_fragments"),
        ),
        *ci_source_violations(
            root,
            setup_action,
            "setup-action",
            required_string_list(contract, "required_setup_fragments"),
            required_string_list(contract, "forbidden_setup_fragments"),
        ),
    ]


def ci_source_violations(
    root: Path,
    source: Path,
    label: str,
    required_fragments: list[str],
    forbidden_fragments: list[str],
) -> list[Violation]:
    relative = source.relative_to(root).as_posix()
    if not source.is_file():
        return [Violation("ci-contract", f"missing {label}: {relative}")]
    text = source.read_text(encoding="utf-8")
    violations: list[Violation] = []
    for fragment in required_fragments:
        if fragment not in text:
            violations.append(
                Violation("ci-contract", f"{relative}: missing required fragment {fragment!r}")
            )
    for fragment in forbidden_fragments:
        if fragment in text:
            violations.append(
                Violation("ci-contract", f"{relative}: forbidden fragment {fragment!r}")
            )
    return violations
