from __future__ import annotations

import json
from contextlib import contextmanager
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from runner.authority.run_identity import CANONICAL_RUNTIME_ROOT, RuntimePaths
from runner.authority.run_identity.runtime_paths import acquire_lock
from runner.facade.lifecycle import inject_operator_override, operator_override_source_exists


@dataclass(frozen=True)
class ReplyDisposition:
    status: str
    detail: str
    signal_id: str | None = None
    run_id: str | None = None


def record_alert(payload: dict[str, Any], telegram_message_id: int, chat_id: str) -> None:
    run_id = required_string(payload, "run_id")
    record = {
        "signal_id": required_string(payload, "signal_id"),
        "run_id": run_id,
        "phase_id": payload.get("phase_id"),
        "turn": payload.get("turn"),
        "telegram_message_id": telegram_message_id,
        "chat_id": str(chat_id),
    }
    path = RuntimePaths(run_id).telegram_alerts
    append_json_line(path, record)


def route_reply(update: dict[str, Any], expected_chat_id: str) -> ReplyDisposition:
    message = update.get("message")
    if not isinstance(message, dict) or str(message.get("chat", {}).get("id")) != str(expected_chat_id):
        return receipt(update, ReplyDisposition("rejected_wrong_chat", "message is not from the configured chat"))
    reply = message.get("reply_to_message")
    text = message.get("text")
    if not isinstance(reply, dict) or not isinstance(text, str) or not text.strip():
        return receipt(update, ReplyDisposition("rejected_unthreaded", "reply directly to a runner alert"))
    alert = find_alert(str(expected_chat_id), reply.get("message_id"))
    if alert is None:
        return receipt(update, ReplyDisposition("rejected_unmapped", "the replied-to message is not a recorded runner alert"))
    update_id = update.get("update_id")
    if not isinstance(update_id, int):
        return receipt(update, ReplyDisposition("rejected_invalid", "Telegram update has no numeric update_id"))
    source_id = f"telegram:{update_id}"
    if operator_override_source_exists(RuntimePaths(alert["run_id"]), source_id):
        return receipt(
            update,
            ReplyDisposition(
                "duplicate_ignored",
                "Telegram update was already recorded for this run",
                alert["signal_id"],
                alert["run_id"],
            ),
        )
    try:
        inject_operator_override(
            alert["run_id"],
            text.strip(),
            phase_id=alert.get("phase_id"),
            turn=alert.get("turn"),
            source_id=source_id,
        )
    except ValueError as error:
        # A completed, stopped, or advanced run makes an old alert stale.  The
        # update is intentionally consumed instead of repeatedly waking it.
        return receipt(update, ReplyDisposition("rejected_stale", str(error), alert["signal_id"], alert["run_id"]))
    return receipt(update, ReplyDisposition("injected", "instruction recorded for the active runner cursor", alert["signal_id"], alert["run_id"]))


def receipt(update: dict[str, Any], disposition: ReplyDisposition) -> ReplyDisposition:
    update_id = update.get("update_id")
    record = {
        "at": datetime.now(timezone.utc).isoformat(),
        "update_id": update_id,
        "status": disposition.status,
        "detail": disposition.detail,
        "signal_id": disposition.signal_id,
        "run_id": disposition.run_id,
    }
    append_json_line(telegram_receipts_path(), record)
    return disposition


def find_alert(chat_id: str, message_id: object) -> dict[str, Any] | None:
    if not isinstance(message_id, int):
        return None
    latest: dict[str, Any] | None = None
    for path in (CANONICAL_RUNTIME_ROOT / "telegram").glob("*.jsonl"):
        for line in path.read_text(encoding="utf-8").splitlines():
            try:
                record = json.loads(line)
            except json.JSONDecodeError as error:
                raise ValueError(f"Telegram alert mapping is unreadable: {path}") from error
            if record.get("chat_id") == chat_id and record.get("telegram_message_id") == message_id:
                latest = record
    return latest


def load_offset() -> int | None:
    path = offset_path()
    if not path.exists():
        return None
    try:
        value = json.loads(path.read_text(encoding="utf-8")).get("next_update_id")
    except (json.JSONDecodeError, AttributeError):
        return None
    return value if isinstance(value, int) and value >= 0 else None


def save_offset(next_update_id: int) -> None:
    path = offset_path()
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(".tmp")
    temporary.write_text(json.dumps({"next_update_id": next_update_id}), encoding="utf-8")
    temporary.replace(path)


def offset_path() -> Path:
    return CANONICAL_RUNTIME_ROOT / "telegram" / "update-offset.json"


def telegram_receipts_path() -> Path:
    return CANONICAL_RUNTIME_ROOT / "telegram" / "inbound-receipts.jsonl"


def append_json_line(path: Path, record: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with append_lock(path):
        with path.open("a", encoding="utf-8") as output:
            output.write(json.dumps(record, separators=(",", ":")) + "\n")


@contextmanager
def append_lock(path: Path):
    lock_path = CANONICAL_RUNTIME_ROOT / "locks" / f"{path.stem}.append.lock"
    with acquire_lock(lock_path, f"telegram bridge cannot append to {path} because its append lock is held"):
        yield


def required_string(payload: dict[str, Any], key: str) -> str:
    value = payload.get(key)
    if not isinstance(value, str) or not value:
        raise ValueError(f"Telegram hook payload requires non-empty {key}")
    return value
