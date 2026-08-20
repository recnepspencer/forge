from __future__ import annotations

import hashlib
import json
import os
import secrets
import shutil
import tempfile
import time
from pathlib import Path
from typing import Any

from worth_ui_ledger_runner_authentication import authentication_tag, authenticates


SCHEMA = "worth-ui-ledger-row-receipt-v3"


class RowEvidenceCache:
    def __init__(
        self,
        root: Path,
        cache_root: Path,
        ledger_basis: bytes,
        revision: str,
        state_digest: str,
    ) -> None:
        self._root = root
        self._cache_portfolio_root = cache_root.parent
        self._cache_root = cache_root / "rows"
        self._ledger_digest = digest(ledger_basis)
        self._revision = revision
        self._state_digest = state_digest

    def restore(
        self, requirement: str, command: str, claim_digest: str
    ) -> dict[str, Any] | None:
        binding = self.binding(requirement, command, claim_digest)
        identities = [self.identity(binding)]
        identities.extend(
            path.parent
            for path in self._cache_portfolio_root.glob(
                f"*/rows/{requirement.lower()}/*/manifest.json"
            )
            if path.parent not in identities
        )
        for identity in identities:
            restored = self.restore_identity(
                identity, requirement, command, claim_digest, binding
            )
            if restored is not None:
                return restored
        return None

    def restore_identity(
        self,
        identity: Path,
        requirement: str,
        command: str,
        claim_digest: str,
        binding: dict[str, object],
    ) -> dict[str, Any] | None:
        try:
            envelope = json.loads((identity / "manifest.json").read_text(encoding="utf-8"))
            manifest = envelope["manifest"]
            if (
                envelope.get("manifest_sha256") != digest_json(manifest)
                or not authenticates(
                    manifest, envelope.get("runner_authentication"), self._root
                )
                or not causal_binding_matches(manifest.get("binding"), binding)
            ):
                return None
            artifact = artifact_identity(command)
            owned = owned_artifact_identities(requirement, command)
            contents = self.validated_contents(identity, manifest, owned)
            payload = json.loads(contents[artifact].decode("utf-8"))
            if not valid_restored_payload(payload, requirement, claim_digest, self._root):
                return None
            artifact_digest = digest(contents[artifact])
            if manifest.get("artifact_sha256") != artifact_digest:
                return None
            replace_bytes(contained_artifact(self._root, artifact), contents[artifact])
            for owned_identity in owned:
                if owned_identity != artifact:
                    replace_bytes(
                        contained_artifact(self._root, owned_identity),
                        contents[owned_identity],
                    )
            return {"artifact_sha256": artifact_digest, **payload}
        except (KeyError, OSError, TypeError, ValueError, json.JSONDecodeError):
            return None

    def retain(
        self,
        requirement: str,
        command: str,
        claim_digest: str,
        result: dict[str, Any],
    ) -> None:
        binding = self.binding(requirement, command, claim_digest)
        destination = self.identity(binding)
        destination.parent.mkdir(parents=True, exist_ok=True)
        preparing = Path(tempfile.mkdtemp(prefix=".row-", dir=destination.parent))
        artifact = artifact_identity(command)
        owned = owned_artifact_identities(requirement, command)
        if any(not contained_artifact(self._root, item).is_file() for item in owned):
            shutil.rmtree(preparing)
            raise RuntimeError("row cache cannot retain a missing owned artifact")
        content = contained_artifact(self._root, artifact).read_bytes()
        if result.get("artifact_sha256") != digest(content):
            shutil.rmtree(preparing)
            raise RuntimeError("row cache result digest differs from its artifact")
        payload = json.loads(content.decode("utf-8"))
        if not valid_payload(
            payload,
            requirement,
            claim_digest,
            self._revision,
            self._state_digest,
            self._root,
        ):
            shutil.rmtree(preparing)
            raise RuntimeError("row cache cannot retain unauthenticated result evidence")
        records = []
        for index, owned_identity in enumerate(owned):
            owned_content = contained_artifact(self._root, owned_identity).read_bytes()
            stored = f"{index}.bin"
            (preparing / stored).write_bytes(owned_content)
            records.append(
                {"identity": owned_identity, "stored": stored, "sha256": digest(owned_content)}
            )
        manifest = {
            "schema": SCHEMA,
            "binding": binding,
            "artifact_sha256": result["artifact_sha256"],
            "files": records,
        }
        envelope = {
            "manifest": manifest,
            "manifest_sha256": digest_json(manifest),
            "runner_authentication": authentication_tag(manifest, self._root),
        }
        (preparing / "manifest.json").write_text(
            json.dumps(envelope, sort_keys=True) + "\n", encoding="utf-8"
        )
        if destination.exists() and self.existing_matches(destination, envelope):
            shutil.rmtree(preparing, ignore_errors=True)
            return
        # Keep the swap name independent of the 64-byte binding identity.  The
        # cache root is already deep enough that repeating that identity can
        # cross the legacy Windows MAX_PATH boundary during os.replace.
        retired = destination.parent / f".old-{secrets.token_hex(8)}"
        if destination.exists():
            replace_with_retry(destination, retired)
        try:
            replace_with_retry(preparing, destination)
        except BaseException:
            if not destination.exists() and retired.exists():
                replace_with_retry(retired, destination)
            raise
        finally:
            shutil.rmtree(retired, ignore_errors=True)

    def binding(
        self, requirement: str, command: str, claim_digest: str
    ) -> dict[str, str]:
        return {
            "schema": SCHEMA,
            "requirement": requirement,
            "exact_command": command,
            "claim_digest": claim_digest,
            "ledger_basis_sha256": self._ledger_digest,
            "source_revision": self._revision,
            "source_state_digest": self._state_digest,
            "source_artifact_bindings": source_artifact_bindings(
                self._root, command, requirement
            ),
        }

    def identity(self, binding: dict[str, str]) -> Path:
        return self._cache_root / binding["requirement"].lower() / digest_json(binding)

    @staticmethod
    def validated_contents(
        identity: Path, manifest: dict[str, Any], artifacts: tuple[str, ...]
    ) -> dict[str, bytes]:
        records = manifest.get("files")
        if not isinstance(records, list) or len(records) != len(artifacts):
            raise ValueError("row cache owns the wrong artifact inventory")
        contents = {}
        for record, artifact in zip(records, artifacts, strict=True):
            if not isinstance(record, dict) or record.get("identity") != artifact:
                raise ValueError("row cache attempted to restore a non-owned artifact")
            stored = (identity / str(record.get("stored", ""))).resolve()
            stored.relative_to(identity.resolve())
            content = stored.read_bytes()
            if digest(content) != record.get("sha256"):
                raise ValueError("row cache content drifted")
            contents[artifact] = content
        return contents

    @staticmethod
    def existing_matches(identity: Path, expected: dict[str, Any]) -> bool:
        try:
            observed = json.loads((identity / "manifest.json").read_text(encoding="utf-8"))
            return observed == expected
        except (OSError, json.JSONDecodeError):
            return False


def artifact_identity(command: str) -> str:
    words = command.split()
    try:
        return words[words.index("--artifact") + 1]
    except (ValueError, IndexError) as error:
        raise ValueError("governed command omits result artifact") from error


def owned_artifact_identities(requirement: str, command: str) -> tuple[str, ...]:
    result = artifact_identity(command)
    if requirement not in {
        "P3-PREDECESSOR-01",
        "P4-PREDECESSOR-01",
        "P5-PREDECESSOR-01",
    }:
        return (result,)
    suffix = f"p{requirement[1]}-predecessor-handoff.json"
    sources = [
        command.split()[index + 1]
        for index, word in enumerate(command.split()[:-1])
        if word == "--source" and command.split()[index + 1].endswith(suffix)
    ]
    if len(sources) != 1:
        raise ValueError("predecessor row must own exactly one handoff artifact")
    return (result, sources[0])


def source_artifact_bindings(
    root: Path, command: str, requirement: str | None = None
) -> dict[str, str]:
    words = command.split()
    bindings = {}
    for index, word in enumerate(words[:-1]):
        if word != "--source":
            continue
        identity = words[index + 1]
        if requirement in {
            "P3-PREDECESSOR-01",
            "P4-PREDECESSOR-01",
            "P5-PREDECESSOR-01",
        } and identity.endswith(
            f"p{requirement[1]}-predecessor-handoff.json"
        ):
            continue
        source = root / identity
        bindings[identity] = digest(source.read_bytes()) if source.is_file() else "missing"
    return bindings


def valid_payload(
    payload: dict[str, Any],
    requirement: str,
    claim_digest: str,
    revision: str,
    state_digest: str,
    root: Path,
) -> bool:
    unsigned = {key: value for key, value in payload.items() if key != "runner_authentication"}
    return (
        payload.get("requirement") == requirement
        and payload.get("exit_posture") == "passed"
        and payload.get("claim_digest") == claim_digest
        and payload.get("source_revision") == revision
        and payload.get("source_state_digest") == state_digest
        and authenticates(unsigned, payload.get("runner_authentication"), root)
    )


def valid_restored_payload(
    payload: dict[str, Any],
    requirement: str,
    claim_digest: str,
    root: Path,
) -> bool:
    unsigned = {key: value for key, value in payload.items() if key != "runner_authentication"}
    return (
        payload.get("requirement") == requirement
        and payload.get("exit_posture") == "passed"
        and payload.get("claim_digest") == claim_digest
        and authenticates(unsigned, payload.get("runner_authentication"), root)
    )


def causal_binding_matches(
    observed: object, expected: dict[str, object]
) -> bool:
    if not isinstance(observed, dict):
        return False
    return all(
        observed.get(field) == expected.get(field)
        for field in (
            "schema",
            "requirement",
            "exact_command",
            "claim_digest",
            "source_artifact_bindings",
        )
    )


def contained_artifact(root: Path, identity: str) -> Path:
    candidate = (root / identity).resolve()
    candidate.relative_to(root.resolve())
    if Path(identity).is_absolute():
        raise ValueError("row artifact path must be repository-relative")
    return candidate


def digest(content: bytes) -> str:
    return hashlib.sha256(content).hexdigest()


def digest_json(value: object) -> str:
    return digest(
        json.dumps(value, sort_keys=True, separators=(",", ":")).encode("utf-8")
    )


def replace_bytes(destination: Path, content: bytes) -> None:
    destination.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary = tempfile.mkstemp(prefix=f".{destination.name}.", dir=destination.parent)
    try:
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(content)
        os.replace(temporary, destination)
    finally:
        if os.path.exists(temporary):
            os.unlink(temporary)


def replace_with_retry(source: Path, destination: Path) -> None:
    for attempt in range(20):
        try:
            os.replace(source, destination)
            return
        except PermissionError:
            if attempt == 19:
                raise
            time.sleep(0.05 * (attempt + 1))
