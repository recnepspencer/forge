from __future__ import annotations

import json
import subprocess
from typing import Any


def deliver_command_hook(command: tuple[str, ...], payload: dict[str, Any]) -> dict[str, Any]:
    result = subprocess.run(command, input=json.dumps(payload), text=True, capture_output=True, check=False, timeout=30)
    return {"sink": "command_hook", "delivered": result.returncode == 0, "exit_code": result.returncode}
