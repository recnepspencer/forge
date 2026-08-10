from __future__ import annotations

import csv
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
LEDGER = ROOT / "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv"
TARGET = ROOT / "workspaces/worth-ui/target"
COMPILE_ARTIFACT = "_docs/worth-ui/milestone-3.14.1-evidence/compile-contracts.json"
NATIVE_WORLD_ARTIFACT = "_docs/worth-ui/milestone-3.14.1-evidence/p2-world-01.json"
MOUNTED_WORLD_ARTIFACT = "_docs/worth-ui/milestone-3.14.1-evidence/p1-worlds-01.json"


def rows() -> list[dict[str, str]]:
    with LEDGER.open(encoding="utf-8", newline="") as stream:
        result = list(csv.DictReader(stream))
    if len(result) != 30 or any(
        row["result"] != "PROVED" or row["final_source"] != "true" for row in result
    ):
        raise RuntimeError("operational verification requires 30 final-source proved rows")
    return result


def run(command: list[str]) -> None:
    completed = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, check=False)
    if completed.returncode != 0:
        sys.stderr.write(completed.stdout)
        sys.stderr.write(completed.stderr)
        raise RuntimeError(f"operational verification failed: {' '.join(command)}")


def rerun_row(
    row: dict[str, str],
    temporary: Path,
    compile_artifact: str,
    shared_world_artifact: str | None = None,
) -> None:
    command = row["exact_command"].split()
    artifact_index = command.index("--artifact") + 1
    command[artifact_index] = temporary.as_posix()
    command = bind_fresh_compile_artifact(command, compile_artifact)
    if shared_world_artifact is not None:
        command = bind_fresh_shared_world(command, shared_world_artifact)
    environment = dict(os.environ)
    environment["WORTH_UI_COMPILE_ARTIFACT"] = compile_artifact
    if shared_world_artifact is not None:
        environment["WORTH_UI_SHARED_WORLD_ARTIFACT"] = shared_world_artifact
    completed = subprocess.run(
        command, cwd=ROOT, env=environment, capture_output=True, text=True, check=False
    )
    if completed.returncode != 0:
        sys.stderr.write(completed.stdout)
        sys.stderr.write(completed.stderr)
        raise RuntimeError(f"fresh governed execution failed for {row['requirement']}")
    payload = json.loads(completed.stdout.splitlines()[-1])
    if payload.get("exit_posture") != "passed" or payload.get("requirement") != row["requirement"]:
        raise RuntimeError(f"fresh governed execution was not exact for {row['requirement']}")


def bind_fresh_compile_artifact(command: list[str], compile_artifact: str) -> list[str]:
    rebound = list(command)
    for index, word in enumerate(rebound[:-1]):
        if word == "--source" and rebound[index + 1] == COMPILE_ARTIFACT:
            rebound[index + 1] = compile_artifact
    return rebound


def bind_fresh_shared_world(command: list[str], shared_world_artifact: str) -> list[str]:
    rebound = list(command)
    for index, word in enumerate(rebound[:-1]):
        if word == "--source" and rebound[index + 1] in {
            NATIVE_WORLD_ARTIFACT, MOUNTED_WORLD_ARTIFACT
        }:
            rebound[index + 1] = shared_world_artifact
    return rebound


def closure_tests() -> None:
    prefix = [
        "cargo", "test", "--manifest-path", "workspaces/worth-ui/Cargo.toml",
        "-p", "worth-ui-certification", "--test", "topology_contracts",
    ]
    run(prefix + [
        "milestone_3141_phase1_ledger::mutation_tests::milestone_ledger_has_exact_schema_inventory_and_honest_posture",
        "--", "--exact", "--nocapture",
    ])
    run(prefix + [
        "milestone_3141_phase1_ledger::phase_two_closure_requires_every_phase_one_and_two_row",
        "--", "--exact", "--ignored", "--nocapture",
    ])


def main() -> int:
    TARGET.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="worth-ui-3141-verify-", dir=TARGET) as directory:
        temporary = Path(directory)
        fresh_compile = (temporary / "compile-contracts.json").relative_to(ROOT).as_posix()
        run([
            sys.executable,
            "scripts/ci/run_worth_ui_compile_contracts.py",
            "--artifact",
            fresh_compile,
        ])
        governed = rows()
        mounted_world = next(row for row in governed if row["requirement"] == "P1-WORLDS-01")
        headless_cost = next(
            row for row in governed if row["requirement"] == "P1-HEADLESS-COST-01"
        )
        phase_one = [
            row for row in governed
            if row["phase"] == "1"
            and row["requirement"] not in {"P1-WORLDS-01", "P1-HEADLESS-COST-01"}
        ]
        world = next(row for row in governed if row["requirement"] == "P2-WORLD-01")
        dependent_phase_two = [
            row for row in governed
            if row["phase"] == "2" and row["requirement"] != "P2-WORLD-01"
        ]
        for index, row in enumerate(phase_one):
            rerun_row(row, temporary / f"p1-{index:02}.json", fresh_compile)
        fresh_mounted = (temporary / "mounted-world.json").relative_to(ROOT).as_posix()
        rerun_row(mounted_world, ROOT / fresh_mounted, fresh_compile)
        rerun_row(
            headless_cost,
            temporary / "p1-headless-cost.json",
            fresh_compile,
            fresh_mounted,
        )
        fresh_world = (temporary / "native-world.json").relative_to(ROOT).as_posix()
        rerun_row(world, ROOT / fresh_world, fresh_compile)
        for index, row in enumerate(dependent_phase_two):
            rerun_row(
                row,
                temporary / f"p2-{index:02}.json",
                fresh_compile,
                fresh_world,
            )
    closure_tests()
    print("Worth UI milestone 3.14.1 ledger operationally verified: 30 fresh rows")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
