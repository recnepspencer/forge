from __future__ import annotations

import json
from typing import Any
from urllib import request

from runner.telegram_bridge.settings import TelegramSettings


def telegram_call(settings: TelegramSettings, method: str, payload: dict[str, Any]) -> Any:
    body = json.dumps(payload).encode("utf-8")
    endpoint = f"https://api.telegram.org/bot{settings.bot_token}/{method}"
    http_request = request.Request(endpoint, data=body, headers={"Content-Type": "application/json"})
    with request.urlopen(http_request, timeout=30) as response:
        decoded = json.loads(response.read().decode("utf-8"))
    if not decoded.get("ok"):
        raise ValueError(f"Telegram {method} failed")
    return decoded.get("result")


def send_message(settings: TelegramSettings, text: str) -> dict[str, Any]:
    result = telegram_call(settings, "sendMessage", {"chat_id": settings.chat_id, "text": text})
    if not isinstance(result, dict):
        raise ValueError("Telegram sendMessage returned no object result")
    return result


def get_updates(settings: TelegramSettings, offset: int | None) -> list[dict[str, Any]]:
    payload: dict[str, Any] = {"timeout": 0, "allowed_updates": ["message"]}
    if offset is not None:
        payload["offset"] = offset
    result = telegram_call(settings, "getUpdates", payload)
    return result if isinstance(result, list) else []
