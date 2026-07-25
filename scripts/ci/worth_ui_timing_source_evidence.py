import hashlib
import subprocess
from pathlib import Path
from typing import Any

from worth_ui_test_topology_config import Violation, load_json, required_string


def source_snapshot_violations(
    root: Path, label: str, run_set: dict[str, Any]
) -> list[Violation]:
    snapshot = run_set.get("source_snapshot")
    if not isinstance(snapshot, dict):
        return [Violation("timing-evidence-source", f"{label}.source_snapshot is missing")]
    if snapshot.get("algorithm") != "sha256-path-and-git-blob-v1":
        return [
            Violation(
                "timing-evidence-source",
                f"{label}.source_snapshot.algorithm is unsupported",
            )
        ]
    scope = snapshot.get("scope")
    if not isinstance(scope, list) or not scope or not all(
        isinstance(path, str) and path for path in scope
    ):
        return [
            Violation("timing-evidence-source", f"{label}.source_snapshot.scope is invalid")
        ]
    kind = snapshot.get("kind")
    try:
        if kind == "working_tree":
            digest, file_count = filesystem_source_digest(root, scope)
        elif kind == "git_commit":
            digest, file_count = git_commit_source_digest(
                root, required_string(run_set, "git_commit"), scope
            )
        elif kind == "captured_working_tree":
            digest, file_count = captured_working_tree_handoff(root, snapshot)
        else:
            return [
                Violation(
                    "timing-evidence-source",
                    f"{label}.source_snapshot.kind must be working_tree, "
                    "git_commit, or captured_working_tree",
                )
            ]
    except (OSError, subprocess.CalledProcessError, ValueError) as error:
        return [
            Violation(
                "timing-evidence-source",
                f"{label}.source_snapshot could not be verified: {error}",
            )
        ]
    violations = []
    if snapshot.get("digest") != digest:
        violations.append(
            Violation(
                "timing-evidence-source",
                f"{label}.source_snapshot.digest does not match the declared source bytes",
            )
        )
    if snapshot.get("file_count") != file_count:
        violations.append(
            Violation(
                "timing-evidence-source",
                f"{label}.source_snapshot.file_count must be {file_count}",
            )
        )
    return violations


def captured_working_tree_handoff(
    root: Path, snapshot: dict[str, Any]
) -> tuple[str, int]:
    handoff_path = (root / required_string(snapshot, "handoff_evidence")).resolve()
    if not handoff_path.is_relative_to(root.resolve()) or not handoff_path.is_file():
        raise ValueError("captured working-tree handoff evidence is unavailable")
    handoff = load_json(handoff_path)
    opening = handoff.get("opening")
    if not isinstance(opening, dict):
        raise ValueError("captured working-tree handoff opening is missing")
    handoff_snapshot = opening.get("source_snapshot")
    if not isinstance(handoff_snapshot, dict):
        raise ValueError("captured working-tree handoff source snapshot is missing")
    for field in ("algorithm", "scope", "digest", "file_count"):
        if handoff_snapshot.get(field) != snapshot.get(field):
            raise ValueError(f"captured working-tree handoff {field} does not match")
    return required_string(handoff_snapshot, "digest"), handoff_snapshot["file_count"]


def filesystem_source_digest(root: Path, scope: list[str]) -> tuple[str, int]:
    files: set[Path] = set()
    for raw in scope:
        scoped = root / raw
        if scoped.is_file():
            files.add(scoped)
        elif scoped.is_dir():
            files.update(
                path
                for path in scoped.rglob("*")
                if path.is_file()
                and not any(part in {".git", "target", "__pycache__"} for part in path.parts)
            )
        else:
            raise ValueError(f"source scope does not exist: {raw}")
    entries = []
    for path in sorted(files):
        relative = path.relative_to(root).as_posix()
        entries.append((relative, git_blob_digest(path.read_bytes())))
    return aggregate_source_entries(entries), len(entries)


def git_commit_source_digest(
    root: Path, commit: str, scope: list[str]
) -> tuple[str, int]:
    command = ["git", "ls-tree", "-r", "-z", commit, "--", *scope]
    result = subprocess.run(command, cwd=root, check=True, capture_output=True)
    entries = []
    for raw in result.stdout.split(b"\0"):
        if not raw:
            continue
        metadata, raw_path = raw.split(b"\t", 1)
        _, object_type, object_id = metadata.split(b" ", 2)
        if object_type != b"blob":
            continue
        entries.append((raw_path.decode("utf-8"), object_id.decode("ascii")))
    return aggregate_source_entries(entries), len(entries)


def git_blob_digest(data: bytes) -> str:
    header = f"blob {len(data)}\0".encode("ascii")
    return hashlib.sha1(header + data).hexdigest()


def aggregate_source_entries(entries: list[tuple[str, str]]) -> str:
    digest = hashlib.sha256()
    for path, object_id in sorted(entries):
        digest.update(path.encode("utf-8"))
        digest.update(b"\0")
        digest.update(object_id.encode("ascii"))
        digest.update(b"\n")
    return digest.hexdigest()


def source_transition_digest(opening_digest: str, closing_digest: str) -> str:
    return hashlib.sha256(
        opening_digest.encode("ascii") + b"\0" + closing_digest.encode("ascii")
    ).hexdigest()


def source_transition_violations(opening, closing) -> list[Violation]:
    opening_snapshot = opening.get("source_snapshot", {})
    closing_snapshot = closing.get("source_snapshot", {})
    if not isinstance(opening_snapshot, dict) or not isinstance(closing_snapshot, dict):
        return []
    opening_digest = opening_snapshot.get("digest")
    closing_digest = closing_snapshot.get("digest")
    if not isinstance(opening_digest, str) or not isinstance(closing_digest, str):
        return []
    expected = source_transition_digest(opening_digest, closing_digest)
    if closing.get("source_transition_digest") != expected:
        return [
            Violation(
                "timing-evidence-source",
                "closing.source_transition_digest does not bind opening and closing source trees",
            )
        ]
    return []
