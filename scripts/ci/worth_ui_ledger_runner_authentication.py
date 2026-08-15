from __future__ import annotations

import hashlib
import hmac
import json
import os
import secrets
from pathlib import Path


KEY_BYTES = 32
KEY_FILE = "ledger-runner-hmac-v1.key"


class RunnerProvenanceUnavailable(RuntimeError):
    """The retained proof was produced by a different private runner key."""


def authentication_tag(value: object, repository_root: Path) -> str:
    return hmac.new(
        machine_key(repository_root), canonical_json(value), hashlib.sha256
    ).hexdigest()


def authenticates(value: object, tag: object, repository_root: Path) -> bool:
    return isinstance(tag, str) and hmac.compare_digest(
        authentication_tag(value, repository_root), tag
    )


def runner_key_fingerprint(repository_root: Path) -> str:
    return hashlib.sha256(machine_key(repository_root)).hexdigest()


def machine_key(repository_root: Path) -> bytes:
    identity = machine_key_identity()
    require_runner_private_location(identity, repository_root)
    identity.parent.mkdir(parents=True, exist_ok=True)
    if identity.is_symlink():
        raise RuntimeError("ledger runner key cannot be a symbolic link")
    try:
        descriptor = os.open(identity, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    except FileExistsError:
        pass
    else:
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(secrets.token_bytes(KEY_BYTES))
    content = identity.read_bytes()
    if len(content) != KEY_BYTES:
        raise RuntimeError("ledger runner key has an invalid length")
    if os.name != "nt" and identity.stat().st_mode & 0o077:
        raise RuntimeError("ledger runner key permissions are not private")
    return content


def machine_key_identity() -> Path:
    if os.name == "nt":
        base = Path(os.environ.get("LOCALAPPDATA", Path.home() / "AppData/Local"))
    else:
        base = Path(os.environ.get("XDG_STATE_HOME", Path.home() / ".local/state"))
    return base / "Worth" / "ledger-runner" / KEY_FILE


def require_runner_private_location(identity: Path, repository_root: Path) -> None:
    try:
        identity.resolve().relative_to(repository_root.resolve())
    except ValueError:
        return
    raise RuntimeError("ledger runner key must remain outside the repository")


def canonical_json(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode("utf-8")
