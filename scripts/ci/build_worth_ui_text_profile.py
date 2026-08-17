#!/usr/bin/env python3
"""Build the deterministic WORTH UI global text profile indexes."""

from __future__ import annotations

import argparse
import hashlib
import json
import struct
import tomllib
from pathlib import Path


REPOSITORY = Path(__file__).resolve().parents[2]
PROFILE = REPOSITORY / "workspaces/worth-ui/profiles/worth-ui-global-text-v2"
MANIFEST = PROFILE / "manifest.toml"
COVERAGE = PROFILE / "generated/font-coverage-v2.json"
FALLBACK = PROFILE / "generated/fallback-order-v2.json"


def u16(data: bytes, offset: int) -> int:
    return struct.unpack_from(">H", data, offset)[0]


def u24(data: bytes, offset: int) -> int:
    return int.from_bytes(data[offset : offset + 3], "big")


def u32(data: bytes, offset: int) -> int:
    return struct.unpack_from(">I", data, offset)[0]


def fixed(data: bytes, offset: int) -> float:
    return struct.unpack_from(">i", data, offset)[0] / 65536.0


def face_offsets(data: bytes) -> list[int]:
    if data[:4] != b"ttcf":
        return [0]
    count = u32(data, 8)
    return [u32(data, 12 + index * 4) for index in range(count)]


def table_directory(data: bytes, face_offset: int) -> dict[str, tuple[int, int]]:
    count = u16(data, face_offset + 4)
    tables: dict[str, tuple[int, int]] = {}
    for index in range(count):
        entry = face_offset + 12 + index * 16
        tag = data[entry : entry + 4].decode("latin1")
        tables[tag] = (u32(data, entry + 8), u32(data, entry + 12))
    return tables


def decode_name(platform: int, raw: bytes) -> str:
    encoding = "utf-16-be" if platform in (0, 3) else "mac_roman"
    return raw.decode(encoding, errors="replace").replace("\x00", "").strip()


def font_names(data: bytes, tables: dict[str, tuple[int, int]]) -> dict[str, str]:
    offset, _ = tables["name"]
    count, string_offset = u16(data, offset + 2), u16(data, offset + 4)
    wanted = {1: "family", 2: "subfamily", 4: "full", 6: "postscript"}
    candidates: dict[str, tuple[int, str]] = {}
    for index in range(count):
        record = offset + 6 + index * 12
        platform, language, name_id = u16(data, record), u16(data, record + 4), u16(data, record + 6)
        if name_id not in wanted:
            continue
        length, relative = u16(data, record + 8), u16(data, record + 10)
        raw = data[offset + string_offset + relative : offset + string_offset + relative + length]
        priority = 0 if platform == 3 and language in (0x409, 0) else 1 if platform == 0 else 2
        key, value = wanted[name_id], decode_name(platform, raw)
        if value and (key not in candidates or priority < candidates[key][0]):
            candidates[key] = (priority, value)
    return {key: value for key, (_, value) in candidates.items()}


def format_four_codepoints(data: bytes, offset: int) -> set[int]:
    seg_count = u16(data, offset + 6) // 2
    ends = offset + 14
    starts = ends + seg_count * 2 + 2
    deltas = starts + seg_count * 2
    range_offsets = deltas + seg_count * 2
    covered: set[int] = set()
    for index in range(seg_count):
        start, end = u16(data, starts + index * 2), u16(data, ends + index * 2)
        delta, relative = u16(data, deltas + index * 2), u16(data, range_offsets + index * 2)
        for codepoint in range(start, min(end, 0x10FFFF) + 1):
            if codepoint == 0xFFFF:
                continue
            glyph = (codepoint + delta) & 0xFFFF
            if relative:
                position = range_offsets + index * 2 + relative + (codepoint - start) * 2
                glyph = u16(data, position) if position + 2 <= len(data) else 0
                glyph = (glyph + delta) & 0xFFFF if glyph else 0
            if glyph:
                covered.add(codepoint)
    return covered


def grouped_codepoints(data: bytes, offset: int) -> set[int]:
    groups = u32(data, offset + 12)
    covered: set[int] = set()
    for index in range(groups):
        record = offset + 16 + index * 12
        start, end, glyph = u32(data, record), u32(data, record + 4), u32(data, record + 8)
        if glyph:
            covered.update(range(start, end + 1))
    return covered


def variation_sequences(data: bytes, offset: int) -> list[dict[str, object]]:
    records: list[dict[str, object]] = []
    for index in range(u32(data, offset + 6)):
        record = offset + 10 + index * 11
        selector = u24(data, record)
        defaults, nondefaults = u32(data, record + 3), u32(data, record + 7)
        default_ranges = []
        if defaults:
            base = offset + defaults
            for item in range(u32(data, base)):
                entry = base + 4 + item * 4
                start, extra = u24(data, entry), data[entry + 3]
                default_ranges.append([start, start + extra])
        mappings = []
        if nondefaults:
            base = offset + nondefaults
            mappings = [u24(data, base + 4 + item * 5) for item in range(u32(data, base))]
        records.append({"selector": selector, "default_ranges": default_ranges, "mappings": mappings})
    return records


def cmap_coverage(data: bytes, tables: dict[str, tuple[int, int]]) -> tuple[list[list[int]], list[dict[str, object]]]:
    offset, _ = tables["cmap"]
    codepoints: set[int] = set()
    variations: list[dict[str, object]] = []
    for index in range(u16(data, offset + 2)):
        record = offset + 4 + index * 8
        platform, encoding = u16(data, record), u16(data, record + 2)
        if platform != 0 and not (platform == 3 and encoding in (1, 10)):
            continue
        table = offset + u32(data, record + 4)
        kind = u16(data, table)
        if kind == 4:
            codepoints.update(format_four_codepoints(data, table))
        elif kind in (12, 13):
            codepoints.update(grouped_codepoints(data, table))
        elif kind == 14:
            variations.extend(variation_sequences(data, table))
    return compress_ranges(codepoints), variations


def compress_ranges(codepoints: set[int]) -> list[list[int]]:
    ranges: list[list[int]] = []
    for codepoint in sorted(codepoints):
        if ranges and codepoint == ranges[-1][1] + 1:
            ranges[-1][1] = codepoint
        else:
            ranges.append([codepoint, codepoint])
    return ranges


def variation_axes(data: bytes, tables: dict[str, tuple[int, int]]) -> list[dict[str, object]]:
    if "fvar" not in tables:
        return []
    offset, _ = tables["fvar"]
    data_offset, count, size = u16(data, offset + 4), u16(data, offset + 8), u16(data, offset + 10)
    axes = []
    for index in range(count):
        axis = offset + data_offset + index * size
        axes.append({"tag": data[axis : axis + 4].decode("ascii"), "min": fixed(data, axis + 4), "default": fixed(data, axis + 8), "max": fixed(data, axis + 12)})
    return axes


def face_record(path: Path, face_index: int) -> dict[str, object]:
    data = path.read_bytes()
    offsets = face_offsets(data)
    if face_index >= len(offsets):
        raise ValueError(f"{path.name} has {len(offsets)} faces, not index {face_index}")
    tables = table_directory(data, offsets[face_index])
    ranges, variations = cmap_coverage(data, tables)
    return {"names": font_names(data, tables), "axes": variation_axes(data, tables), "coverage_ranges": ranges, "variation_sequences": variations, "color_tables": sorted(tag for tag in ("CBDT", "CBLC", "COLR", "CPAL", "sbix", "SVG ") if tag in tables)}


def canonical_json(value: object) -> bytes:
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n").encode()


def build_indexes() -> tuple[bytes, bytes]:
    manifest = tomllib.loads(MANIFEST.read_text(encoding="utf-8"))
    faces = []
    fallback = []
    for face in sorted(manifest["face"], key=lambda item: item["fallback_rank"]):
        record = {"id": face["id"], "path": face["path"], "face_index": face["face_index"]}
        record.update(face_record(PROFILE / face["path"], face["face_index"]))
        faces.append(record)
        fallback.append({
            "rank": face["fallback_rank"],
            "face": face["id"],
            "scripts": face["scripts"],
            "languages": face["languages"],
            "styles": face["styles"],
            "emoji": face.get("emoji", False),
            "last_resort": face.get("last_resort", False),
        })
    corpus = PROFILE / manifest["unicode"]["emoji_test"]
    coverage = {"schema": "worth-ui-font-coverage-v2", "unicode": "17.0.0", "emoji_test_sha256": hashlib.sha256(corpus.read_bytes()).hexdigest(), "faces": faces}
    order = {"schema": "worth-ui-fallback-order-v2", "policy": "complete-cluster-first-face", "faces": fallback}
    return canonical_json(coverage), canonical_json(order)


def publish(path: Path, candidate: bytes, check: bool) -> None:
    if check:
        if not path.exists() or path.read_bytes() != candidate:
            raise SystemExit(f"generated text profile index drifted: {path}")
        return
    path.write_bytes(candidate)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    coverage, fallback = build_indexes()
    publish(COVERAGE, coverage, args.check)
    publish(FALLBACK, fallback, args.check)


if __name__ == "__main__":
    main()
