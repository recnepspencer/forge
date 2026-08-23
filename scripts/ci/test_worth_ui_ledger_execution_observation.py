from __future__ import annotations

import json
import os
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parent))

import worth_ui_ledger_execution_observation_migration as migration_publication

from worth_ui_ledger_execution_binding import (
    GovernedExecutionSnapshot,
    digest_json,
    execution_binding,
)
from worth_ui_ledger_execution_observation_migration import (
    LEGACY_ROOT,
    migrate_reference,
    migration_identity,
)
from worth_ui_ledger_execution_observation import create_observation
from worth_ui_ledger_execution_observation_store import (
    CACHE_ENV,
    durable_identity,
    retain,
    retain_envelope,
    stage,
)
from worth_ui_ledger_execution_reference_validation import (
    ExecutionExpectation,
    validate_execution,
)
from worth_ui_ledger_runner_authentication import authentication_tag
from worth_ui_ledger_artifact_transaction import ArtifactTransaction


class ExecutionObservationTests(unittest.TestCase):
    def test_same_binding_preserves_two_physical_durations(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            binding = execution_binding(
                ["cargo", "test", "exact"],
                root,
                GovernedExecutionSnapshot("a" * 40, "b" * 64),
            )
            with patch.dict(os.environ, {CACHE_ENV: str(root / "cache")}):
                first, first_reference = create_observation(
                    root, binding, 0, "passed", "", 628
                )
                second, second_reference = create_observation(
                    root, binding, 0, "passed", "", 641
                )
                stage(first)
                stage(second)
                retain(root, first_reference.observation_sha256)
                retain(root, second_reference.observation_sha256)
            self.assertEqual(
                first_reference.execution_binding_key,
                second_reference.execution_binding_key,
            )
            self.assertNotEqual(
                first_reference.observation_sha256,
                second_reference.observation_sha256,
            )
            self.assertTrue(
                durable_identity(root, first_reference.observation_sha256).is_file()
            )
            self.assertTrue(
                durable_identity(root, second_reference.observation_sha256).is_file()
            )

    def test_reference_mutation_cannot_relabel_an_observation(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            binding = execution_binding(
                ["cargo", "test", "exact"],
                root,
                GovernedExecutionSnapshot("a" * 40, "b" * 64),
            )
            with patch.dict(os.environ, {CACHE_ENV: str(root / "cache")}):
                envelope, reference = create_observation(
                    root, binding, 0, "passed", "", 628
                )
                stage(envelope)
                retain(root, reference.observation_sha256)
            payload = {"role": "main-test", **reference.payload()}
            expectation = ExecutionExpectation(
                root, "a" * 40, "b" * 64, "main-test", "P1-FIXTURE-01"
            )
            validate_execution(payload, expectation)
            payload["duration_ms"] = 641
            with self.assertRaisesRegex(RuntimeError, "differs from its observation"):
                validate_execution(payload, expectation)

    def test_canonical_lookup_rejects_path_and_cache_substitution(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            cache = root / "cache"
            binding = execution_binding(
                ["cargo", "test", "exact"],
                root,
                GovernedExecutionSnapshot("a" * 40, "b" * 64),
            )
            with patch.dict(os.environ, {CACHE_ENV: str(cache)}):
                first, first_reference = create_observation(
                    root, binding, 0, "first-output", "", 628
                )
                second, _ = create_observation(
                    root, binding, 0, "different-output", "", 628
                )
                stage(first)
                stage(second)
                retain(root, first_reference.observation_sha256)
            reference = {"role": "main-test", **first_reference.payload()}
            expectation = ExecutionExpectation(
                root, "a" * 40, "b" * 64, "main-test", "P1-FIXTURE-01"
            )
            identity = durable_identity(root, first_reference.observation_sha256)
            lawful = identity.read_bytes()
            substituted = next(
                path for path in (cache / "execution-observations").rglob("*.json")
                if first_reference.observation_sha256 not in path.name
            )
            identity.write_bytes(substituted.read_bytes())
            with self.assertRaisesRegex(RuntimeError, "absent or unauthenticated"):
                validate_execution(reference, expectation)
            identity.write_bytes(lawful)
            validate_execution(reference, expectation)
            identity.unlink()
            with self.assertRaisesRegex(RuntimeError, "absent or unauthenticated"):
                validate_execution(reference, expectation)

    def test_read_only_legacy_admission_cannot_publish_migration_artifacts(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            binding = {
                "schema": "worth-ui-ledger-execution-receipt-v2",
                "command": ["cargo", "test", "exact"],
                "source_revision": "a" * 40,
                "source_state_digest": "b" * 64,
                "artifact_bindings": {},
            }
            key = digest_json(binding)
            record = {
                **binding,
                "key": key,
                "returncode": 0,
                "stdout": "passed",
                "stderr": "",
                "duration_ms": 628,
            }
            envelope = {
                "receipt_sha256": digest_json(record),
                "record": record,
                "runner_authentication": authentication_tag(record, root),
            }
            legacy = root / LEGACY_ROOT / key[:2] / f"{key}.json"
            legacy.parent.mkdir(parents=True)
            legacy.write_text(json.dumps(envelope), encoding="utf-8")
            reference = {
                "role": "main-test",
                "key": key,
                "command_sha256": digest_json(binding["command"]),
                "duration_ms": 628,
                "reused": False,
            }

            with self.assertRaisesRegex(RuntimeError, "migration publication"):
                migrate_reference(root, reference)
            self.assertFalse(migration_identity(root, key).exists())

            with self.assertRaisesRegex(RuntimeError, "active artifact transaction"):
                migrate_reference(root, reference, materialize=True)
            self.assertFalse(migration_identity(root, key).exists())
            ledger = root / "ledger.csv"
            ledger.write_text("phase,requirement\n", encoding="utf-8")

            _expected_envelope, expected_reference = create_observation(
                root,
                {
                    "schema": "worth-ui-ledger-execution-binding-v3",
                    "command": binding["command"],
                    "source_revision": binding["source_revision"],
                    "source_state_digest": binding["source_state_digest"],
                    "artifact_bindings": binding["artifact_bindings"],
                },
                0,
                "passed",
                "",
                628,
            )
            observation_path = durable_identity(
                root, expected_reference.observation_sha256
            )
            transaction = ArtifactTransaction(root, ledger, [])
            with patch.object(
                migration_publication,
                "persist_migration",
                side_effect=RuntimeError("injected after observation write"),
            ):
                with self.assertRaisesRegex(RuntimeError, "after observation"):
                    migrate_reference(root, reference, materialize=True)
            transaction.rollback()
            self.assertFalse(observation_path.exists())
            self.assertFalse(migration_identity(root, key).exists())

            transaction = ArtifactTransaction(root, ledger, [])
            with patch.object(
                migration_publication,
                "reference_from_payload",
                side_effect=RuntimeError("injected after migration write"),
            ):
                with self.assertRaisesRegex(RuntimeError, "after migration"):
                    migrate_reference(root, reference, materialize=True)
            transaction.rollback()
            self.assertFalse(observation_path.exists())
            self.assertFalse(migration_identity(root, key).exists())

            transaction = ArtifactTransaction(root, ledger, [])
            migrated = migrate_reference(root, reference, materialize=True)
            transaction.prepare_commit(ledger.read_bytes())
            transaction.commit()
            self.assertTrue(migration_identity(root, key).is_file())
            self.assertTrue(
                durable_identity(root, str(migrated["observation_sha256"])).is_file()
            )

            migration_path = migration_identity(root, key)
            embedded = json.loads(migration_path.read_text(encoding="utf-8"))
            legacy_free = dict(embedded)
            legacy_free.pop("legacy_envelope")
            migration_path.write_text(json.dumps(legacy_free), encoding="utf-8")
            with self.assertRaisesRegex(RuntimeError, "migration publication"):
                migrate_reference(root, reference)
            transaction = ArtifactTransaction(root, ledger, [])
            migrate_reference(root, reference, materialize=True)
            transaction.prepare_commit(ledger.read_bytes())
            transaction.commit()
            self.assertEqual(
                json.loads(migration_path.read_text(encoding="utf-8")), embedded
            )

            modern_binding = {
                "schema": "worth-ui-ledger-execution-binding-v3",
                "command": binding["command"],
                "source_revision": binding["source_revision"],
                "source_state_digest": binding["source_state_digest"],
                "artifact_bindings": binding["artifact_bindings"],
            }
            alternate_envelope, alternate = create_observation(
                root, modern_binding, 0, "different physical output", "", 641
            )
            retain_envelope(root, alternate_envelope)
            lawful_migration = json.loads(migration_path.read_text(encoding="utf-8"))
            mutants = {
                "schema": "wrong-schema",
                "legacy_execution_key": "c" * 64,
                "legacy_record_digest": "d" * 64,
                "execution_binding_key": "e" * 64,
                "observation_sha256": alternate.observation_sha256,
            }
            for field, value in mutants.items():
                with self.subTest(field=field):
                    mutant = {**lawful_migration, field: value}
                    migration_path.write_text(json.dumps(mutant), encoding="utf-8")
                    before = tree_snapshot(root)
                    with self.assertRaisesRegex(RuntimeError, "provenance"):
                        migrate_reference(root, reference)
                    self.assertEqual(tree_snapshot(root), before)
            migration_path.write_text(json.dumps(lawful_migration), encoding="utf-8")
            self.assertEqual(
                migrate_reference(root, reference)["observation_sha256"],
                migrated["observation_sha256"],
            )


def tree_snapshot(root: Path) -> dict[str, bytes]:
    return {
        path.relative_to(root).as_posix(): path.read_bytes()
        for path in root.rglob("*")
        if path.is_file()
    }


if __name__ == "__main__":
    unittest.main()
