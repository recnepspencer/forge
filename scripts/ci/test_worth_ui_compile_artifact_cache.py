from __future__ import annotations

import json
import os
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch


sys.path.insert(0, str(Path(__file__).resolve().parent))

import worth_ui_compile_artifact_cache as cache
from worth_ui_ledger_execution_cache import CACHE_ENV


class CompileArtifactCacheTests(unittest.TestCase):
    def test_current_source_compile_artifact_is_materialized_once(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            cache_root = root / "cache"
            first = root / "first.json"
            second = root / "second.json"

            def execute(_root: Path, destination: Path) -> None:
                destination.parent.mkdir(parents=True, exist_ok=True)
                destination.write_text(json.dumps(lawful_payload()), encoding="utf-8")

            with (
                patch.dict(os.environ, {CACHE_ENV: str(cache_root)}),
                patch.object(cache, "execute", side_effect=execute) as execution,
            ):
                cache.materialize(root, first, "a" * 40, "b" * 64)
                cache.materialize(root, second, "a" * 40, "b" * 64)
            self.assertEqual(execution.call_count, 1)
            self.assertEqual(first.read_bytes(), second.read_bytes())

    def test_source_or_manifest_drift_forces_fresh_compile_sessions(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            cache_root = root / "cache"

            def execute(_root: Path, destination: Path) -> None:
                destination.parent.mkdir(parents=True, exist_ok=True)
                payload = lawful_payload()
                payload["source_state_digest"] = current_state[0]
                destination.write_text(json.dumps(payload), encoding="utf-8")

            current_state = ["b" * 64]
            with (
                patch.dict(os.environ, {CACHE_ENV: str(cache_root)}),
                patch.object(cache, "execute", side_effect=execute) as execution,
            ):
                cache.materialize(root, root / "one.json", "a" * 40, current_state[0])
                current_state[0] = "c" * 64
                cache.materialize(root, root / "two.json", "a" * 40, current_state[0])
            self.assertEqual(execution.call_count, 2)

    def test_self_consistent_forged_manifest_forces_fresh_compile_sessions(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            cache_root = root / "cache"

            def execute(_root: Path, destination: Path) -> None:
                destination.parent.mkdir(parents=True, exist_ok=True)
                destination.write_text(json.dumps(lawful_payload()), encoding="utf-8")

            with (
                patch.dict(os.environ, {CACHE_ENV: str(cache_root)}),
                patch.object(cache, "execute", side_effect=execute) as execution,
            ):
                cache.materialize(root, root / "one.json", "a" * 40, "b" * 64)
                artifact = cache_root / "compile-contracts.json"
                payload = json.loads(artifact.read_text(encoding="utf-8"))
                payload["forged"] = True
                artifact.write_text(json.dumps(payload), encoding="utf-8")
                manifest = cache_root / "compile-contracts.manifest.json"
                envelope = json.loads(manifest.read_text(encoding="utf-8"))
                envelope["record"]["artifact_sha256"] = cache.digest(
                    artifact.read_bytes()
                )
                manifest.write_text(json.dumps(envelope), encoding="utf-8")
                cache.materialize(root, root / "two.json", "a" * 40, "b" * 64)
            self.assertEqual(execution.call_count, 2)


def lawful_payload() -> dict[str, str]:
    return {
        "exit_posture": "passed",
        "source_revision": "a" * 40,
        "source_state_digest": "b" * 64,
    }


if __name__ == "__main__":
    unittest.main()
