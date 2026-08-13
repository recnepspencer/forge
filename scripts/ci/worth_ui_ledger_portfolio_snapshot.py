from __future__ import annotations

import contextlib
import os
from collections.abc import Iterator

from worth_ui_ledger_source_state import source_state_digest


REVISION_ENV = "WORTH_UI_LEDGER_PORTFOLIO_SOURCE_REVISION"
DIGEST_ENV = "WORTH_UI_LEDGER_PORTFOLIO_SOURCE_STATE_DIGEST"


def source_state_for_row(revision: str) -> str:
    snapshot_revision = os.environ.get(REVISION_ENV)
    snapshot_digest = os.environ.get(DIGEST_ENV)
    if snapshot_revision is None and snapshot_digest is None:
        return source_state_digest(revision)
    if snapshot_revision != revision:
        raise RuntimeError("operational portfolio source revision drifted")
    if snapshot_digest is None or not is_digest(snapshot_digest):
        raise RuntimeError("operational portfolio source digest is invalid")
    return snapshot_digest


@contextlib.contextmanager
def operational_source_snapshot(revision: str, digest: str) -> Iterator[None]:
    if len(revision) != 40 or not is_lower_hex(revision):
        raise RuntimeError("operational portfolio revision is invalid")
    if not is_digest(digest):
        raise RuntimeError("operational portfolio digest is invalid")
    previous_revision = os.environ.get(REVISION_ENV)
    previous_digest = os.environ.get(DIGEST_ENV)
    os.environ[REVISION_ENV] = revision
    os.environ[DIGEST_ENV] = digest
    try:
        yield
    finally:
        restore(REVISION_ENV, previous_revision)
        restore(DIGEST_ENV, previous_digest)


def is_digest(value: str) -> bool:
    return len(value) == 64 and is_lower_hex(value)


def is_lower_hex(value: str) -> bool:
    return all(character in "0123456789abcdef" for character in value)


def restore(name: str, value: str | None) -> None:
    if value is None:
        os.environ.pop(name, None)
    else:
        os.environ[name] = value
