from __future__ import annotations

from contextlib import contextmanager

from runner.authority.run_identity import CANONICAL_RUNTIME_ROOT
from runner.authority.run_identity.runtime_paths import acquire_lock


@contextmanager
def acquire_telegram_update_consumer():
    """Grant one process exclusive authority to consume Telegram updates."""
    lock_path = CANONICAL_RUNTIME_ROOT / "locks" / "telegram-update-consumer.lock"
    with acquire_lock(
        lock_path,
        "Telegram updates already have an active poller; stop it before running another poll or poll-once",
    ):
        yield
