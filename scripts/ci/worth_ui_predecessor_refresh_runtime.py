from __future__ import annotations

import csv
import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any

from worth_ui_ledger_command import claim_digest_for_row
from worth_ui_ledger_execution_binding import COMPILE_ARTIFACT
from worth_ui_ledger_execution_observation_store import CACHE_ENV
from worth_ui_ledger_portfolio_snapshot import DIGEST_ENV, REVISION_ENV


def ensure_compile_artifact(root: Path, revision: str, state_digest: str) -> None:
    identity = root / COMPILE_ARTIFACT
    try:
        retained = json.loads(identity.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        retained = {}
    if (
        retained.get("exit_posture") == "passed"
        and retained.get("source_revision") == revision
        and retained.get("source_state_digest") == state_digest
    ):
        return
    subprocess.run(
        [
            sys.executable,
            "scripts/ci/run_worth_ui_compile_contracts.py",
            "--artifact",
            COMPILE_ARTIFACT,
        ],
        cwd=root,
        check=True,
    )


def execute_row(
    root: Path,
    ledger: Path,
    executor: Any,
    row: dict[str, str],
    artifact: Path,
    refresh_mode: Any,
    predecessor_handoff: Path | None = None,
) -> dict[str, object]:
    candidate_root = root / "workspaces/worth-ui/target/milestone-3141-candidates"
    candidate_root.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(delete=False, dir=candidate_root) as stream:
        candidate = Path(stream.name)
    shutil.copyfile(ledger, candidate)
    try:
        return executor(
            row,
            artifact,
            COMPILE_ARTIFACT,
            candidate_ledger=candidate,
            refresh_mode=refresh_mode.name(),
            predecessor_handoff=(
                predecessor_handoff.resolve().relative_to(root.resolve()).as_posix()
                if predecessor_handoff is not None
                else None
            ),
        )
    finally:
        candidate.unlink(missing_ok=True)


def closure_tests(root: Path, ledger: Path, through_phase: int) -> int:
    prefix = [
        "cargo", "test", "--manifest-path", "workspaces/worth-ui/Cargo.toml",
        "-p", "worth-ui-certification", "--test", "topology_contracts",
    ]
    tests = [
        "milestone_3141_phase1_ledger::mutation_tests::milestone_ledger_has_exact_schema_inventory_and_honest_posture",
        {
            2: "phase_two_closure_requires_every_phase_one_and_two_row",
            3: "phase_three_closure_requires_every_predecessor_and_phase_three_row",
            4: "phase_four_closure_requires_every_predecessor_and_phase_four_row",
            5: "phase_five_closure_requires_every_predecessor_and_phase_five_row",
            6: "phase_six_closure_requires_every_predecessor_and_phase_six_row",
        }[through_phase],
    ]
    for index, name in enumerate(tests):
        command = [*prefix, f"milestone_3141_phase1_ledger::{name}", "--", "--exact"]
        if index == 1:
            command.append("--ignored")
        command.append("--nocapture")
        environment = dict(os.environ)
        environment["WORTH_UI_MILESTONE_3141_LEDGER"] = str(ledger.resolve())
        completed = subprocess.run(command, cwd=root, env=environment, check=False)
        if completed.returncode != 0:
            raise RuntimeError("current-source predecessor closure check failed")
    return 2


def governed_rows(ledger: Path, through_phase: int) -> list[dict[str, str]]:
    with ledger.open(encoding="utf-8", newline="") as stream:
        rows = [
            row for row in csv.DictReader(stream) if int(row["phase"]) <= through_phase
        ]
    expected = {2: 30, 3: 47, 4: 68, 5: 80, 6: 90}[through_phase]
    if len(rows) != expected or any(
        row["result"] != "PROVED" or row["final_source"] != "true" for row in rows
    ):
        raise RuntimeError("predecessor causal refresh requires a proved prefix")
    return rows


def environment_snapshot() -> dict[str, str | None]:
    return {name: os.environ.get(name) for name in (CACHE_ENV, REVISION_ENV, DIGEST_ENV)}


def restore_environment(previous: dict[str, str | None]) -> None:
    for name, value in previous.items():
        if value is None:
            os.environ.pop(name, None)
        else:
            os.environ[name] = value
