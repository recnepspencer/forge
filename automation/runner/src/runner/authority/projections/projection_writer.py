from __future__ import annotations

import json
from pathlib import Path


def write_projection(path: Path, projection: dict[str, object]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(projection, indent=2) + "\n", encoding="utf-8")
