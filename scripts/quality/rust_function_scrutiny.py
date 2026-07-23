"""Rust function discovery and structural-candidate analysis."""

from __future__ import annotations

import fnmatch
import os
import pathlib
import re
import subprocess
from dataclasses import dataclass
from typing import Iterable, Sequence


DEFAULT_LINE_THRESHOLD = 60
DEFAULT_PARAMETER_THRESHOLD = 5
SKIPPED_DIRECTORIES = {
    ".git",
    ".hg",
    ".svn",
    ".venv",
    "node_modules",
    "target",
    "vendor",
}
FUNCTION_PATTERN = re.compile(
    r"\bfn\s+(?P<name>r#[A-Za-z_][A-Za-z0-9_]*|[A-Za-z_][A-Za-z0-9_]*)"
)
RECEIVER_PATTERN = re.compile(
    r"^(?:&(?:'[A-Za-z_][A-Za-z0-9_]*\s*)?(?:mut\s+)?)?"
    r"(?:mut\s+)?self(?:\s*:|$)"
)


@dataclass(frozen=True)
class FunctionCandidate:
    path: str
    name: str
    start_line: int
    end_line: int
    line_count: int
    parameter_count: int
    reasons: tuple[str, ...]


@dataclass(frozen=True)
class ScanError:
    path: str
    line: int
    detail: str


@dataclass(frozen=True)
class ScrutinyPolicy:
    line_threshold: int = DEFAULT_LINE_THRESHOLD
    parameter_threshold: int = DEFAULT_PARAMETER_THRESHOLD
    exclusions: tuple[str, ...] = ()


def sanitize_rust(source: str) -> str:
    """Replace non-code literal/comment content with spaces, preserving lines."""
    output = list(source)
    index = 0
    while index < len(source):
        if source.startswith("//", index):
            end = source.find("\n", index)
            end = len(source) if end < 0 else end
            blank_non_newlines(output, index, end)
            index = end
            continue
        if source.startswith("/*", index):
            index = sanitize_block_comment(source, output, index)
            continue
        raw = raw_string_opening(source, index)
        if raw is not None:
            content_start, hashes = raw
            closing = '"' + ("#" * hashes)
            end = source.find(closing, content_start)
            end = len(source) if end < 0 else end + len(closing)
            blank_non_newlines(output, index, end)
            index = end
            continue
        string_quote = normal_string_quote(source, index)
        if string_quote is not None:
            index = sanitize_escaped_literal(source, output, index, string_quote)
            continue
        if source[index] == "'" and looks_like_character_literal(source, index):
            index = sanitize_escaped_literal(source, output, index, index)
            continue
        index += 1
    return "".join(output)


def blank_non_newlines(output: list[str], start: int, end: int) -> None:
    for index in range(start, end):
        if output[index] not in "\r\n":
            output[index] = " "


def sanitize_block_comment(source: str, output: list[str], start: int) -> int:
    depth = 1
    index = start + 2
    while index < len(source) and depth:
        if source.startswith("/*", index):
            depth += 1
            index += 2
        elif source.startswith("*/", index):
            depth -= 1
            index += 2
        else:
            index += 1
    blank_non_newlines(output, start, index)
    return index


def raw_string_opening(source: str, start: int) -> tuple[int, int] | None:
    match = re.match(r"(?:br|cr|r)(?P<hashes>#{0,255})\"", source[start:])
    if match is None:
        return None
    return start + match.end(), len(match.group("hashes"))


def normal_string_quote(source: str, start: int) -> int | None:
    for prefix in ('"', 'b"', 'c"'):
        if source.startswith(prefix, start):
            return start + len(prefix) - 1
    return None


def sanitize_escaped_literal(
    source: str, output: list[str], start: int, quote_index: int
) -> int:
    quote = source[quote_index]
    index = quote_index + 1
    escaped = False
    while index < len(source):
        character = source[index]
        if escaped:
            escaped = False
        elif character == "\\":
            escaped = True
        elif character == quote:
            index += 1
            break
        elif character == "\n" and quote == "'":
            break
        index += 1
    blank_non_newlines(output, start, index)
    return index


def looks_like_character_literal(source: str, start: int) -> bool:
    index = start + 1
    if index >= len(source) or source[index] in "\r\n'":
        return False
    index += 2 if source[index] == "\\" else 1
    return index < len(source) and source[index] == "'"


def scan_source(
    source: str,
    display_path: str,
    policy: ScrutinyPolicy = ScrutinyPolicy(),
) -> tuple[list[FunctionCandidate], list[ScanError]]:
    sanitized = sanitize_rust(source)
    candidates: list[FunctionCandidate] = []
    errors: list[ScanError] = []
    for match in FUNCTION_PATTERN.finditer(sanitized):
        start = match.start()
        start_line = line_number(source, start)
        opening = sanitized.find("(", match.end())
        if opening < 0:
            errors.append(ScanError(display_path, start_line, "missing parameter list"))
            continue
        closing = matching_delimiter(sanitized, opening, "(", ")")
        if closing is None:
            errors.append(ScanError(display_path, start_line, "unclosed parameter list"))
            continue
        terminator = signature_terminator(sanitized, closing + 1)
        if terminator is None:
            errors.append(ScanError(display_path, start_line, "missing body or semicolon"))
            continue
        if sanitized[terminator] == "{":
            end_index = matching_delimiter(sanitized, terminator, "{", "}")
            if end_index is None:
                errors.append(ScanError(display_path, start_line, "unclosed function body"))
                continue
        else:
            end_index = terminator
        end_line = line_number(source, end_index)
        line_count = end_line - start_line + 1
        parameter_count = count_explicit_parameters(sanitized[opening + 1 : closing])
        reasons = candidate_reasons(line_count, parameter_count, policy)
        if reasons:
            candidates.append(
                FunctionCandidate(
                    display_path,
                    match.group("name"),
                    start_line,
                    end_line,
                    line_count,
                    parameter_count,
                    reasons,
                )
            )
    return candidates, errors


def candidate_reasons(
    line_count: int,
    parameter_count: int,
    policy: ScrutinyPolicy,
) -> tuple[str, ...]:
    reasons = []
    if line_count > policy.line_threshold:
        reasons.append("long-function")
    if parameter_count >= policy.parameter_threshold:
        reasons.append("many-parameters")
    return tuple(reasons)


def line_number(source: str, index: int) -> int:
    return source.count("\n", 0, index) + 1


def matching_delimiter(
    source: str, opening: int, open_character: str, close_character: str
) -> int | None:
    depth = 0
    for index in range(opening, len(source)):
        character = source[index]
        if character == open_character:
            depth += 1
        elif character == close_character:
            depth -= 1
            if depth == 0:
                return index
    return None


def signature_terminator(source: str, start: int) -> int | None:
    parentheses = 0
    brackets = 0
    for index in range(start, len(source)):
        character = source[index]
        if character == "(":
            parentheses += 1
        elif character == ")":
            parentheses -= 1
        elif character == "[":
            brackets += 1
        elif character == "]":
            brackets -= 1
        elif parentheses == 0 and brackets == 0 and character in "{;":
            return index
    return None


def count_explicit_parameters(parameters: str) -> int:
    parts = split_top_level(parameters)
    return sum(1 for part in parts if part.strip() and not is_receiver(part))


def split_top_level(value: str) -> list[str]:
    parts: list[str] = []
    start = 0
    depths = {"(": 0, "[": 0, "{": 0, "<": 0}
    closing = {")": "(", "]": "[", "}": "{", ">": "<"}
    for index, character in enumerate(value):
        if character in depths:
            depths[character] += 1
        elif character in closing and depths[closing[character]] > 0:
            depths[closing[character]] -= 1
        elif character == "," and all(depth == 0 for depth in depths.values()):
            parts.append(value[start:index])
            start = index + 1
    parts.append(value[start:])
    return parts


def is_receiver(parameter: str) -> bool:
    without_attributes = re.sub(r"#\s*\[[^\]]*\]\s*", "", parameter).strip()
    return RECEIVER_PATTERN.match(without_attributes) is not None


def rust_files_for_paths(paths: Sequence[pathlib.Path]) -> list[pathlib.Path]:
    files: set[pathlib.Path] = set()
    for path in paths:
        resolved = path.resolve()
        if resolved.is_file() and resolved.suffix == ".rs":
            files.add(resolved)
        elif resolved.is_dir():
            for directory, children, names in os.walk(resolved):
                children[:] = [
                    child for child in children if child not in SKIPPED_DIRECTORIES
                ]
                for name in names:
                    if name.endswith(".rs"):
                        files.add((pathlib.Path(directory) / name).resolve())
    return sorted(files)


def dirty_rust_files(path: pathlib.Path) -> tuple[pathlib.Path, list[pathlib.Path]]:
    root = git_output(path, ["rev-parse", "--show-toplevel"]).strip()
    if not root:
        raise RuntimeError(f"{path} is not inside a Git worktree")
    worktree = pathlib.Path(root).resolve()
    names: set[str] = set()
    for arguments in (
        ["diff", "--name-only", "--diff-filter=ACMRTUXB", "-z"],
        ["diff", "--cached", "--name-only", "--diff-filter=ACMRTUXB", "-z"],
        ["ls-files", "--others", "--exclude-standard", "-z"],
    ):
        names.update(
            name
            for name in git_output(worktree, arguments).split("\0")
            if name.endswith(".rs")
        )
    files = [worktree / name for name in names if (worktree / name).is_file()]
    return worktree, sorted(path.resolve() for path in files)


def git_output(path: pathlib.Path, arguments: Sequence[str]) -> str:
    completed = subprocess.run(
        ["git", "-C", str(path), *arguments],
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        encoding="utf-8",
        errors="surrogateescape",
    )
    if completed.returncode != 0:
        raise RuntimeError(completed.stderr.strip() or "Git command failed")
    return completed.stdout


def excluded(path: pathlib.Path, root: pathlib.Path, patterns: Sequence[str]) -> bool:
    relative = display_path(path, root)
    return any(fnmatch.fnmatch(relative, pattern) for pattern in patterns)


def display_path(path: pathlib.Path, root: pathlib.Path) -> str:
    try:
        return path.resolve().relative_to(root.resolve()).as_posix()
    except ValueError:
        return path.resolve().as_posix()


def scan_files(
    files: Iterable[pathlib.Path],
    root: pathlib.Path,
    policy: ScrutinyPolicy,
) -> tuple[list[FunctionCandidate], list[ScanError], int]:
    candidates: list[FunctionCandidate] = []
    errors: list[ScanError] = []
    scanned = 0
    for path in files:
        if excluded(path, root, policy.exclusions):
            continue
        scanned += 1
        relative = display_path(path, root)
        try:
            source = path.read_text(encoding="utf-8")
        except (OSError, UnicodeError) as error:
            errors.append(ScanError(relative, 1, str(error)))
            continue
        file_candidates, file_errors = scan_source(source, relative, policy)
        candidates.extend(file_candidates)
        errors.extend(file_errors)
    candidates.sort(key=lambda item: (item.path, item.start_line, item.name))
    errors.sort(key=lambda item: (item.path, item.line, item.detail))
    return candidates, errors, scanned
