from __future__ import annotations

from typing import Any


def deliver_stdout(payload: dict[str, Any]) -> dict[str, Any]:
    print(f"[{payload['signal_kind']}] {payload['summary']}")
    return {"sink": "stdout", "delivered": True}
