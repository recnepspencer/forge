from __future__ import annotations

import hashlib
import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

from worth_ui_ledger_execution_cache import CACHE_ENV
from worth_ui_ledger_runner_authentication import authentication_tag, authenticates


SCHEMA = "worth-ui-compile-artifact-cache-v2"


def materialize(
    root: Path,
    destination: Path,
    revision: str,
    state_digest: str,
) -> None:
    cache_root = os.environ.get(CACHE_ENV)
    if cache_root is None:
        execute(root, destination)
        return
    cache = Path(cache_root) / "compile-contracts.json"
    manifest = cache.with_suffix(".manifest.json")
    if not valid(root, cache, manifest, revision, state_digest):
        cache.parent.mkdir(parents=True, exist_ok=True)
        execute(root, cache)
        record = {
            "schema": SCHEMA,
            "source_revision": revision,
            "source_state_digest": state_digest,
            "artifact_sha256": digest(cache.read_bytes()),
        }
        replace_json(
            manifest,
            {
                "record": record,
                "runner_authentication": authentication_tag(record, root),
            },
        )
        print("[compile:execute] fresh two-session contract artifact", file=sys.stderr)
    else:
        print("[compile:reuse] current-source contract artifact", file=sys.stderr)
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(cache, destination)


def execute(root: Path, destination: Path) -> None:
    identity = destination.resolve().relative_to(root.resolve()).as_posix()
    subprocess.run(
        [
            sys.executable,
            "scripts/ci/run_worth_ui_compile_contracts.py",
            "--artifact",
            identity,
        ],
        cwd=root,
        check=True,
    )


def valid(
    root: Path,
    artifact: Path,
    manifest: Path,
    revision: str,
    state_digest: str,
) -> bool:
    try:
        envelope = json.loads(manifest.read_text(encoding="utf-8"))
        record = envelope["record"]
        payload = json.loads(artifact.read_text(encoding="utf-8"))
        return (
            authenticates(record, envelope.get("runner_authentication"), root)
            and record
            == {
                "schema": SCHEMA,
                "source_revision": revision,
                "source_state_digest": state_digest,
                "artifact_sha256": digest(artifact.read_bytes()),
            }
            and payload.get("exit_posture") == "passed"
            and payload.get("source_revision") == revision
            and payload.get("source_state_digest") == state_digest
        )
    except (KeyError, OSError, ValueError, json.JSONDecodeError):
        return False


def replace_json(destination: Path, value: object) -> None:
    descriptor, temporary = tempfile.mkstemp(prefix=".compile-", dir=destination.parent)
    try:
        with os.fdopen(descriptor, "w", encoding="utf-8") as stream:
            json.dump(value, stream, sort_keys=True)
            stream.write("\n")
        os.replace(temporary, destination)
    finally:
        if os.path.exists(temporary):
            os.unlink(temporary)


def digest(content: bytes) -> str:
    return hashlib.sha256(content).hexdigest()
