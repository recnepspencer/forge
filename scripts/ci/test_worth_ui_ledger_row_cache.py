from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parent))

from worth_ui_ledger_row_cache import RowEvidenceCache, replace_with_retry
from worth_ui_ledger_row_execution import CachedEvidenceRejected, execute_or_restore
from worth_ui_ledger_runner_authentication import authentication_tag


class RowEvidenceCacheTests(unittest.TestCase):
    def test_predecessor_refresh_can_bypass_row_restore(self) -> None:
        class Cache:
            def restore(self, *_args):
                raise AssertionError("predecessor row cache was consulted")

            def retain(self, *args):
                self.retained = args

        cache = Cache()
        result = execute_or_restore(
            {"requirement": "P5-PREDECESSOR-01", "exact_command": "runner"},
            Path("candidate.csv"),
            cache,
            "c" * 64,
            lambda command, candidate: {
                "command": command,
                "candidate": str(candidate),
            },
            restore=False,
        )
        self.assertEqual(result["candidate"], "candidate.csv")
        self.assertEqual(cache.retained[-1], result)

    def test_closure_wiring_does_not_execute_a_restored_row(self) -> None:
        class Cache:
            def restore(self, requirement, command, claim):
                self.observed = (requirement, command, claim)
                return {"exit_posture": "passed"}

            def retain(self, *_args):
                raise AssertionError("restored row was retained again")

        cache = Cache()
        result = execute_or_restore(
            {"requirement": "P4-ROW-01", "exact_command": "runner"},
            Path("candidate.csv"),
            cache,
            "c" * 64,
            lambda *_args: self.fail("restored row executed again"),
        )
        self.assertEqual(result["exit_posture"], "passed")
        self.assertEqual(cache.observed, ("P4-ROW-01", "runner", "c" * 64))

    def test_mapping_rejected_cache_executes_and_replaces_the_receipt(self) -> None:
        class Cache:
            def restore(self, *_args):
                return {"mapping": "stale"}

            def retain(self, *args):
                self.retained = args

        cache = Cache()
        executions = []

        def execute(command, candidate):
            executions.append((command, candidate))
            return {"mapping": "current"}

        def finalize(result):
            if result["mapping"] == "stale":
                raise CachedEvidenceRejected("mapping changed")
            return result

        result = execute_or_restore(
            {"requirement": "P5-ROW-01", "exact_command": "runner"},
            Path("candidate.csv"),
            cache,
            "c" * 64,
            execute,
            finalize,
        )
        self.assertEqual(result, {"mapping": "current"})
        self.assertEqual(executions, [("runner", Path("candidate.csv"))])
        self.assertEqual(cache.retained[-1], result)

    def test_consumer_cache_never_rewinds_its_dependency_input(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            artifact = "_docs/worth-ui/milestone-3.14.1-evidence/p4-row.json"
            dependency = "_docs/worth-ui/milestone-3.14.1-evidence/p4-input.json"
            command = f"python runner --source {dependency} --artifact {artifact}"
            write(root / dependency, b"input")
            payload = lawful_payload(root)
            write(root / artifact, json.dumps(payload).encode())
            cache = RowEvidenceCache(root, root / "cache", b"ledger", "a" * 40, "b" * 64)
            result = {"artifact_sha256": sha(root / artifact), **payload}
            cache.retain("P4-ROW-01", command, "c" * 64, result)
            write(root / dependency, b"rolled-back")
            (root / artifact).unlink()
            restored = cache.restore("P4-ROW-01", command, "c" * 64)
            self.assertIsNone(restored)
            self.assertEqual((root / dependency).read_bytes(), b"rolled-back")
            write(root / dependency, b"input")
            restored = cache.restore("P4-ROW-01", command, "c" * 64)
            self.assertIsNotNone(restored)
            self.assertEqual((root / dependency).read_bytes(), b"input")
            self.assertEqual(json.loads((root / artifact).read_text())["exit_posture"], "passed")

    def test_predecessor_cache_restores_its_exact_authenticated_handoff(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            artifact = "_docs/worth-ui/milestone-3.14.1-evidence/p4-predecessor-01.json"
            handoff = "_docs/worth-ui/milestone-3.14.1-evidence/p4-predecessor-handoff.json"
            command = f"python runner --source {handoff} --artifact {artifact}"
            write(root / handoff, b"authenticated handoff")
            compile_artifact = (
                root / "_docs/worth-ui/milestone-3.14.1-evidence/compile-contracts.json"
            )
            write(compile_artifact, b"authenticated compile contracts")
            payload = lawful_payload(root)
            payload["requirement"] = "P4-PREDECESSOR-01"
            payload.pop("runner_authentication")
            payload["runner_authentication"] = authentication_tag(payload, root)
            write(root / artifact, json.dumps(payload).encode())
            cache = RowEvidenceCache(root, root / "cache", b"ledger", "a" * 40, "b" * 64)
            result = {"artifact_sha256": sha(root / artifact), **payload}
            cache.retain("P4-PREDECESSOR-01", command, "c" * 64, result)
            (root / artifact).unlink()
            (root / handoff).unlink()
            compile_artifact.unlink()
            restored = cache.restore("P4-PREDECESSOR-01", command, "c" * 64)
            self.assertIsNotNone(restored)
            self.assertEqual((root / handoff).read_bytes(), b"authenticated handoff")
            self.assertEqual(
                compile_artifact.read_bytes(), b"authenticated compile contracts"
            )

    def test_claim_drift_rejects_but_unrelated_source_state_drift_reuses_row(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            artifact = "_docs/worth-ui/milestone-3.14.1-evidence/p4-row.json"
            command = f"python runner --artifact {artifact}"
            payload = lawful_payload(root)
            write(root / artifact, json.dumps(payload).encode())
            cache = RowEvidenceCache(root, root / "cache", b"ledger", "a" * 40, "b" * 64)
            cache.retain(
                "P4-ROW-01", command, "c" * 64,
                {"artifact_sha256": sha(root / artifact), **payload},
            )
            self.assertIsNone(cache.restore("P4-ROW-01", command, "d" * 64))
            drifted = RowEvidenceCache(root, root / "cache", b"ledger", "a" * 40, "e" * 64)
            self.assertIsNotNone(drifted.restore("P4-ROW-01", command, "c" * 64))

    def test_content_mutation_invalidates_cached_bundle(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            artifact = "_docs/worth-ui/milestone-3.14.1-evidence/p4-row.json"
            command = f"python runner --artifact {artifact}"
            payload = lawful_payload(root)
            write(root / artifact, json.dumps(payload).encode())
            cache = RowEvidenceCache(root, root / "cache", b"ledger", "a" * 40, "b" * 64)
            cache.retain(
                "P4-ROW-01", command, "c" * 64,
                {"artifact_sha256": sha(root / artifact), **payload},
            )
            stored = next((root / "cache").rglob("*.bin"))
            stored.write_bytes(b"forged")
            self.assertIsNone(cache.restore("P4-ROW-01", command, "c" * 64))

    def test_retain_is_idempotent_and_atomically_replaces_a_damaged_bundle(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            artifact = "_docs/worth-ui/milestone-3.14.1-evidence/p4-row.json"
            command = f"python runner --artifact {artifact}"
            payload = lawful_payload(root)
            write(root / artifact, json.dumps(payload).encode())
            cache = RowEvidenceCache(root, root / "cache", b"ledger", "a" * 40, "b" * 64)
            result = {"artifact_sha256": sha(root / artifact), **payload}
            cache.retain("P4-ROW-01", command, "c" * 64, result)
            cache.retain("P4-ROW-01", command, "c" * 64, result)
            manifest = next((root / "cache").rglob("manifest.json"))
            manifest.write_text("damaged", encoding="utf-8")
            cache.retain("P4-ROW-01", command, "c" * 64, result)
            self.assertIsNotNone(cache.restore("P4-ROW-01", command, "c" * 64))

    def test_atomic_publication_retries_a_transient_windows_reader(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source, destination = root / "source", root / "destination"
            source.mkdir()
            real_replace = __import__("os").replace
            attempts = 0

            def transient_replace(left, right):
                nonlocal attempts
                attempts += 1
                if attempts == 1:
                    raise PermissionError("injected reader")
                return real_replace(left, right)

            with patch("worth_ui_ledger_row_cache.os.replace", transient_replace), patch(
                "worth_ui_ledger_row_cache.time.sleep"
            ):
                replace_with_retry(source, destination)
            self.assertEqual(attempts, 2)
            self.assertTrue(destination.is_dir())

    def test_self_consistent_forged_bundle_is_not_runner_authenticated(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            artifact = "evidence/result.json"
            command = f"python runner --artifact {artifact}"
            payload = lawful_payload(root)
            write(root / artifact, json.dumps(payload).encode())
            cache = RowEvidenceCache(root, root / "cache", b"ledger", "a" * 40, "b" * 64)
            cache.retain(
                "P4-ROW-01",
                command,
                "c" * 64,
                {"artifact_sha256": sha(root / artifact), **payload},
            )
            manifest_identity = next((root / "cache").rglob("manifest.json"))
            envelope = json.loads(manifest_identity.read_text(encoding="utf-8"))
            stored = manifest_identity.parent / "0.bin"
            forged = json.loads(stored.read_text(encoding="utf-8"))
            forged["run_nonce"] = "forged-but-content-addressed"
            stored.write_text(json.dumps(forged), encoding="utf-8")
            forged_digest = sha(stored)
            envelope["manifest"]["artifact_sha256"] = forged_digest
            envelope["manifest"]["files"][0]["sha256"] = forged_digest
            from worth_ui_ledger_row_cache import digest_json

            envelope["manifest_sha256"] = digest_json(envelope["manifest"])
            manifest_identity.write_text(json.dumps(envelope), encoding="utf-8")
            self.assertIsNone(cache.restore("P4-ROW-01", command, "c" * 64))

    def test_authenticated_manifest_cannot_restore_a_source_or_extra_file(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = "evidence/dependency.json"
            artifact = "evidence/result.json"
            command = f"python runner --source {source} --artifact {artifact}"
            write(root / source, b"current")
            payload = lawful_payload(root)
            write(root / artifact, json.dumps(payload).encode())
            cache = RowEvidenceCache(root, root / "cache", b"ledger", "a" * 40, "b" * 64)
            cache.retain(
                "P4-ROW-01",
                command,
                "c" * 64,
                {"artifact_sha256": sha(root / artifact), **payload},
            )
            manifest_identity = next((root / "cache").rglob("manifest.json"))
            envelope = json.loads(manifest_identity.read_text(encoding="utf-8"))
            source_bytes = b"rewound"
            (manifest_identity.parent / "1.bin").write_bytes(source_bytes)
            envelope["manifest"]["files"].append(
                {
                    "identity": source,
                    "stored": "1.bin",
                    "sha256": __import__("hashlib").sha256(source_bytes).hexdigest(),
                }
            )
            from worth_ui_ledger_row_cache import digest_json

            envelope["manifest_sha256"] = digest_json(envelope["manifest"])
            envelope["runner_authentication"] = authentication_tag(
                envelope["manifest"], root
            )
            manifest_identity.write_text(json.dumps(envelope), encoding="utf-8")
            self.assertIsNone(cache.restore("P4-ROW-01", command, "c" * 64))
            self.assertEqual((root / source).read_bytes(), b"current")

    def test_row_artifact_path_must_remain_inside_repository_root(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            base = Path(directory)
            root = base / "repository"
            root.mkdir()
            payload = lawful_payload(root)
            escaped = base / "escaped.json"
            escaped.write_text(json.dumps(payload), encoding="utf-8")
            command = "python runner --artifact ../escaped.json"
            cache = RowEvidenceCache(root, root / "cache", b"ledger", "a" * 40, "b" * 64)
            with self.assertRaises(ValueError):
                cache.retain(
                    "P4-ROW-01",
                    command,
                    "c" * 64,
                    {"artifact_sha256": sha(escaped), **payload},
                )


def lawful_payload(root: Path) -> dict[str, object]:
    payload = {
        "requirement": "P4-ROW-01",
        "exit_posture": "passed",
        "claim_digest": "c" * 64,
        "source_revision": "a" * 40,
        "source_state_digest": "b" * 64,
    }
    payload["runner_authentication"] = authentication_tag(payload, root)
    return payload


def write(identity: Path, content: bytes) -> None:
    identity.parent.mkdir(parents=True, exist_ok=True)
    identity.write_bytes(content)


def sha(identity: Path) -> str:
    import hashlib

    return hashlib.sha256(identity.read_bytes()).hexdigest()


if __name__ == "__main__":
    unittest.main()
