#!/usr/bin/env python3
"""Report Rust functions that deserve composition scrutiny.

Candidates are advisory unless --fail-on-candidates is supplied. Scan explicit
files/directories, a Cargo workspace root, or all dirty Rust files in Git.
"""

from __future__ import annotations

import argparse
import json
import os
import pathlib
import sys
from dataclasses import asdict
from typing import Sequence

from rust_function_scrutiny import (
    DEFAULT_LINE_THRESHOLD,
    DEFAULT_PARAMETER_THRESHOLD,
    FunctionCandidate,
    ScanError,
    ScrutinyPolicy,
    dirty_rust_files,
    rust_files_for_paths,
    scan_files,
)


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser(description=__doc__)
    mode = result.add_mutually_exclusive_group()
    mode.add_argument(
        "--workspace", metavar="ROOT", type=pathlib.Path, help="scan a Cargo workspace tree"
    )
    mode.add_argument(
        "--dirty",
        nargs="?",
        const=pathlib.Path.cwd(),
        type=pathlib.Path,
        metavar="ROOT",
        help="scan staged, unstaged, and untracked Rust files in a Git worktree",
    )
    result.add_argument("targets", nargs="*", type=pathlib.Path, help="Rust files or folders")
    result.add_argument("--line-threshold", type=int, default=DEFAULT_LINE_THRESHOLD)
    result.add_argument(
        "--parameter-threshold", type=int, default=DEFAULT_PARAMETER_THRESHOLD
    )
    result.add_argument("--exclude", action="append", default=[], metavar="GLOB")
    result.add_argument("--relative-to", type=pathlib.Path)
    result.add_argument("--format", choices=("text", "json"), default="text")
    result.add_argument("--fail-on-candidates", action="store_true")
    return result


def resolve_mode(arguments: argparse.Namespace) -> tuple[pathlib.Path, list[pathlib.Path]]:
    if arguments.dirty is not None:
        return dirty_rust_files(arguments.dirty)
    if arguments.workspace is not None:
        root = arguments.workspace.resolve()
        if not (root / "Cargo.toml").is_file():
            raise RuntimeError(f"{root} does not contain Cargo.toml")
        return root, rust_files_for_paths([root])
    targets = arguments.targets or [pathlib.Path.cwd()]
    resolved = [target.resolve() for target in targets]
    missing = [path for path in resolved if not path.exists()]
    if missing:
        raise RuntimeError(f"target does not exist: {missing[0]}")
    common = pathlib.Path(
        os.path.commonpath(
            [str(path if path.is_dir() else path.parent) for path in resolved]
        )
    )
    return common, rust_files_for_paths(resolved)


def emit(
    candidates: Sequence[FunctionCandidate],
    errors: Sequence[ScanError],
    scanned: int,
    arguments: argparse.Namespace,
) -> None:
    if arguments.format == "json":
        print(
            json.dumps(
                {
                    "line_threshold": arguments.line_threshold,
                    "parameter_threshold": arguments.parameter_threshold,
                    "scanned_file_count": scanned,
                    "candidate_count": len(candidates),
                    "candidates": [asdict(candidate) for candidate in candidates],
                    "errors": [asdict(error) for error in errors],
                },
                indent=2,
            )
        )
        return
    for candidate in candidates:
        reasons = ", ".join(candidate.reasons)
        print(
            f"{candidate.path}:{candidate.start_line}: {candidate.name}: "
            f"{candidate.line_count} lines, {candidate.parameter_count} parameters [{reasons}]"
        )
    for error in errors:
        print(f"ERROR {error.path}:{error.line}: {error.detail}", file=sys.stderr)
    print(
        f"scrutinized {scanned} Rust files; {len(candidates)} candidate functions; "
        f"{len(errors)} scan errors"
    )


def main(argv: Sequence[str] | None = None) -> int:
    argument_parser = parser()
    arguments = argument_parser.parse_args(argv)
    if arguments.targets and (arguments.workspace is not None or arguments.dirty is not None):
        argument_parser.error("targets cannot be combined with --workspace or --dirty")
    if arguments.line_threshold < 1 or arguments.parameter_threshold < 1:
        argument_parser.error("thresholds must be positive")
    try:
        root, files = resolve_mode(arguments)
        display_root = arguments.relative_to.resolve() if arguments.relative_to else root
        policy = ScrutinyPolicy(
            line_threshold=arguments.line_threshold,
            parameter_threshold=arguments.parameter_threshold,
            exclusions=tuple(arguments.exclude),
        )
        candidates, errors, scanned = scan_files(files, display_root, policy)
    except RuntimeError as error:
        print(f"function scrutiny failed: {error}", file=sys.stderr)
        return 2
    emit(candidates, errors, scanned, arguments)
    if errors:
        return 2
    if candidates and arguments.fail_on_candidates:
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
