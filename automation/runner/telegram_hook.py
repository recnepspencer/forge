#!/usr/bin/env python3
from __future__ import annotations

import sys
from pathlib import Path


def main() -> int:
    sys.path.insert(0, str(Path(__file__).resolve().parent / "src"))
    from runner.telegram_bridge.cli import main as bridge_main

    return bridge_main()


if __name__ == "__main__":
    raise SystemExit(main())
