from __future__ import annotations

import hashlib
import json
import secrets
import tempfile
from pathlib import Path

from worth_ui_ledger_runner_authentication import authentication_tag


ROOT = Path(__file__).resolve().parents[2]


def predecessor_artifact(
    through_phase: int,
    revision: str,
    state_digest: str,
    observations: list[dict[str, object]],
    closure_count: int,
) -> dict[str, object]:
    main_tests, controls = unique_test_executions(observations)
    presentations = sum(unique_presentations(row) for row in observations)
    product_processes = sum(
        unique_construction_amount(row, "product-processes") for row in observations
    )
    courtroom_worlds = sum(
        unique_construction_amount(row, "courtroom-worlds") for row in observations
    )
    return {
        "schema": "worth-ui-phase-predecessor-handoff-v1",
        "through_phase": through_phase,
        "source_revision": revision,
        "source_state_digest": state_digest,
        "verified_requirement_count": len(observations),
        "main_test_executions": main_tests,
        "hostile_control_executions": controls,
        "closure_test_executions": closure_count,
        "compile_sessions": 2,
        "product_processes": product_processes,
        "courtroom_worlds": courtroom_worlds,
        "presentations": presentations,
        "mapping_digest": mapping_digest(observations),
        "run_nonce": secrets.token_hex(16),
        "rows": [retained_row(row, ROOT) for row in observations],
    }


def unique_presentations(row: dict[str, object]) -> int:
    if "shared_main_artifact" in row:
        return 0
    execution = str(row.get("execution_cost", ""))
    for field in execution.split(";"):
        if field.startswith("presentations="):
            return int(field.split("=", 1)[1])
    return 0


def unique_test_executions(
    observations: list[dict[str, object]],
) -> tuple[int, int]:
    main: set[str] = set()
    controls: set[str] = set()
    for row in observations:
        receipts = row.get("execution_receipts")
        if not isinstance(receipts, list):
            continue
        for receipt in receipts:
            if not isinstance(receipt, dict) or not isinstance(receipt.get("key"), str):
                continue
            if receipt.get("role") == "main-test":
                main.add(receipt["key"])
            elif receipt.get("role") == "control-test":
                controls.add(receipt["key"])
    if main or controls:
        return len(main), len(controls)
    return (
        sum(int(row["executed_test_count"]) for row in observations),
        sum(
            int(row.get("hostile_control", {}).get("executed_test_count", 0))
            for row in observations
            if isinstance(row.get("hostile_control"), dict)
        ),
    )


def unique_construction_amount(row: dict[str, object], name: str) -> int:
    if "shared_main_artifact" in row:
        return 0
    construction = str(row.get("construction_cost", ""))
    for field in construction.split(";"):
        if field.startswith(f"{name}="):
            return int(field.split("=", 1)[1])
    return 0


def mapping_digest(rows: list[dict[str, object]]) -> str:
    digest = hashlib.sha256()
    for row in sorted(rows, key=lambda candidate: str(candidate["requirement"])):
        for field in ("requirement", "production_entry", "independent_oracle"):
            digest.update(str(row[field]).encode())
            digest.update(b"\0")
        for source in row["mapping_source_identity"]:
            digest.update(str(source).encode())
            digest.update(b"\0")
        digest.update(b"\xff")
    return digest.hexdigest()


def retained_row(row: dict[str, object], root: Path) -> dict[str, object]:
    retained = {
        field: row[field]
        for field in (
            "requirement", "production_entry", "independent_oracle", "package",
            "target_kind", "target_name", "features", "test_name",
            "matched_test_count", "declared_ignored_test_count",
            "expected_declared_ignored", "executed_test_count", "passed_test_count",
            "ignored_test_count", "exit_posture", "source_revision", "source_identity",
            "mapping_source_identity", "source_rebindings", "source_digest",
            "source_state_digest", "run_nonce", "artifact_sha256", "structural_counter",
            "claim_digest", "executed_exact_command",
            "hostile_control", "construction_cost", "execution_cost",
            "execution_receipts", "list_command", "ignored_list_command", "test_command",
            "public_example_command",
        )
        if field in row
    }
    if "shared_main_artifact" in row:
        retained["shared_main_artifact"] = row["shared_main_artifact"]
    if "causal_reuse" in row:
        retained["causal_reuse"] = row["causal_reuse"]
    retained["runner_authentication"] = authentication_tag(retained, root)
    return retained


def write_artifact(identity: str, payload: dict[str, object]) -> None:
    destination = (ROOT / identity).resolve()
    try:
        destination.relative_to(ROOT)
    except ValueError as error:
        raise RuntimeError("predecessor artifact escapes the repository") from error
    destination.parent.mkdir(parents=True, exist_ok=True)
    data = json.dumps(payload, indent=2, sort_keys=True) + "\n"
    with tempfile.NamedTemporaryFile(
        "w", encoding="utf-8", dir=destination.parent, delete=False
    ) as stream:
        stream.write(data)
        temporary = Path(stream.name)
    temporary.replace(destination)
