from __future__ import annotations

import json
from pathlib import Path
from typing import Any


def load_config(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8-sig") as config_file:
        config = json.load(config_file)
    config["_config_path"] = str(path.resolve())
    return config
