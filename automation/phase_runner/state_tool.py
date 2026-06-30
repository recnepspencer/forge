#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

from phase_update import PhaseUpdateError, apply_phase_update, parse_phase_update
from state import load_state, public_state, save_state
from validation import validate_state


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Phase runner state tool.")
    subparsers = parser.add_subparsers(dest="command", required=True)

    show = subparsers.add_parser("show-current")
    show.add_argument("state_file", type=Path)

    apply = subparsers.add_parser("apply")
    apply.add_argument("state_file", type=Path)
    apply.add_argument(
        "payload_file",
        help="JSON file path or '-' to read the phase update payload from stdin",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.command == "show-current":
        state = load_state(args.state_file)
        print(json.dumps(public_state({"current": state.get("current")}), indent=2))
        return 0
    if args.command == "apply":
        return apply_command(args.state_file, args.payload_file)
    return 2


def apply_command(state_path: Path, payload_file: str) -> int:
    state = load_state(state_path)
    payload = read_payload(payload_file)
    try:
        update = parse_phase_update(payload)
        apply_phase_update(state, update)
        errors = validate_state(state, state_path)
    except PhaseUpdateError as error:
        print(f"phase update error: {error}")
        return 2
    if errors:
        for error in errors:
            print(f"validation error: {error}")
        return 2
    save_state(state_path, state)
    print("phase update applied")
    return 0


def read_payload(payload_file: str) -> dict:
    if payload_file == "-":
        return json.loads(sys.stdin.read())
    return json.loads(Path(payload_file).read_text(encoding="utf-8"))


if __name__ == "__main__":
    raise SystemExit(main())
