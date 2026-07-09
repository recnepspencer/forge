#!/usr/bin/env python3
"""Phase 3 physical-format topology restructure: move files and generate mod.rs."""

from __future__ import annotations

import os
import shutil
from pathlib import Path

SRC = Path(__file__).resolve().parents[1] / "src"

# (target_dir, [(src_filename, dst_filename), ...])
MOVES: dict[str, list[tuple[str, str]]] = {
    "format_identity": [
        ("ids.rs", "ids.rs"),
        ("format_identity.rs", "magic.rs"),
        ("vocabulary.rs", "vocabulary.rs"),
    ],
    "binary_format": [
        ("algorithm_review.rs", "algorithm_review.rs"),
        ("alignment.rs", "alignment.rs"),
        ("allocation.rs", "allocation.rs"),
        ("binary_format.rs", "declaration.rs"),
        ("binary_format_denials.rs", "denials.rs"),
        ("binary_format_tests.rs", "tests.rs"),
        ("binary_format_witness.rs", "witness.rs"),
        ("byte_order.rs", "byte_order.rs"),
        ("field_widths.rs", "field_widths.rs"),
        ("forward_compatibility.rs", "forward_compatibility.rs"),
        ("free_space_policy.rs", "free_space_policy.rs"),
        ("golden_bytes.rs", "golden_bytes.rs"),
        ("operation_complexity.rs", "operation_complexity.rs"),
        ("operation_complexity_tests.rs", "operation_complexity_tests.rs"),
        ("operation_counters.rs", "operation_counters.rs"),
        ("page_size.rs", "page_size.rs"),
        ("reserved_fields.rs", "reserved_fields.rs"),
    ],
    "header": [
        ("header_authority.rs", "authority.rs"),
        ("header_counters.rs", "counters.rs"),
        ("header_decode_tests.rs", "tests.rs"),
        ("header_denials.rs", "denials.rs"),
        ("header_kinds.rs", "kinds.rs"),
        ("header_layout.rs", "layout.rs"),
        ("header_publication.rs", "publication.rs"),
        ("header_reserved.rs", "reserved.rs"),
        ("header_witness.rs", "witness.rs"),
    ],
    "page_record": [
        ("page_record_authority.rs", "authority.rs"),
        ("page_record_counters.rs", "counters.rs"),
        ("page_record_denials.rs", "denials.rs"),
        ("page_record_test_support.rs", "test_support.rs"),
        ("page_record_tests.rs", "tests.rs"),
        ("slot_directory.rs", "slot_directory.rs"),
        ("slot_state.rs", "slot_state.rs"),
    ],
    "extent_record": [
        ("extent_record_authority.rs", "authority.rs"),
        ("extent_record_counters.rs", "counters.rs"),
        ("extent_record_denials.rs", "denials.rs"),
        ("extent_record_tests.rs", "tests.rs"),
        ("extent_membership.rs", "membership.rs"),
    ],
    "record_framing": [
        ("record_framing.rs", "framing.rs"),
    ],
    "payload": [
        ("payload_view.rs", "view.rs"),
    ],
    "reference": [
        ("references.rs", "references.rs"),
        ("reference_authority.rs", "authority.rs"),
        ("reference_counters.rs", "counters.rs"),
        ("reference_denials.rs", "denials.rs"),
        ("reference_identity_tests.rs", "tests.rs"),
        ("reference_witnesses.rs", "witnesses.rs"),
        ("physical_scope.rs", "scope.rs"),
        ("future_chunk_reference.rs", "future_chunk.rs"),
    ],
    "generation": [
        ("generation_authority.rs", "authority.rs"),
        ("generation_cells.rs", "cells.rs"),
        ("generation_owner.rs", "owner.rs"),
    ],
    "manifest": [
        ("manifest_authority.rs", "authority.rs"),
        ("manifest_counters.rs", "counters.rs"),
        ("manifest_denials.rs", "denials.rs"),
        ("manifest_entries.rs", "entries.rs"),
        ("manifest_tests.rs", "tests.rs"),
        ("manifest_universe.rs", "universe.rs"),
        ("manifests.rs", "vocabulary.rs"),
        ("reclaim_region.rs", "reclaim_region.rs"),
        ("reclaimed_byte_interpretation.rs", "reclaimed_byte_interpretation.rs"),
    ],
    "checksum": [
        ("physical_chunk_checksum.rs", "chunk_checksum.rs"),
        ("physical_chunk_checksum_denials.rs", "chunk_denials.rs"),
        ("checksum_coverage.rs", "coverage.rs"),
        ("checksum_coverage_denials.rs", "coverage_denials.rs"),
        ("checksum_coverage_fields.rs", "coverage_fields.rs"),
    ],
    "offline_verifier": [
        ("offline_manifest_codec.rs", "codec.rs"),
        ("offline_manifest_codec_decode.rs", "codec_decode.rs"),
        ("offline_manifest_codec_decode_fields.rs", "codec_decode_fields.rs"),
        ("offline_manifest_codec_encode.rs", "codec_encode.rs"),
        ("offline_persisted_layout.rs", "persisted_layout.rs"),
        ("offline_verifier.rs", "verifier.rs"),
        ("offline_verifier_counters.rs", "counters.rs"),
        ("offline_verifier_denials.rs", "denials.rs"),
        ("offline_verifier_observation.rs", "observation.rs"),
        ("offline_verifier_report.rs", "report.rs"),
        ("offline_verifier_tests.rs", "tests.rs"),
        ("runtime_layout_observation.rs", "runtime_layout.rs"),
    ],
    "security_metadata": [
        ("security_metadata_carrier.rs", "carrier.rs"),
        ("security_metadata_denials.rs", "denials.rs"),
        ("security_metadata_envelope.rs", "envelope.rs"),
        ("security_metadata_tests.rs", "tests.rs"),
        ("security_metadata_vocabulary.rs", "vocabulary.rs"),
        ("security_scope_propagation_denials.rs", "scope_propagation_denials.rs"),
    ],
    "denial": [
        ("denials.rs", "vocabulary.rs"),
        ("shortcut_boundary_denials.rs", "shortcut_boundary.rs"),
    ],
    "compile_fail": [
        ("physical_format_compile_fail.rs", "physical_format_compile_fail.rs"),
    ],
    "facade": [
        ("facade.rs", "mod.rs"),
        ("facade_append.rs", "append.rs"),
        ("facade_counters.rs", "counters.rs"),
        ("facade_denials.rs", "denials.rs"),
        ("facade_evidence.rs", "evidence.rs"),
        ("facade_locate.rs", "locate.rs"),
        ("facade_reports.rs", "reports.rs"),
        ("facade_requests.rs", "requests.rs"),
        ("facade_root_publication.rs", "root_publication.rs"),
        ("facade_storage.rs", "storage.rs"),
        ("facade_tests.rs", "tests.rs"),
    ],
}

# mod.rs content templates: list of (mod_name, is_test)
MOD_SPECS: dict[str, list[tuple[str, bool]]] = {
    "format_identity": [
        ("ids", False),
        ("magic", False),
        ("vocabulary", False),
    ],
    "binary_format": [
        ("algorithm_review", False),
        ("alignment", False),
        ("allocation", False),
        ("declaration", False),
        ("denials", False),
        ("tests", True),
        ("witness", False),
        ("byte_order", False),
        ("field_widths", False),
        ("forward_compatibility", False),
        ("free_space_policy", False),
        ("golden_bytes", False),
        ("operation_complexity", False),
        ("operation_complexity_tests", True),
        ("operation_counters", False),
        ("page_size", False),
        ("reserved_fields", False),
    ],
    "header": [
        ("authority", False),
        ("counters", False),
        ("tests", True),
        ("denials", False),
        ("kinds", False),
        ("layout", False),
        ("publication", False),
        ("reserved", False),
        ("witness", False),
    ],
    "page_record": [
        ("authority", False),
        ("counters", False),
        ("denials", False),
        ("test_support", True),
        ("tests", True),
        ("slot_directory", False),
        ("slot_state", False),
    ],
    "extent_record": [
        ("authority", False),
        ("counters", False),
        ("denials", False),
        ("tests", True),
        ("membership", False),
    ],
    "record_framing": [("framing", False)],
    "payload": [("view", False)],
    "reference": [
        ("references", False),
        ("authority", False),
        ("counters", False),
        ("denials", False),
        ("tests", True),
        ("witnesses", False),
        ("scope", False),
        ("future_chunk", False),
    ],
    "generation": [
        ("authority", False),
        ("cells", False),
        ("owner", False),
    ],
    "manifest": [
        ("authority", False),
        ("counters", False),
        ("denials", False),
        ("entries", False),
        ("tests", True),
        ("universe", False),
        ("vocabulary", False),
        ("reclaim_region", False),
        ("reclaimed_byte_interpretation", False),
    ],
    "checksum": [
        ("chunk_checksum", False),
        ("chunk_denials", False),
        ("coverage", False),
        ("coverage_denials", False),
        ("coverage_fields", False),
    ],
    "offline_verifier": [
        ("codec", False),
        ("codec_decode", False),
        ("codec_decode_fields", False),
        ("codec_encode", False),
        ("persisted_layout", False),
        ("verifier", False),
        ("counters", False),
        ("denials", False),
        ("observation", False),
        ("report", False),
        ("tests", True),
        ("runtime_layout", False),
    ],
    "security_metadata": [
        ("carrier", False),
        ("denials", False),
        ("envelope", False),
        ("tests", True),
        ("vocabulary", False),
        ("scope_propagation_denials", False),
    ],
    "denial": [
        ("vocabulary", False),
        ("shortcut_boundary", False),
    ],
    "compile_fail": [("physical_format_compile_fail", False)],
    "facade": [
        ("append", False),
        ("counters", False),
        ("denials", False),
        ("evidence", False),
        ("locate", False),
        ("reports", False),
        ("requests", False),
        ("root_publication", False),
        ("storage", False),
        ("tests", True),
    ],
}

# Path replacements inside moved files (old -> new)
PATH_REPLACEMENTS = [
    ("crate::facade_storage::", "crate::facade::storage::"),
    ("crate::facade_root_publication::", "crate::facade::root_publication::"),
    ("crate::facade_append::", "crate::facade::append::"),
    ("crate::facade_locate::", "crate::facade::locate::"),
    ("crate::alignment::", "crate::binary_format::alignment::"),
    ("crate::field_widths::", "crate::binary_format::field_widths::"),
    ("crate::binary_format::", "crate::binary_format::declaration::"),
    ("crate::golden_bytes::", "crate::binary_format::golden_bytes::"),
    ("crate::slot_directory::", "crate::page_record::slot_directory::"),
    ("crate::offline_manifest_codec::", "crate::offline_verifier::codec::"),
    ("crate::offline_manifest_codec_decode_fields::", "crate::offline_verifier::codec_decode_fields::"),
    ("crate::offline_manifest_codec_decode::", "crate::offline_verifier::codec_decode::"),
    ("crate::offline_manifest_codec_encode::", "crate::offline_verifier::codec_encode::"),
    ("crate::manifest_universe::", "crate::manifest::universe::"),
]


def move_files() -> None:
    for target_dir, files in MOVES.items():
        dest = SRC / target_dir
        dest.mkdir(parents=True, exist_ok=True)
        for src_name, dst_name in files:
            src_path = SRC / src_name
            dst_path = dest / dst_name
            if not src_path.exists():
                if dst_path.exists():
                    continue
                raise FileNotFoundError(src_path)
            if dst_path.exists():
                raise FileExistsError(dst_path)
            shutil.move(str(src_path), str(dst_path))
            print(f"moved {src_name} -> {target_dir}/{dst_name}")


def patch_file(path: Path) -> None:
    text = path.read_text(encoding="utf-8")
    original = text
    for old, new in PATH_REPLACEMENTS:
        text = text.replace(old, new)
    if text != original:
        path.write_text(text, encoding="utf-8")
        print(f"patched {path.relative_to(SRC)}")


def patch_all_rs() -> None:
    for path in SRC.rglob("*.rs"):
        patch_file(path)


def write_family_mod(dir_name: str, specs: list[tuple[str, bool]]) -> None:
    lines = []
    for mod_name, is_test in specs:
        if is_test:
            lines.append("#[cfg(test)]")
        lines.append(f"mod {mod_name};")
    # Re-export public items from non-test modules for intra-crate use
    lines.append("")
    for mod_name, is_test in specs:
        if not is_test and mod_name != "mod":
            lines.append(f"pub use {mod_name}::*;")
    mod_path = SRC / dir_name / "mod.rs"
    if dir_name == "facade":
        # facade already has mod.rs from facade.rs move; append submodule decls
        existing = mod_path.read_text(encoding="utf-8")
        if "mod append;" not in existing:
            header = "\n".join(lines[: len([s for s in specs]) * 2 + 1])  # rough
            # Insert submodule declarations after the first line block
            insert = "\n".join(
                line
                for mod_name, is_test in specs
                for line in (
                    (["#[cfg(test)]", f"mod {mod_name};"] if is_test else [f"mod {mod_name};"])
                )
            )
            # Prepend submodule mods before impl block - find a good anchor
            if "mod append;" not in existing:
                anchor = "use crate::facade::root_publication::encode_root_publication;"
                if anchor in existing:
                    existing = existing.replace(
                        anchor,
                        insert + "\n\n" + anchor,
                    )
                else:
                    existing = insert + "\n\n" + existing
            mod_path.write_text(existing, encoding="utf-8")
        return
    mod_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"wrote {dir_name}/mod.rs")


def main() -> None:
    os.chdir(SRC)
    move_files()
    patch_all_rs()
    for dir_name, specs in MOD_SPECS.items():
        if dir_name != "facade":
            write_family_mod(dir_name, specs)
    print("done")


if __name__ == "__main__":
    main()