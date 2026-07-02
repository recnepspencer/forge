#!/usr/bin/env python3
from __future__ import annotations

import sys

from operator_commands import build_parser, dispatch


def main() -> int:
    if hasattr(sys.stdout, "reconfigure"):
        sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    if hasattr(sys.stderr, "reconfigure"):
        sys.stderr.reconfigure(encoding="utf-8", errors="replace")
    parser = build_parser()
    args = parser.parse_args()
    return dispatch(args)


if __name__ == "__main__":
    raise SystemExit(main())
