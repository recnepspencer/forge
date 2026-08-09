from __future__ import annotations

import hashlib
import json
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
LEDGER = Path("_docs/worth-ui/milestone-3.14.1-proof-ledger.csv")
EVIDENCE_ROOT = Path("_docs/worth-ui/milestone-3.14.1-evidence")
GOVERNANCE_PATHS = (
    "workspaces/worth-ui",
    "_docs/worth-ui/milestone-3.14.1.md",
    "_docs/worth-ui/native-host-platform.md",
    "_docs/worth-ui/worth_ui_roadmap.md",
    "scripts/ci/run_worth_ui_ledger_test.py",
    "scripts/ci/close_worth_ui_3141_ledger.py",
    "scripts/ci/verify_worth_ui_3141_ledger.py",
    "scripts/ci/worth_ui_3141_ledger_contracts.py",
    "scripts/ci/worth_ui_3141_p1_proofs.py",
    "scripts/ci/worth_ui_3141_p2_proofs.py",
    "scripts/ci/worth_ui_ledger_source_state.py",
    "scripts/ci/run_worth_ui_compile_contracts.py",
    "scripts/ci/test_worth_ui_ledger_races.py",
    "tools/boundary-check/config/road1.toml",
)


def git_bytes(arguments: list[str]) -> bytes:
    result = subprocess.run(
        ["git", *arguments], cwd=ROOT, capture_output=True, check=False
    )
    if result.returncode != 0:
        raise RuntimeError(f"git {' '.join(arguments)} failed")
    return result.stdout


def local_package_paths() -> set[str]:
    result = subprocess.run(
        [
            "cargo", "metadata", "--manifest-path", "workspaces/worth-ui/Cargo.toml",
            "--format-version", "1", "--locked",
        ],
        cwd=ROOT,
        capture_output=True,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError("cargo metadata failed while inventorying local source roots")
    metadata = json.loads(result.stdout)
    paths: set[str] = set()
    for package in metadata["packages"]:
        if package["source"] is not None:
            continue
        manifest = Path(package["manifest_path"]).resolve()
        try:
            package_root = manifest.parent.relative_to(ROOT.resolve())
        except ValueError as error:
            raise RuntimeError(f"local package escapes repository: {manifest}") from error
        paths.add(package_root.as_posix())
        add_ancestor_manifests(paths, manifest.parent)
    return paths


def add_ancestor_manifests(paths: set[str], directory: Path) -> None:
    root = ROOT.resolve()
    current = directory.resolve()
    while current != root and root in current.parents:
        for name in ("Cargo.toml", "Cargo.lock"):
            candidate = current / name
            if candidate.is_file():
                paths.add(candidate.relative_to(root).as_posix())
        current = current.parent


def source_state_paths() -> list[str]:
    return sorted(set(GOVERNANCE_PATHS) | local_package_paths())


def source_state_digest(revision: str) -> str:
    digest = hashlib.sha256()
    digest.update(revision.encode("ascii"))
    digest.update(b"\0repository-content-v2\0")
    inventory = git_bytes([
        "ls-files", "--cached", "--others", "--exclude-standard", "-z", "--",
        *source_state_paths(),
    ])
    identities = sorted(set(item for item in inventory.split(b"\0") if item))
    for encoded_identity in identities:
        identity = encoded_identity.decode("utf-8")
        normalized = Path(identity).as_posix()
        if normalized == LEDGER.as_posix() or normalized.startswith(
            EVIDENCE_ROOT.as_posix() + "/"
        ):
            continue
        source = ROOT / identity
        digest.update(b"\0file\0" if source.is_file() else b"\0missing\0")
        digest.update(encoded_identity)
        digest.update(b"\0")
        if source.is_file():
            digest.update(source.read_bytes())
    return digest.hexdigest()
