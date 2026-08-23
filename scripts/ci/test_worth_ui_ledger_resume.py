from __future__ import annotations

import csv
from contextlib import ExitStack, redirect_stdout
import hashlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parent))

import worth_ui_ledger_atomic_closure as closer
from worth_ui_ledger_row_cache import RowEvidenceCache
from worth_ui_ledger_runner_authentication import authentication_tag


class AtomicClosureResumeTests(unittest.TestCase):
    def test_late_failure_reuses_completed_rows_without_rewinding_inputs(self) -> None:
        fields = [
            "requirement", "exact_command", "matched_test_count", "source_revision",
            "source_digest", "source_state_digest", "run_nonce", "command_result",
            "result_artifact_digest", "result", "final_source", "source_identity",
            "production_entry", "independent_oracle",
        ]
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            rows = [row(fields, index) for index in range(2)]
            ledger = root / "ledger.csv"
            dependency = root / "inputs/dependency.json"
            dependency.parent.mkdir(parents=True)
            dependency.write_text("current dependency", encoding="utf-8")
            write_ledger(ledger, fields, rows)
            cache = RowEvidenceCache(
                root,
                root / "cache",
                ledger.read_bytes(),
                "a" * 40,
                "b" * 64,
            )
            executions: list[str] = []

            def execute(command: str, _candidate: Path):
                requirement = command.split()[2]
                identity = command.split()[-1]
                payload = {
                    "requirement": requirement,
                    "exit_posture": "passed",
                    "claim_digest": "c" * 64,
                    "source_revision": "a" * 40,
                    "source_digest": "sources",
                    "source_state_digest": "b" * 64,
                    "run_nonce": f"nonce-{requirement}",
                    "matched_test_count": 1,
                    "source_identity": ["inputs/dependency.json"],
                }
                payload["runner_authentication"] = authentication_tag(payload, root)
                artifact = root / identity
                artifact.parent.mkdir(parents=True, exist_ok=True)
                artifact.write_text(json.dumps(payload), encoding="utf-8")
                executions.append(requirement)
                return {
                    **payload,
                    "artifact_sha256": hashlib.sha256(artifact.read_bytes()).hexdigest(),
                }

            common = (
                patch.object(closer, "ROOT", root),
                patch.object(closer, "LEDGER", ledger),
                patch.object(
                    closer,
                    "run_row",
                    side_effect=lambda _root, command, candidate: execute(command, candidate),
                ),
                patch.object(
                    closer, "claim_digest_for_row", return_value="c" * 64
                ),
                patch.object(closer, "source_revision", return_value="a" * 40),
                patch.object(closer, "source_state_digest", return_value="b" * 64),
                patch.object(closer, "RowEvidenceCache", return_value=cache),
                patch.object(closer, "publish"),
                patch.object(closer, "synchronize_historical_rows"),
                patch.object(closer, "validate_ledger_posture"),
            )
            telemetry = io.StringIO()
            with ExitStack() as stack:
                for context in common:
                    stack.enter_context(context)
                stack.enter_context(patch.object(
                    closer, "verify_closed_prefix", side_effect=RuntimeError("late failure")
                ))
                with redirect_stdout(telemetry):
                    with self.assertRaisesRegex(RuntimeError, "late failure"):
                        closer.close_atomically(
                            rows,
                            fields,
                            closer.AtomicClosurePlan(tuple(rows), 3),
                            root,
                            ledger,
                        )
            self.assertEqual(executions, ["P4-RESUME-A-01", "P4-RESUME-B-01"])
            self.assertTrue(all(row["result"] == "OPEN" for row in read_rows(ledger)))
            self.assertEqual(dependency.read_text(encoding="utf-8"), "current dependency")

            rows = read_rows(ledger)
            common = (
                patch.object(closer, "ROOT", root),
                patch.object(closer, "LEDGER", ledger),
                patch.object(
                    closer,
                    "run_row",
                    side_effect=lambda _root, command, candidate: execute(command, candidate),
                ),
                patch.object(
                    closer, "claim_digest_for_row", return_value="c" * 64
                ),
                patch.object(closer, "source_revision", return_value="a" * 40),
                patch.object(closer, "source_state_digest", return_value="b" * 64),
                patch.object(closer, "RowEvidenceCache", return_value=cache),
                patch.object(closer, "publish"),
                patch.object(closer, "synchronize_historical_rows"),
                patch.object(closer, "validate_ledger_posture"),
            )
            with ExitStack() as stack:
                for context in common:
                    stack.enter_context(context)
                stack.enter_context(patch.object(closer, "verify_closed_prefix"))
                with redirect_stdout(telemetry):
                    closer.close_atomically(
                        rows,
                        fields,
                        closer.AtomicClosurePlan(tuple(rows), 3),
                        root,
                        ledger,
                    )
            self.assertEqual(
                executions,
                ["P4-RESUME-A-01", "P4-RESUME-B-01"],
                "retry must restore source-bound row evidence instead of reexecuting it",
            )
            self.assertTrue(all(row["result"] == "PROVED" for row in read_rows(ledger)))
            self.assertEqual(dependency.read_text(encoding="utf-8"), "current dependency")
            output = telemetry.getvalue()
            for requirement in ("P4-RESUME-A-01", "P4-RESUME-B-01"):
                self.assertIn(f"[row:start] {requirement} disposition=execute", output)
                self.assertIn(
                    f"[row:complete] {requirement} disposition=execute posture=passed duration_ms=",
                    output,
                )
                self.assertIn(
                    f"[row:complete] {requirement} disposition=reuse posture=passed duration_ms=",
                    output,
                )


def row(fields: list[str], index: int) -> dict[str, str]:
    requirement = ("P4-RESUME-A-01", "P4-RESUME-B-01")[index]
    identity = (
        f"_docs/worth-ui/milestone-3.14.1-evidence/{requirement.lower()}.json"
    )
    return {field: "" for field in fields} | {
        "requirement": requirement,
        "exact_command": (
            f"runner --requirement {requirement} --source inputs/dependency.json "
            f"--artifact {identity}"
        ),
        "source_identity": "inputs/dependency.json",
        "production_entry": "inputs/dependency.json::production",
        "independent_oracle": "inputs/dependency.json::oracle",
        "result": "OPEN",
        "final_source": "false",
    }


def write_ledger(identity: Path, fields: list[str], rows: list[dict[str, str]]) -> None:
    with identity.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def read_rows(identity: Path) -> list[dict[str, str]]:
    with identity.open(encoding="utf-8", newline="") as stream:
        return [dict(row) for row in csv.DictReader(stream)]


if __name__ == "__main__":
    unittest.main()
