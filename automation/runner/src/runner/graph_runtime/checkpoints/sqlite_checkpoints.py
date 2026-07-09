from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path

from runner.authority.run_identity import RuntimePaths


@dataclass(frozen=True)
class SqliteCheckpointSurface:
    run_id: str
    root: Path
    database_path: Path


def checkpoint_root(run_id: str) -> Path:
    return RuntimePaths(run_id).checkpoints


def checkpoint_database_path(run_id: str) -> Path:
    return checkpoint_root(run_id) / "runner.sqlite3"


def sqlite_checkpoint_surface(run_id: str) -> SqliteCheckpointSurface:
    root = checkpoint_root(run_id)
    return SqliteCheckpointSurface(
        run_id=run_id,
        root=root,
        database_path=checkpoint_database_path(run_id),
    )
