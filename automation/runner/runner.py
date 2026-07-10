#!/usr/bin/env python3
from __future__ import annotations

import sys
from pathlib import Path


def main() -> int:
    root = Path(__file__).resolve().parent
    sys.path.insert(0, str(root / "src"))
    current = sys.modules.get("runner")
    if current is not None and not hasattr(current, "__path__"):
        del sys.modules["runner"]
    from runner.facade.cli import main as facade_main

    return facade_main()


if __name__ == "__main__":
    raise SystemExit(main())
