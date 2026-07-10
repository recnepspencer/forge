from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path


def main() -> int:
    process = subprocess.Popen([str(resolve_cursor_agent_cmd()), *sys.argv[1:]])
    return process.wait()


def resolve_cursor_agent_cmd() -> Path:
    root = Path(os.environ.get("LOCALAPPDATA", "")) / "cursor-agent" / "versions"
    if not root.exists():
        raise FileNotFoundError(f"Cursor agent versions directory not found: {root}")
    candidates = []
    for child in root.iterdir():
        if not child.is_dir():
            continue
        cmd = child / "cursor-agent.cmd"
        node = child / "node.exe"
        index = child / "index.js"
        if cmd.exists() and node.exists() and index.exists():
            candidates.append(child)
    if not candidates:
        raise FileNotFoundError(f"No usable Cursor agent installation found in {root}")
    latest = max(candidates, key=sort_key)
    return latest / "cursor-agent.cmd"


def sort_key(path: Path) -> tuple[float, str]:
    stat = path.stat()
    return (stat.st_mtime, path.name)


if __name__ == "__main__":
    raise SystemExit(main())
