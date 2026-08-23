from __future__ import annotations

import argparse
import subprocess
from pathlib import Path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Operationally verify the Worth UI ledger")
    parser.add_argument("--through-phase", type=int, choices=(2, 3, 4, 5, 6), default=2)
    parser.add_argument(
        "--refresh-predecessor-for-phase",
        type=int,
        choices=(3, 4, 5, 6),
    )
    return parser.parse_args()


def source_revision(root: Path) -> str:
    completed = subprocess.run(
        ["git", "rev-parse", "--verify", "HEAD"],
        cwd=root,
        capture_output=True,
        text=True,
        check=False,
    )
    revision = completed.stdout.strip()
    if completed.returncode != 0 or len(revision) != 40:
        raise RuntimeError("cannot resolve operational source revision")
    return revision
