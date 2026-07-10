from __future__ import annotations

import json

from runner.authority.run_identity import RuntimePaths


def load_prompt_instantiation(run_id: str, turn_instance_id: str) -> dict:
    record_path = RuntimePaths(run_id).instantiations / turn_instance_id / "record.json"
    return json.loads(record_path.read_text(encoding="utf-8"))
