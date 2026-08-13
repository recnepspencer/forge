from __future__ import annotations

import hashlib
import json
import os
import shutil
import tempfile
import uuid
from pathlib import Path


EVIDENCE_PREFIX = "_docs/worth-ui/milestone-3.14.1-evidence/"
JOURNAL_IDENTITY = "workspaces/worth-ui/target/milestone-3141-artifact-transactions"


class ArtifactTransaction:
    """Crash-recoverable evidence publication paired with one ledger replacement."""

    def __init__(
        self,
        root: Path,
        ledger: Path,
        command_texts: list[str],
        extra_identities: tuple[str, ...] = (),
    ) -> None:
        self._root = root
        self._ledger = ledger
        recover_incomplete(root, ledger)
        journal_root = root / JOURNAL_IDENTITY
        journal_root.mkdir(parents=True, exist_ok=True)
        preparing = Path(tempfile.mkdtemp(prefix=".preparing-", dir=journal_root))
        identities = tuple(
            sorted(set(governed_artifact_identities(command_texts)) | set(extra_identities))
        )
        records = []
        for index, identity in enumerate(identities):
            source = root / identity
            backup = f"{index}.bin" if source.exists() else None
            if backup is not None:
                (preparing / backup).write_bytes(source.read_bytes())
            records.append({"identity": identity, "backup": backup})
        manifest = {
            "schema": "worth-ui-ledger-artifact-transaction-v1",
            "state": "active",
            "ledger_before_sha256": digest(ledger.read_bytes()),
            "ledger_after_sha256": None,
            "artifacts": records,
        }
        write_manifest(preparing, manifest)
        self._journal = journal_root / uuid.uuid4().hex
        os.replace(preparing, self._journal)

    def prepare_commit(self, candidate_ledger: bytes) -> None:
        manifest = read_manifest(self._journal)
        manifest["state"] = "prepared"
        manifest["ledger_after_sha256"] = digest(candidate_ledger)
        write_manifest(self._journal, manifest)

    def commit(self) -> None:
        manifest = read_manifest(self._journal)
        if manifest["state"] != "prepared" or digest(self._ledger.read_bytes()) != manifest[
            "ledger_after_sha256"
        ]:
            raise RuntimeError("artifact transaction committed without its exact ledger")
        shutil.rmtree(self._journal)

    def rollback(self) -> None:
        recover_journal(self._root, self._ledger, self._journal)


def recover_incomplete(root: Path, ledger: Path) -> None:
    journal_root = root / JOURNAL_IDENTITY
    if not journal_root.exists():
        return
    for journal in sorted(journal_root.iterdir()):
        if journal.name.startswith(".preparing-"):
            shutil.rmtree(journal)
        elif journal.is_dir():
            recover_journal(root, ledger, journal)


def recover_journal(root: Path, ledger: Path, journal: Path) -> None:
    manifest = read_manifest(journal)
    ledger_digest = digest(ledger.read_bytes())
    if (
        manifest["state"] == "prepared"
        and ledger_digest == manifest["ledger_after_sha256"]
    ):
        shutil.rmtree(journal)
        return
    if ledger_digest != manifest["ledger_before_sha256"]:
        raise RuntimeError("ledger changed outside an incomplete artifact transaction")
    for record in manifest["artifacts"]:
        destination = root / record["identity"]
        if record["backup"] is None:
            destination.unlink(missing_ok=True)
        else:
            replace_bytes(destination, (journal / record["backup"]).read_bytes())
    shutil.rmtree(journal)


def governed_artifact_identities(command_texts: list[str]) -> tuple[str, ...]:
    identities: set[str] = set()
    for command_text in command_texts:
        words = command_text.split()
        for index, word in enumerate(words[:-1]):
            identity = words[index + 1]
            if (
                word in {"--artifact", "--source"}
                and identity.startswith(EVIDENCE_PREFIX)
                and identity.endswith(".json")
            ):
                identities.add(identity)
    return tuple(sorted(identities))


def read_manifest(journal: Path) -> dict[str, object]:
    return json.loads((journal / "manifest.json").read_text(encoding="utf-8"))


def write_manifest(journal: Path, manifest: dict[str, object]) -> None:
    replace_bytes(
        journal / "manifest.json",
        json.dumps(manifest, sort_keys=True).encode("utf-8"),
    )


def digest(content: bytes) -> str:
    return hashlib.sha256(content).hexdigest()


def replace_bytes(destination: Path, content: bytes) -> None:
    destination.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary = tempfile.mkstemp(
        prefix=f".{destination.name}.", dir=destination.parent
    )
    try:
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(content)
        os.replace(temporary, destination)
    finally:
        if os.path.exists(temporary):
            os.unlink(temporary)
