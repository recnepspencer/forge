from __future__ import annotations

from dataclasses import dataclass
import os
from pathlib import Path


@dataclass(frozen=True)
class TelegramSettings:
    bot_token: str
    chat_id: str


def load_settings(env_file: Path | None = None) -> TelegramSettings:
    values = dotenv_values(env_file or default_env_file())
    token = os.environ.get("RUNNER_TELEGRAM_BOT_TOKEN") or values.get("RUNNER_TELEGRAM_BOT_TOKEN")
    chat_id = os.environ.get("RUNNER_TELEGRAM_CHAT_ID") or values.get("RUNNER_TELEGRAM_CHAT_ID")
    if not token or not chat_id:
        raise ValueError("RUNNER_TELEGRAM_BOT_TOKEN and RUNNER_TELEGRAM_CHAT_ID are required")
    return TelegramSettings(bot_token=token, chat_id=chat_id)


def default_env_file() -> Path:
    return Path(__file__).resolve().parents[3] / ".env"


def dotenv_values(path: Path) -> dict[str, str]:
    if not path.exists():
        return {}
    values: dict[str, str] = {}
    for raw_line in path.read_text(encoding="utf-8").splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key.strip()] = value.strip().strip('"').strip("'")
    return values
