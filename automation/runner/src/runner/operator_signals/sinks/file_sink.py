from __future__ import annotations

import json
from typing import Any

from runner.authority.run_identity import RuntimePaths


def deliver_file(paths: RuntimePaths, payload: dict[str, Any]) -> dict[str, Any]:
    paths.notifications.parent.mkdir(parents=True, exist_ok=True)
    with paths.notifications.open("a", encoding="utf-8") as output:
        output.write(json.dumps(payload, separators=(",", ":")) + "\n")
    return {"sink": "file", "delivered": True}
