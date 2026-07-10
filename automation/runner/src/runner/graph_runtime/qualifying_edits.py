from __future__ import annotations

from fnmatch import fnmatch
from pathlib import Path
import subprocess
from typing import Iterable


def latest_qualifying_edit_timestamp(
    cwd: Path,
    includes: Iterable[str],
    excludes: Iterable[str],
) -> float | None:
    latest_mtime: float | None = None
    for path in qualifying_paths(cwd, includes, excludes):
        try:
            mtime = path.stat().st_mtime
        except OSError:
            continue
        if latest_mtime is None or mtime > latest_mtime:
            latest_mtime = mtime
    return latest_mtime


def qualifying_paths(cwd: Path, includes: Iterable[str], excludes: Iterable[str]) -> Iterable[Path]:
    seen: set[Path] = set()
    for pattern in includes:
        for path in cwd.glob(pattern):
            if path in seen or not path.is_file():
                continue
            relative_path = path.resolve().relative_to(cwd).as_posix()
            if any(fnmatch(relative_path, exclude) for exclude in excludes):
                continue
            seen.add(path)
            yield path


def qualifying_git_diff_exists(cwd: Path, includes: Iterable[str], excludes: Iterable[str]) -> bool | None:
    """Confirm a stale mtime against the configured, proof-bearing Git scope."""
    try:
        tracked = subprocess.run(
            ["git", "diff", "--name-only", "HEAD", "--"],
            cwd=cwd,
            check=False,
            capture_output=True,
            text=True,
            timeout=5,
        )
        untracked = subprocess.run(
            ["git", "ls-files", "--others", "--exclude-standard"],
            cwd=cwd,
            check=False,
            capture_output=True,
            text=True,
            timeout=5,
        )
    except (OSError, subprocess.TimeoutExpired):
        return None
    if tracked.returncode != 0 or untracked.returncode != 0:
        return None
    for name in (*tracked.stdout.splitlines(), *untracked.stdout.splitlines()):
        normalized = name.replace("\\", "/")
        if any(fnmatch(normalized, pattern) for pattern in includes) and not any(
            fnmatch(normalized, pattern) for pattern in excludes
        ):
            return True
    return False
