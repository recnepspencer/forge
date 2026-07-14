from __future__ import annotations

import uuid
from datetime import datetime, timezone


def new_run_id() -> str:
    return uuid.uuid4().hex[:12]


def now_iso() -> str:
    return datetime.now(timezone.utc).isoformat()
