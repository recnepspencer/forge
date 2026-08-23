from __future__ import annotations

import hashlib
import json
import os
import tempfile
import time
from pathlib import Path
from typing import Mapping

from worth_ui_ledger_artifact_identity import ArtifactIdentity


def publish_json_artifact(
    root: Path, identity: ArtifactIdentity, payload: Mapping[str, object]
) -> str:
    identity.validate_json_payload(payload)
    destination = identity.destination(root)
    destination.parent.mkdir(parents=True, exist_ok=True)
    content = (json.dumps(payload, indent=2) + "\n").encode("utf-8")
    replace_bytes_with_retry(destination, content)
    return hashlib.sha256(content).hexdigest()


def replace_bytes_with_retry(destination: Path, content: bytes) -> None:
    descriptor, temporary_name = tempfile.mkstemp(
        prefix=f".{destination.name}.", dir=destination.parent
    )
    temporary = Path(temporary_name)
    try:
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(content)
        for attempt in range(20):
            try:
                os.replace(temporary, destination)
                return
            except OSError as error:
                retryable = error.errno in {5, 13, 22} or getattr(
                    error, "winerror", None
                ) in {5, 22, 32, 87}
                if not retryable or attempt == 19:
                    raise
                time.sleep(0.05 * (attempt + 1))
    finally:
        temporary.unlink(missing_ok=True)
