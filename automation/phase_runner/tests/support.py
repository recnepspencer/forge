from __future__ import annotations

import json
import shutil
import sys
import tempfile
from pathlib import Path

RUNNER_DIR = Path(__file__).resolve().parents[1]
if str(RUNNER_DIR) not in sys.path:
    sys.path.insert(0, str(RUNNER_DIR))

STATE_FIXTURE = (
    RUNNER_DIR
    / "worth-touched-graph-milestone-13-aspect-routed-conflict-independence-and-batch-admission.json"
)


def make_temp_state_copy() -> Path:
    temp_root = Path(tempfile.mkdtemp(prefix="phase-runner-tests-"))
    runner_root = temp_root / "automation" / "phase_runner"
    shutil.copytree(RUNNER_DIR / "templates", runner_root / "templates")
    target = runner_root / STATE_FIXTURE.name
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(STATE_FIXTURE.read_text(encoding="utf-8"), encoding="utf-8")
    return target


def load_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))
