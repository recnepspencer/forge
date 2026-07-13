from __future__ import annotations

import argparse
import json
import sys
import time
from typing import Any

from runner.telegram_bridge.routing import load_offset, record_alert, route_reply, save_offset
from runner.telegram_bridge.settings import load_settings
from runner.telegram_bridge.single_consumer import acquire_telegram_update_consumer
from runner.telegram_bridge.transport import get_updates, send_message


def main() -> int:
    parser = argparse.ArgumentParser(description="Runner Telegram command-hook adapter")
    parser.add_argument("command", choices=("send", "poll-once", "poll"))
    args = parser.parse_args()
    settings = load_settings()
    if args.command == "send":
        payload = load_signal_payload(sys.stdin.read())
        response = send_message(settings, format_signal(payload))
        message_id = response.get("message_id")
        if not isinstance(message_id, int):
            raise ValueError("Telegram sendMessage response omitted message_id")
        record_alert(payload, message_id, settings.chat_id)
        return 0
    if args.command == "poll-once":
        with acquire_telegram_update_consumer():
            return poll_once(settings.chat_id, settings)
    with acquire_telegram_update_consumer():
        while True:
            try:
                poll_once(settings.chat_id, settings)
            except Exception as error:
                write_poller_health(False, str(error))
            else:
                write_poller_health(True, None)
            time.sleep(5)


def poll_once(chat_id: str, settings) -> int:
    highest_update_id = load_offset()
    for update in get_updates(settings, highest_update_id):
        update_id = update.get("update_id")
        if isinstance(update_id, int):
            highest_update_id = max(highest_update_id or 0, update_id + 1)
        disposition = route_reply(update, chat_id)
        acknowledge_reply(settings, update, disposition)
    if highest_update_id is not None:
        save_offset(highest_update_id)
    return 0


def acknowledge_reply(settings, update: dict[str, Any], disposition) -> None:
    message = update.get("message")
    if not isinstance(message, dict) or disposition.status == "rejected_wrong_chat":
        return
    try:
        send_message(settings, f"Runner reply {disposition.status}: {disposition.detail}")
    except Exception:
        # Receipt is already durable; acknowledgement transport is derived.
        pass


def write_poller_health(healthy: bool, error: str | None) -> None:
    from runner.authority.run_identity import CANONICAL_RUNTIME_ROOT
    path = CANONICAL_RUNTIME_ROOT / "telegram" / "poller-health.json"
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps({"healthy": healthy, "error": error, "at": time.time()}), encoding="utf-8")


def load_signal_payload(raw: str) -> dict[str, Any]:
    try:
        payload = json.loads(raw)
    except json.JSONDecodeError as error:
        raise ValueError("Telegram hook stdin must be one JSON signal payload") from error
    if not isinstance(payload, dict):
        raise ValueError("Telegram hook stdin must be a JSON object")
    return payload


def format_signal(payload: dict[str, Any]) -> str:
    phase = payload.get("phase_id")
    turn = payload.get("turn")
    location = " / ".join(str(part) for part in (payload.get("project_name"), f"phase {phase}" if phase is not None else None, turn) if part)
    return "\n".join((
        f"[{payload.get('signal_kind', 'signal').upper()}] {payload.get('summary', 'Runner needs attention')}",
        location,
        f"Run: {payload.get('run_id')}  Signal: {payload.get('signal_id')}",
        "Reply directly to this message with instructions for this run.",
    ))
