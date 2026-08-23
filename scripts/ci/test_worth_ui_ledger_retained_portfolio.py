from __future__ import annotations

import csv
import hashlib
import json
import sys
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import call, patch


sys.path.insert(0, str(Path(__file__).resolve().parent))

from worth_ui_ledger_command import claim_digest_for_row
from worth_ui_ledger_retained_portfolio import (
    digest_json,
    execution_input_rows,
    portfolio_identity,
    publish as publish_candidate,
    validate,
)
from worth_ui_ledger_artifact_transaction import ArtifactTransaction
from worth_ui_ledger_causal_revalidation import source_digest_at
from worth_ui_ledger_candidate_basis import from_path
from worth_ui_ledger_portfolio_executions import (
    aggregate_executions,
    authenticated_row_payload,
)
from worth_ui_ledger_execution_identity import AuthenticatedExecution
from worth_ui_ledger_execution_binding import artifact_bindings_match
from worth_ui_ledger_runner_authentication import authentication_tag
from worth_ui_ledger_runner_authentication import RunnerProvenanceUnavailable
import verify_worth_ui_3141_ledger as verifier


CANONICAL = Path("_docs/worth-ui/milestone-3.14.1-proof-ledger.csv")


def publish(
    root: Path,
    ledger: Path,
    phase: int,
    revision: str,
    state_digest: str,
) -> dict[str, object]:
    transaction = ArtifactTransaction(root, ledger, [])
    try:
        result = publish_candidate(root, ledger, phase, revision, state_digest)
        transaction.prepare_commit(ledger.read_bytes())
        transaction.commit()
        return result
    except BaseException:
        transaction.rollback()
        raise


class RetainedPortfolioTests(unittest.TestCase):
    def test_phase_two_portfolio_owns_the_complete_directly_executed_prefix(self) -> None:
        rows = [
            {"phase": "1", "requirement": "P1-ONE-01"},
            {"phase": "2", "requirement": "P2-ONE-01"},
        ]
        self.assertEqual(execution_input_rows(rows, 2), rows)
        self.assertEqual(execution_input_rows(rows, 3), [])

    def test_exact_portfolio_validates_without_reexecution(self) -> None:
        with self.fixture() as (root, ledger):
            published = publish(root, ledger, 4, "a" * 40, "b" * 64)
            retained = validate(root, ledger, 4, "a" * 40, "b" * 64)
            self.assertEqual(retained, published)
            self.assertEqual(len(retained["rows"]), 68)

    def test_stale_predecessor_source_binding_is_rejected(self) -> None:
        with self.fixture() as (root, ledger):
            handoff = root / "_docs/worth-ui/milestone-3.14.1-evidence/p4-predecessor-handoff.json"
            payload = json.loads(handoff.read_text(encoding="utf-8"))
            payload["source_state_digest"] = "c" * 64
            handoff.write_text(json.dumps(payload), encoding="utf-8")
            with self.assertRaisesRegex(RuntimeError, "predecessor handoff is stale"):
                publish(root, ledger, 4, "a" * 40, "b" * 64)

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

    def test_ordinary_verifier_cannot_materialize_a_missing_migration(self) -> None:
        with self.fixture() as (root, ledger):
            publish(root, ledger, 4, "a" * 40, "b" * 64)
            migration = next(
                (root / "_docs/worth-ui/milestone-3.14.1-evidence/"
                 "execution-observation-migrations").rglob("*.json")
            )
            migration.unlink()
            evidence = root / "_docs/worth-ui/milestone-3.14.1-evidence"
            before = snapshot_tree(evidence)
            arguments = SimpleNamespace(
                through_phase=4, refresh_predecessor_for_phase=None
            )
            with (
                patch.object(verifier, "ROOT", root),
                patch.object(verifier, "LEDGER", ledger),
                patch.object(verifier, "parse_args", return_value=arguments),
                patch.object(verifier, "source_revision", return_value="a" * 40),
                patch.object(
                    verifier, "source_state_digest", return_value="b" * 64
                ),
                patch.object(verifier, "validate_current_causal_sources"),
                patch.dict(
                    "os.environ",
                    {"WORTH_UI_MILESTONE_3141_LEDGER": str(ledger)},
                ),
            ):
                with self.assertRaisesRegex(RuntimeError, "migration publication"):
                    verifier.main()
            self.assertEqual(snapshot_tree(evidence), before)

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
                (root / "_docs/worth-ui/milestone-3.14.1-evidence/"
                 "execution-observations").rglob("*.json")
            )
            envelope = json.loads(envelope_path.read_text(encoding="utf-8"))
            envelope["record"]["stdout"] = "forged durable envelope"
            envelope["receipt_sha256"] = digest_json(envelope["record"])
            envelope_path.write_text(json.dumps(envelope), encoding="utf-8")
            with self.assertRaisesRegex(
                RuntimeError, "collision|differs|migration publication|provenance"
            ):
                validate(root, ledger, 4, "a" * 40, "b" * 64)
        with self.fixture() as (root, ledger):
            publish(root, ledger, 4, "a" * 40, "b" * 64)
            first = next(
                (root / "_docs/worth-ui/milestone-3.14.1-evidence/"
                 "execution-observations").rglob("*.json")
            )
            second = next(
                path
                for path in (root / "_docs/worth-ui/milestone-3.14.1-evidence/"
                              "execution-observations").rglob("*.json")
                if path != first
            )
            first.write_text(second.read_text(encoding="utf-8"), encoding="utf-8")
            with self.assertRaisesRegex(
                RuntimeError, "absent|differs|wrong row command|provenance"
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
            with self.assertRaisesRegex(RuntimeError, "execution role"):
                publish(root, ledger, 4, "a" * 40, "b" * 64)
        with self.fixture() as (root, ledger):
            receipt = next(
                (root / "workspaces/worth-ui/target").rglob("*.json")
            )
            envelope = json.loads(receipt.read_text(encoding="utf-8"))
            envelope["record"]["stdout"] = "forged"
            envelope["receipt_sha256"] = digest_json(envelope["record"])
            receipt.write_text(json.dumps(envelope), encoding="utf-8")
            with self.assertRaisesRegex(RuntimeError, "unauthenticated|differs"):
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
                "claim_digest": claim_digest_for_row(row),
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
            json.dumps(
                {
                    "schema": "worth-ui-phase-predecessor-handoff-v4",
                    "through_phase": 3,
                    "source_revision": "a" * 40,
                    "source_state_digest": "b" * 64,
                    "rows": handoff_rows,
                    "verification_basis": from_path(ledger, 3).payload(),
                }
            ),
            encoding="utf-8",
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


def snapshot_tree(root: Path) -> dict[str, bytes]:
    return {
        path.relative_to(root).as_posix(): path.read_bytes()
        for path in root.rglob("*")
        if path.is_file()
    }


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
