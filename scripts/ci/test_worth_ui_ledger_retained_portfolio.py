from __future__ import annotations

import csv
import hashlib
import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch


sys.path.insert(0, str(Path(__file__).resolve().parent))

from worth_ui_ledger_retained_portfolio import (
    digest_json,
    portfolio_identity,
    publish,
    row_claim_digest,
    validate,
)
from worth_ui_ledger_causal_revalidation import source_digest_at
from worth_ui_ledger_portfolio_executions import (
    aggregate_executions,
    authenticated_row_payload,
)
from worth_ui_ledger_execution_cache import artifact_bindings_match
from worth_ui_ledger_runner_authentication import authentication_tag
from worth_ui_ledger_runner_authentication import RunnerProvenanceUnavailable


CANONICAL = Path("_docs/worth-ui/milestone-3.14.1-proof-ledger.csv")


class RetainedPortfolioTests(unittest.TestCase):
    def test_non_ledger_mutation_control_ignores_only_its_legacy_ledger_binding(self) -> None:
        ledger = "WORTH_UI_MILESTONE_3141_LEDGER"
        non_ledger = [
            "cargo",
            "test",
            "milestone_3141_phase1_ledger::result_artifact::mutation_tests::"
            "phase_two_boundary_observation_rejects_each_causal_mutation",
        ]
        actual_ledger_reader = [
            "cargo",
            "test",
            "milestone_3141_phase1_ledger::phase_five_closure_requires_every_"
            "predecessor_and_phase_five_row",
        ]
        retained = {ledger: {"sha256": "a" * 64}}
        current = {ledger: {"sha256": "b" * 64}}
        self.assertTrue(
            artifact_bindings_match(retained, current, non_ledger)
        )
        self.assertFalse(
            artifact_bindings_match(retained, current, actual_ledger_reader)
        )

    def test_historical_receipts_use_authenticated_causal_bindings(self) -> None:
        command = ["cargo", "test", "--list"]
        command_sha = hashlib.sha256(
            json.dumps(command, separators=(",", ":")).encode("utf-8")
        ).hexdigest()

        def payload(requirement: str) -> dict[str, object]:
            receipts = [
                {
                    "role": role,
                    "key": key,
                    "command_sha256": command_sha,
                    "duration_ms": 1,
                }
                for role, key in (
                    ("main-discovery", "1" * 64),
                    ("ignored-discovery", requirement[0] * 64),
                    ("main-test", requirement[-1] * 64),
                )
            ]
            return {
                "requirement": requirement,
                "execution_receipts": receipts,
                "list_command": command,
                "ignored_list_command": command,
                "test_command": command,
            }

        causal = payload("A2")
        shared = payload("B3")
        with (
            patch(
                "worth_ui_ledger_portfolio_executions.authenticated_row_payload",
                return_value=True,
            ),
            patch(
                "worth_ui_ledger_portfolio_executions.validate_causal_reuse",
                return_value=None,
            ),
            patch(
                "worth_ui_ledger_portfolio_executions.portfolio_receipt_keys",
                side_effect=(
                    {receipt["key"] for receipt in causal["execution_receipts"]},
                    {receipt["key"] for receipt in shared["execution_receipts"]},
                ),
            ),
            patch(
                "worth_ui_ledger_portfolio_executions.validate_execution_receipt",
                return_value="f" * 64,
            ) as validate_receipt,
        ):
            aggregate_executions(
                [causal, shared], Path("."), "a" * 40, "b" * 64, {"A2", "B3"}
            )

        shared_calls = [
            call for call in validate_receipt.call_args_list if call.args[3]["key"] == "1" * 64
        ]
        self.assertEqual(len(shared_calls), 2)
        self.assertTrue(all(call.args[4] for call in shared_calls))

    def test_predecessor_row_authenticates_its_source_artifact_digest(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            unsigned = {
                "requirement": "P1-AFFINITY-01",
                "artifact_sha256": "a" * 64,
            }
            payload = {
                **unsigned,
                "runner_authentication": authentication_tag(unsigned, root),
            }
            self.assertTrue(authenticated_row_payload(root, payload))
            payload["artifact_sha256"] = "b" * 64
            self.assertFalse(authenticated_row_payload(root, payload))

    def test_exact_portfolio_validates_without_reexecution(self) -> None:
        with self.fixture() as (root, ledger):
            published = publish(root, ledger, 4, "a" * 40, "b" * 64)
            retained = validate(root, ledger, 4, "a" * 40, "b" * 64)
            self.assertEqual(retained, published)
            self.assertEqual(len(retained["rows"]), 68)

    def test_artifact_or_portfolio_mutation_is_rejected(self) -> None:
        with self.fixture() as (root, ledger):
            publish(root, ledger, 4, "a" * 40, "b" * 64)
            artifact = root / "evidence/row-47.json"
            artifact.write_text("{}", encoding="utf-8")
            with self.assertRaisesRegex(RuntimeError, "artifact drifted"):
                validate(root, ledger, 4, "a" * 40, "b" * 64)
        with self.fixture() as (root, ledger):
            publish(root, ledger, 4, "a" * 40, "b" * 64)
            identity = root / portfolio_identity(4)
            payload = json.loads(identity.read_text(encoding="utf-8"))
            payload["unique_execution_count"] = 999
            identity.write_text(json.dumps(payload), encoding="utf-8")
            with self.assertRaisesRegex(RuntimeError, "differs"):
                validate(root, ledger, 4, "a" * 40, "b" * 64)

    def test_validation_survives_execution_cache_deletion(self) -> None:
        with self.fixture() as (root, ledger):
            published = publish(root, ledger, 4, "a" * 40, "b" * 64)
            cache = (
                root
                / "workspaces/worth-ui/target/milestone-3141-execution-cache"
            )
            self.assertTrue(any(cache.rglob("*.json")))
            for identity in cache.rglob("*"):
                if identity.is_file():
                    identity.unlink()
            retained = validate(root, ledger, 4, "a" * 40, "b" * 64)
            self.assertEqual(retained, published)

    def test_different_machine_key_requires_operational_revalidation(self) -> None:
        with self.fixture() as (root, ledger), tempfile.TemporaryDirectory() as state:
            publish(root, ledger, 4, "a" * 40, "b" * 64)
            with patch.dict("os.environ", {"LOCALAPPDATA": state}):
                with self.assertRaises(RunnerProvenanceUnavailable):
                    validate(root, ledger, 4, "a" * 40, "b" * 64)

    def test_forged_or_mismatched_durable_envelope_is_rejected(self) -> None:
        with self.fixture() as (root, ledger):
            publish(root, ledger, 4, "a" * 40, "b" * 64)
            envelope_path = next(
                (root / "_docs/worth-ui/milestone-3.14.1-evidence/executions").rglob(
                    "*.json"
                )
            )
            envelope = json.loads(envelope_path.read_text(encoding="utf-8"))
            envelope["record"]["stdout"] = "forged durable envelope"
            envelope["receipt_sha256"] = digest_json(envelope["record"])
            envelope_path.write_text(json.dumps(envelope), encoding="utf-8")
            with self.assertRaisesRegex(RuntimeError, "differs"):
                validate(root, ledger, 4, "a" * 40, "b" * 64)
        with self.fixture() as (root, ledger):
            publish(root, ledger, 4, "a" * 40, "b" * 64)
            first = next(
                (root / "_docs/worth-ui/milestone-3.14.1-evidence/executions").rglob(
                    "*.json"
                )
            )
            second = next(
                path
                for path in (root / "_docs/worth-ui/milestone-3.14.1-evidence/executions").rglob(
                    "*.json"
                )
                if path != first
            )
            first.write_text(second.read_text(encoding="utf-8"), encoding="utf-8")
            with self.assertRaisesRegex(
                RuntimeError, "absent|differs|wrong row command"
            ):
                validate(root, ledger, 4, "a" * 40, "b" * 64)

    def test_missing_or_forged_execution_receipts_are_rejected(self) -> None:
        with self.fixture() as (root, ledger):
            artifact = root / "evidence/row-47.json"
            payload = json.loads(artifact.read_text(encoding="utf-8"))
            payload["execution_receipts"] = []
            payload.pop("runner_authentication", None)
            payload["runner_authentication"] = authentication_tag(payload, root)
            artifact.write_text(json.dumps(payload), encoding="utf-8")
            rewrite_artifact_digest(ledger, payload["requirement"], artifact)
            with self.assertRaisesRegex(RuntimeError, "execution receipts"):
                publish(root, ledger, 4, "a" * 40, "b" * 64)
        with self.fixture() as (root, ledger):
            receipt = next(
                (root / "workspaces/worth-ui/target").rglob("*.json")
            )
            envelope = json.loads(receipt.read_text(encoding="utf-8"))
            envelope["record"]["stdout"] = "forged"
            envelope["receipt_sha256"] = digest_json(envelope["record"])
            receipt.write_text(json.dumps(envelope), encoding="utf-8")
            with self.assertRaisesRegex(RuntimeError, "differs"):
                publish(root, ledger, 4, "a" * 40, "b" * 64)

    def test_self_consistent_row_artifact_forgery_is_rejected(self) -> None:
        with self.fixture() as (root, ledger):
            artifact = root / "evidence/row-47.json"
            payload = json.loads(artifact.read_text(encoding="utf-8"))
            payload["test_stdout"] = "forged but content-addressed"
            artifact.write_text(json.dumps(payload), encoding="utf-8")
            rewrite_artifact_digest(ledger, payload["requirement"], artifact)
            with self.assertRaisesRegex(RuntimeError, "row drifted"):
                publish(root, ledger, 4, "a" * 40, "b" * 64)

    def test_authenticated_receipt_cannot_be_swapped_between_rows(self) -> None:
        with self.fixture() as (root, ledger):
            first = root / "evidence/row-47.json"
            second = root / "evidence/row-48.json"
            first_payload = json.loads(first.read_text(encoding="utf-8"))
            second_payload = json.loads(second.read_text(encoding="utf-8"))
            first_payload["execution_receipts"] = second_payload["execution_receipts"]
            first_payload.pop("runner_authentication", None)
            first_payload["runner_authentication"] = authentication_tag(first_payload, root)
            first.write_text(json.dumps(first_payload), encoding="utf-8")
            rewrite_artifact_digest(ledger, first_payload["requirement"], first)
            with self.assertRaisesRegex(RuntimeError, "wrong row command"):
                publish(root, ledger, 4, "a" * 40, "b" * 64)

    def fixture(self):
        directory = tempfile.TemporaryDirectory()
        root = Path(directory.name)
        with CANONICAL.open(encoding="utf-8", newline="") as stream:
            reader = csv.DictReader(stream)
            fields = list(reader.fieldnames or ())
            rows = [dict(row) for row in reader if int(row["phase"]) <= 4]
        handoff_rows = []
        (root / "source.rs").write_text("fixture source", encoding="utf-8")
        for index, row in enumerate(rows):
            row["result"] = "PROVED"
            row["final_source"] = "true"
            row["command_result"] = "passed"
            row["run_nonce"] = f"nonce-{index}"
            row["retained_result_artifact"] = f"evidence/row-{index:02}.json"
            row["source_identity"] = "source.rs"
            row["source_digest"] = source_digest_at(root, ("source.rs",))
            commands = {
                "main-discovery": ["cargo", "test", row["requirement"], "list"],
                "ignored-discovery": ["cargo", "test", row["requirement"], "ignored"],
                "main-test": ["cargo", "test", row["requirement"], "main"],
            }
            if row["requirement"] == "P4-FONT-COLLECTION-01":
                commands["public-example"] = [
                    "cargo", "check", "-p", "worth-ui", "--example", "text_platform"
                ]
            receipts = [
                execution_receipt(root, role, command, "a" * 40, "b" * 64)
                for role, command in commands.items()
            ]
            payload = {
                "requirement": row["requirement"],
                "exit_posture": "passed",
                "claim_digest": row_claim_digest(row),
                "run_nonce": row["run_nonce"],
                "production_entry": row["production_entry"],
                "independent_oracle": row["independent_oracle"],
                "executed_exact_command": row["exact_command"],
                "source_identity": ["source.rs"],
                "mapping_source_identity": ["source.rs"],
                "source_digest": row["source_digest"],
                "execution_receipts": receipts,
                "list_command": commands["main-discovery"],
                "ignored_list_command": commands["ignored-discovery"],
                "test_command": commands["main-test"],
            }
            if "public-example" in commands:
                payload["public_example_command"] = commands["public-example"]
            payload["runner_authentication"] = authentication_tag(payload, root)
            if int(row["phase"]) < 4:
                handoff_rows.append(payload)
            artifact = root / row["retained_result_artifact"]
            artifact.parent.mkdir(parents=True, exist_ok=True)
            artifact.write_text(json.dumps(payload), encoding="utf-8")
            row["result_artifact_digest"] = hashlib.sha256(artifact.read_bytes()).hexdigest()
        ledger = root / "ledger.csv"
        with ledger.open("w", encoding="utf-8", newline="") as stream:
            writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
            writer.writeheader()
            writer.writerows(rows)
        handoff = root / "_docs/worth-ui/milestone-3.14.1-evidence/p4-predecessor-handoff.json"
        handoff.parent.mkdir(parents=True, exist_ok=True)
        handoff.write_text(
            json.dumps({"through_phase": 3, "rows": handoff_rows}), encoding="utf-8"
        )
        return Fixture(directory, root, ledger)


def execution_receipt(
    root: Path, role: str, command: list[str], revision: str, state_digest: str
) -> dict[str, object]:
    binding = {
        "schema": "worth-ui-ledger-execution-receipt-v2",
        "command": command,
        "source_revision": revision,
        "source_state_digest": state_digest,
        "artifact_bindings": {},
    }
    key = digest_json(binding)
    record = {
        **binding,
        "key": key,
        "returncode": 0,
        "stdout": "passed",
        "stderr": "",
        "duration_ms": 1,
    }
    envelope = {
        "record": record,
        "receipt_sha256": digest_json(record),
        "runner_authentication": authentication_tag(record, root),
    }
    identity = (
        root
        / "workspaces/worth-ui/target/milestone-3141-execution-cache"
        / state_digest
        / "executions"
        / key[:2]
        / f"{key}.json"
    )
    identity.parent.mkdir(parents=True, exist_ok=True)
    identity.write_text(json.dumps(envelope), encoding="utf-8")
    return {
        "role": role,
        "key": key,
        "command_sha256": hashlib.sha256(
            json.dumps(command, separators=(",", ":")).encode("utf-8")
        ).hexdigest(),
        "duration_ms": 1,
        "reused": False,
    }


def rewrite_artifact_digest(ledger: Path, requirement: str, artifact: Path) -> None:
    with ledger.open(encoding="utf-8", newline="") as stream:
        reader = csv.DictReader(stream)
        fields = list(reader.fieldnames or ())
        rows = [dict(row) for row in reader]
    for row in rows:
        if row["requirement"] == requirement:
            row["result_artifact_digest"] = hashlib.sha256(artifact.read_bytes()).hexdigest()
    with ledger.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


class Fixture:
    def __init__(self, directory, root: Path, ledger: Path) -> None:
        self.directory = directory
        self.root = root
        self.ledger = ledger

    def __enter__(self):
        return self.root, self.ledger

    def __exit__(self, *_args):
        self.directory.cleanup()


if __name__ == "__main__":
    unittest.main()
