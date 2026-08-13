import tempfile
import subprocess
import sys
import time
import unittest
from pathlib import Path
from unittest.mock import patch

import close_worth_ui_3141_ledger as ledger_closer
import worth_ui_3141_supporting_world as supporting_world
import worth_ui_ledger_dependency as ledger_dependency
from worth_ui_ledger_runner_authentication import authentication_tag


class LedgerDependencyTests(unittest.TestCase):
    def test_ledger_settlement_lock_is_cross_process_exclusive(self) -> None:
        script = (
            "import pathlib,sys; from close_worth_ui_3141_ledger import ledger_lock; "
            "guard=ledger_lock(pathlib.Path(sys.argv[1])); guard.__enter__(); "
            "print('acquired',flush=True); sys.stdin.readline(); guard.__exit__(None,None,None)"
        )
        with tempfile.TemporaryDirectory() as directory:
            identity = str(Path(directory) / "ledger.lock")
            first = subprocess.Popen(
                [sys.executable, "-c", script, identity], cwd=Path(__file__).parent,
                stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True,
            )
            self.assertEqual(first.stdout.readline().strip(), "acquired")
            second = subprocess.Popen(
                [sys.executable, "-c", script, identity], cwd=Path(__file__).parent,
                stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True,
            )
            time.sleep(0.2)
            self.assertIsNone(second.poll())
            first.stdin.write("\n")
            first.stdin.flush()
            self.assertEqual(second.stdout.readline().strip(), "acquired")
            second.stdin.write("\n")
            second.stdin.flush()
            self.assertEqual(first.wait(timeout=5), 0)
            self.assertEqual(second.wait(timeout=5), 0)
            for stream in (first.stdin, first.stdout, second.stdin, second.stdout):
                stream.close()

    def test_shared_evidence_requires_a_final_producer_and_exact_artifact(self) -> None:
        import csv
        import hashlib
        import json
        import os

        source = Path("_docs/worth-ui/milestone-3.14.1-proof-ledger.csv")
        with source.open(encoding="utf-8", newline="") as stream:
            row = next(
                row for row in csv.DictReader(stream)
                if row["requirement"] == "P3-HEADLESS-COST-01"
            )
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            identity = "evidence/producer.json"
            destination = root / identity
            destination.parent.mkdir(parents=True)
            row["retained_result_artifact"] = identity
            row["result"] = "PROVED"
            row["final_source"] = "true"
            claim = hashlib.sha256()
            for field in ledger_dependency.CLAIM_FIELDS:
                claim.update(field.encode())
                claim.update(b"\0")
                claim.update(row[field].encode())
                claim.update(b"\0")
            artifact = authenticated(root, {
                "claim_digest": claim.hexdigest(),
                "source_revision": row["source_revision"],
                "source_digest": row["source_digest"],
                "source_state_digest": row["source_state_digest"],
                "run_nonce": row["run_nonce"],
                "source_identity": row["source_identity"].split(";"),
            })
            destination.write_text(json.dumps(artifact), encoding="utf-8")
            row["result_artifact_digest"] = hashlib.sha256(destination.read_bytes()).hexdigest()
            ledger = root / "candidate.csv"
            with source.open(encoding="utf-8", newline="") as stream:
                fields = next(csv.reader(stream))
            self._write_ledger(ledger, fields, row)
            with patch.dict(os.environ, {"WORTH_UI_MILESTONE_3141_LEDGER": str(ledger)}):
                ledger_dependency.require_proved_artifact(
                    root, row["requirement"], identity, artifact
                )
                rebound = dict(artifact)
                canonical = artifact["source_identity"]
                rebound_source = (
                    root / "workspaces/worth-ui/target/"
                    "worth-ui-3141-verify-test/p3-delta-world.json"
                )
                rebound_source.parent.mkdir(parents=True)
                rebound_source.write_text("fresh governed world", encoding="utf-8")
                executed = list(canonical)
                slot = executed.index(
                    "_docs/worth-ui/milestone-3.14.1-evidence/"
                    "p3-delta-source-01.json"
                )
                executed[slot] = rebound_source.relative_to(root).as_posix()
                rebound["source_identity"] = executed
                rebound["mapping_source_identity"] = artifact["source_identity"]
                rebound["source_rebindings"] = [{
                    "canonical": canonical[slot],
                    "executed": executed[slot],
                    "sha256": hashlib.sha256(rebound_source.read_bytes()).hexdigest(),
                }]
                row["source_identity"] = ";".join(executed)
                rebound_claim = hashlib.sha256()
                for field in ledger_dependency.CLAIM_FIELDS:
                    rebound_claim.update(field.encode())
                    rebound_claim.update(b"\0")
                    rebound_claim.update(row[field].encode())
                    rebound_claim.update(b"\0")
                rebound["claim_digest"] = rebound_claim.hexdigest()
                rebound = authenticated(root, rebound)
                destination.write_text(json.dumps(rebound), encoding="utf-8")
                row["result_artifact_digest"] = hashlib.sha256(
                    destination.read_bytes()
                ).hexdigest()
                self._write_ledger(ledger, fields, row)
                ledger_dependency.require_proved_artifact(
                    root, row["requirement"], identity, rebound
                )
                bad_rebinding = dict(rebound)
                bad_rebinding["source_rebindings"] = [
                    {**rebound["source_rebindings"][0], "sha256": "0" * 64}
                ]
                bad_rebinding = authenticated(root, bad_rebinding)
                with self.assertRaisesRegex(ValueError, "drifted source rebindings"):
                    ledger_dependency.require_proved_artifact(
                        root, row["requirement"], identity, bad_rebinding
                    )
                bad_mapping = dict(rebound)
                bad_mapping["mapping_source_identity"] = ["substitute"]
                bad_mapping = authenticated(root, bad_mapping)
                with self.assertRaisesRegex(ValueError, "invalid executed sources"):
                    ledger_dependency.require_proved_artifact(
                        root, row["requirement"], identity, bad_mapping
                    )
                row["source_identity"] = ";".join(canonical)
                destination.write_text(json.dumps(artifact), encoding="utf-8")
                row["result_artifact_digest"] = hashlib.sha256(
                    destination.read_bytes()
                ).hexdigest()
                self._write_ledger(ledger, fields, row)
                for field in [
                    "claim_digest", "source_revision", "source_digest",
                    "source_state_digest", "run_nonce",
                ]:
                    mutant = dict(artifact)
                    mutant[field] = "substitute"
                    mutant = authenticated(root, mutant)
                    with self.assertRaisesRegex(ValueError, "drifted"):
                        ledger_dependency.require_proved_artifact(
                            root, row["requirement"], identity, mutant
                        )
                mutant = dict(artifact)
                mutant["source_identity"] = ["substitute"] * len(canonical)
                mutant["mapping_source_identity"] = canonical
                mutant["source_rebindings"] = []
                mutant = authenticated(root, mutant)
                with self.assertRaisesRegex(ValueError, "drifted sources"):
                    ledger_dependency.require_proved_artifact(
                        root, row["requirement"], identity, mutant
                    )
                row["result_artifact_digest"] = "substitute"
                self._write_ledger(ledger, fields, row)
                with self.assertRaisesRegex(ValueError, "artifact digest drifted"):
                    ledger_dependency.require_proved_artifact(
                        root, row["requirement"], identity, artifact
                    )
                row["result_artifact_digest"] = hashlib.sha256(
                    destination.read_bytes()
                ).hexdigest()
                row["result"] = "OPEN"
                self._write_ledger(ledger, fields, row)
                with self.assertRaisesRegex(ValueError, "not final-source proved"):
                    ledger_dependency.require_proved_artifact(
                        root, row["requirement"], identity, artifact
                    )

    def test_atomic_closer_exposes_each_completed_row_only_to_its_successors(self) -> None:
        import csv

        fields = [
            "requirement", "exact_command", "matched_test_count", "source_revision",
            "source_digest", "source_state_digest", "run_nonce", "command_result",
            "result_artifact_digest", "result", "final_source",
        ]
        rows = [
            {field: "" for field in fields} | {
                "requirement": requirement,
                "exact_command": requirement,
                "result": "OPEN",
                "final_source": "false",
            }
            for requirement in ["P3-FIRST", "P3-SECOND"]
        ]
        result = {
            "matched_test_count": 1,
            "source_revision": "revision",
            "source_digest": "sources",
            "source_state_digest": "state",
            "run_nonce": "nonce",
            "artifact_sha256": "artifact",
        }
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            ledger = root / "ledger.csv"
            with ledger.open("w", encoding="utf-8", newline="") as stream:
                writer = csv.DictWriter(stream, fieldnames=fields)
                writer.writeheader()
                writer.writerows(rows)
            calls = 0

            def execute(command: str, candidate: Path):
                nonlocal calls
                with candidate.open(encoding="utf-8", newline="") as stream:
                    observed = list(csv.DictReader(stream))
                expected = ["OPEN", "OPEN"] if calls == 0 else ["PROVED", "OPEN"]
                self.assertEqual([row["result"] for row in observed], expected)
                calls += 1
                return {**result, "run_nonce": f"nonce-{calls}"}

            with (
                patch.object(ledger_closer, "ROOT", root),
                patch.object(ledger_closer, "LEDGER", ledger),
                patch.object(ledger_closer, "run", side_effect=execute),
                patch.object(ledger_closer, "claim_digest", return_value="c" * 64),
                patch.object(ledger_closer, "source_revision", return_value="a" * 40),
                patch.object(ledger_closer, "source_state_digest", return_value="b" * 64),
                patch.object(
                    ledger_closer,
                    "execute_or_restore",
                    side_effect=lambda row, candidate, _cache, _claim, runner: runner(
                        row["exact_command"], candidate
                    ),
                ),
                patch.object(ledger_closer, "publish"),
                patch.object(ledger_closer, "validate_ledger_posture"),
                patch.object(ledger_closer, "verify_closed_prefix") as verify,
            ):
                ledger_closer.close_selected_atomically(rows, fields, rows, verify_phase=3)
            self.assertEqual(calls, 2)
            verify.assert_called_once()
            self.assertEqual(verify.call_args.args[0], 3)
            self.assertNotEqual(verify.call_args.args[1], ledger)
            with ledger.open(encoding="utf-8", newline="") as stream:
                self.assertEqual(
                    [row["result"] for row in csv.DictReader(stream)],
                    ["PROVED", "PROVED"],
                )

    def test_atomic_closer_does_not_publish_a_failed_candidate(self) -> None:
        import csv

        fields = [
            "requirement", "exact_command", "matched_test_count", "source_revision",
            "source_digest", "source_state_digest", "run_nonce", "command_result",
            "result_artifact_digest", "result", "final_source",
        ]
        row = {field: "" for field in fields} | {
            "requirement": "P3-ONLY",
            "exact_command": (
                "runner --artifact "
                "_docs/worth-ui/milestone-3.14.1-evidence/p3-only.json"
            ),
            "result": "OPEN",
            "final_source": "false",
        }
        result = {
            "matched_test_count": 1,
            "source_revision": "revision",
            "source_digest": "sources",
            "source_state_digest": "state",
            "run_nonce": "nonce",
            "artifact_sha256": "artifact",
        }
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            ledger = root / "ledger.csv"
            artifact = root / "_docs/worth-ui/milestone-3.14.1-evidence/p3-only.json"
            artifact.parent.mkdir(parents=True)
            artifact.write_bytes(b"original evidence\n")
            with ledger.open("w", encoding="utf-8", newline="") as stream:
                writer = csv.DictWriter(stream, fieldnames=fields)
                writer.writeheader()
                writer.writerow(row)
            original = ledger.read_bytes()

            def replace_artifact(_command: str, _candidate: Path):
                artifact.write_bytes(b"candidate evidence\n")
                return result

            with (
                patch.object(ledger_closer, "ROOT", root),
                patch.object(ledger_closer, "LEDGER", ledger),
                patch.object(ledger_closer, "run", side_effect=replace_artifact),
                patch.object(ledger_closer, "claim_digest", return_value="c" * 64),
                patch.object(ledger_closer, "source_revision", return_value="a" * 40),
                patch.object(ledger_closer, "source_state_digest", return_value="b" * 64),
                patch.object(
                    ledger_closer,
                    "execute_or_restore",
                    side_effect=lambda row, candidate, _cache, _claim, runner: runner(
                        row["exact_command"], candidate
                    ),
                ),
                patch.object(ledger_closer, "publish"),
                patch.object(ledger_closer, "validate_ledger_posture"),
                patch.object(
                    ledger_closer,
                    "verify_closed_prefix",
                    side_effect=RuntimeError("fresh verification failed"),
                ),
            ):
                with self.assertRaisesRegex(RuntimeError, "fresh verification failed"):
                    ledger_closer.close_selected_atomically(
                        [row], fields, [row], verify_phase=3
                    )
            self.assertEqual(ledger.read_bytes(), original)
            self.assertEqual(artifact.read_bytes(), b"original evidence\n")

    def test_hp02_support_entrypoint_rejects_an_open_mixed_world_producer(self) -> None:
        import csv
        import json
        import os
        from types import SimpleNamespace

        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            artifact_path = root / supporting_world.MIXED_ARTIFACT
            artifact_path.parent.mkdir(parents=True)
            artifact = {
                "schema_version": 5,
                "requirement": supporting_world.MIXED_REQUIREMENT,
                "package": "worth-ui-certification",
                "target_kind": "test",
                "target_name": "application_contracts",
                "test_name": supporting_world.MIXED_TEST,
                "matched_test_count": 1,
                "declared_ignored_test_count": 1,
                "expected_declared_ignored": True,
                "executed_test_count": 1,
                "passed_test_count": 1,
                "ignored_test_count": 0,
                "exit_posture": "passed",
                "test_exit_code": 0,
                "source_revision": "revision",
                "source_state_digest": "state",
                "structural_counter": "source-rows=1",
                "construction_cost": (
                    "main-tests=1;hostile-controls=1;product-processes=0;"
                    "compile-sessions=0;courtroom-worlds=1"
                ),
                "execution_cost": "executed-tests=2;presentations=5",
                "test_stdout": "WORTH_UI_LEDGER_WORLD=1\n",
            }
            artifact_path.write_text(json.dumps(artifact), encoding="utf-8")
            ledger = root / "candidate.csv"
            with ledger.open("w", encoding="utf-8", newline="") as stream:
                writer = csv.DictWriter(
                    stream,
                    fieldnames=["requirement", "result", "final_source"],
                )
                writer.writeheader()
                writer.writerow({
                    "requirement": supporting_world.MIXED_REQUIREMENT,
                    "result": "OPEN",
                    "final_source": "false",
                })
            test = SimpleNamespace(
                requirement="P3-HP02-WORLD-01",
                sources=(supporting_world.MIXED_ARTIFACT,),
            )
            with patch.dict(os.environ, {"WORTH_UI_MILESTONE_3141_LEDGER": str(ledger)}):
                with self.assertRaisesRegex(ValueError, "not final-source proved"):
                    supporting_world.validate_phase3_hp02_support(
                        test, "revision", "state", root
                    )

    @staticmethod
    def _write_ledger(path: Path, fields: list[str], row: dict[str, str]) -> None:
        import csv

        with path.open("w", encoding="utf-8", newline="") as stream:
            writer = csv.DictWriter(stream, fieldnames=fields)
            writer.writeheader()
            writer.writerow(row)


def authenticated(root: Path, payload: dict[str, object]) -> dict[str, object]:
    result = {
        key: value for key, value in payload.items() if key != "runner_authentication"
    }
    result["runner_authentication"] = authentication_tag(result, root)
    return result
