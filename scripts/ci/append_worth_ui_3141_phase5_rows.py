from __future__ import annotations

import csv
import hashlib
import io
import os
import tempfile
from pathlib import Path

from worth_ui_3141_ledger_contracts import TEXT_PLATFORM_VERSIONS
from worth_ui_3141_p5_contracts import (
    P5_FAULT_BOUNDARIES,
    P5_MUTATIONS,
    P5_REQUIREMENTS,
    P5_COUNTERS,
    p5_construction_cost,
    p5_execution_cost,
)


LEDGER = Path("_docs/worth-ui/milestone-3.14.1-proof-ledger.csv")
FONT_DIGEST = "cec6005c5baef6d69ada9c30c02ced25b0f253f80c012784fe925e307935c3f2"
NATIVE_DIGEST = "1c937a22f42660267480a055e48256b25decf0c4cd5d4d7b493e5df034c6c65b"
CONTRACTS = {
    "P5-PREDECESSOR-01": (
        "worth-ui-certification",
        "current Phase 1-4 source handoff",
        "phase-five-ledger-world",
        "operational-revalidation",
        "worth_ui_certification::phase_five_ledger",
    ),
    "P5-GLYPH-RASTER-01": (
        "worth-ui-text",
        "typed alpha and color glyph raster batches",
        "qualified-text-world",
        "raster-oracle",
        "worth_ui_text::raster",
    ),
    "P5-COLOR-EMOJI-01": (
        "worth-ui-text",
        "intrinsic color emoji raster without cluster split",
        "qualified-text-world",
        "emoji-conformance",
        "worth_ui_text::raster::color",
    ),
    "P5-ATLAS-01": (
        "worth-ui-host-native",
        "bounded alpha/color atlas and physical Signal lifecycle",
        "qualified-text-world",
        "atlas-lifecycle",
        "worth_ui_host_native::native::physical_work_signal",
    ),
    "P5-ATLAS-PINNING-01": (
        "worth-ui-host-native",
        "live-layout atlas pinning and deterministic eviction",
        "qualified-text-world",
        "atlas-pinning",
        "worth_ui_host_native::atlas::pinning",
    ),
    "P5-TEXT-DPI-01": (
        "worth-ui-text",
        "pure DPI raster replacement without relayout",
        "qualified-text-world",
        "dpi-replacement",
        "worth_ui_text::raster::dpi",
    ),
    "P5-TEXT-SPAN-PAINT-01": (
        "worth-ui-runtime",
        "paint-span identity and logical foreground RGBA",
        "qualified-text-world",
        "paint-span-oracle",
        "worth_ui_runtime::mounting::text_paint",
    ),
    "P5-TEXT-PIXELS-01": (
        "worth-ui-host-native",
        "native and headless paint-span pixel identity",
        "qualified-text-world",
        "pixel-identity",
        "worth_ui_host_native::presentation::text",
    ),
    "P5-TEXT-RECONSTRUCTION-01": (
        "worth-ui-runtime",
        "layout raster and atlas reconstruction from mounted authority",
        "qualified-text-world",
        "reconstruction",
        "worth_ui_runtime::mounting::text",
    ),
    "P5-TEXT-COST-01": (
        "worth-ui-certification",
        "realized semantic and physical Signal UI frontier cost",
        "qualified-text-world",
        "slope-model",
        "worth_ui_certification::phase_five::text_frontier_cost",
    ),
    "P5-TEXT-ASYNC-PRESENTATION-01": (
        "worth-ui-query-binding",
        "Query-owned native text presentation async result",
        "qualified-text-world",
        "async-presentation-lifecycle",
        "worth_ui_query_binding::presentation_async",
    ),
    "P5-CLOSE-01": (
        "worth-ui-certification",
        "phase five final source closure",
        "phase-five-ledger-world",
        "ledger-closure",
        "worth_ui_certification::phase_five_ledger",
    ),
}

CONTRACT_FIELDS = (
    "owner",
    "production_boundary",
    "world_identity",
    "world_version",
    "proof_kind",
    "evidence_schema",
    "scenario_delta",
    "generated_seed",
    "authority_provenance",
    "mutation_control",
    "fault_injection_boundary",
    "construction_cost",
    "execution_cost",
    "structural_counters",
)

INVALIDATED_PROOF_ROWS = {
    "P5-ATLAS-01",
    "P5-ATLAS-PINNING-01",
    "P5-TEXT-ASYNC-PRESENTATION-01",
}
EXECUTION_BINDING_FIELDS = (
    "production_entry",
    "independent_oracle",
    "exact_command",
    "matched_test_count",
    "command_result",
    "source_revision",
    "source_digest",
    "source_state_digest",
    "run_nonce",
    "source_identity",
    "result_artifact_digest",
)


def open_row(requirement: str) -> dict[str, str]:
    owner, boundary, world, proof, authority = CONTRACTS[requirement]
    family, case = P5_MUTATIONS[requirement]
    counter, _amount = P5_COUNTERS[requirement]
    artifact = f"_docs/worth-ui/milestone-3.14.1-evidence/{requirement.lower()}.json"
    return {
        "phase": "5",
        "requirement": requirement,
        "owner": owner,
        "production_boundary": boundary,
        "world_identity": world,
        "world_version": "1",
        "proof_kind": proof,
        "evidence_schema": "worth-ui-ledger-evidence-v3",
        "baseline_digest": hashlib.sha256(
            f"not-applicable:{requirement}".encode()
        ).hexdigest(),
        "scenario_delta": case,
        "generated_seed": "not-applicable",
        "authority_provenance": authority,
        "production_entry": "not-bound",
        "independent_oracle": "not-bound",
        "mutation_control": f"family={family};case={case}",
        "fault_injection_boundary": P5_FAULT_BOUNDARIES[requirement],
        "retained_failure_artifact": artifact,
        "teardown_result": "not-applicable",
        "construction_cost": p5_construction_cost(requirement),
        "execution_cost": p5_execution_cost(requirement),
        "exact_command": "not-bound",
        "matched_test_count": "0",
        "command_result": "not-run",
        "retained_result_artifact": artifact,
        "source_revision": "not-bound",
        "source_digest": "not-bound",
        "source_state_digest": "not-bound",
        "run_nonce": "not-bound",
        "source_identity": "not-bound",
        "font_profile_identity": "worth-ui-global-text-v2",
        "font_profile_digest": FONT_DIGEST,
        "native_profile_identity": "worth-ui-windows-dx12-v1",
        "native_profile_digest": NATIVE_DIGEST,
        "platform_versions": TEXT_PLATFORM_VERSIONS,
        "structural_counters": f"{counter}=open",
        "presented_source_readback": "not-applicable",
        "client_area_observation": "not-applicable",
        "result": "OPEN",
        "reopen_lineage": "none",
        "final_source": "false",
        "result_artifact_digest": "not-bound",
    }


def serialized_open_rows(fields: list[str], requirements: list[str]) -> bytes:
    stream = io.StringIO(newline="")
    writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
    writer.writerows(open_row(requirement) for requirement in requirements)
    return stream.getvalue().encode("utf-8")


def close_insertion_point(original: bytes) -> int:
    close_line = next(
        line for line in original.splitlines(keepends=True) if b",P5-CLOSE-01," in line
    )
    return original.index(close_line)


def candidate_bytes(
    original: bytes,
    fields: list[str],
    missing: list[str],
    close_present: bool,
) -> bytes:
    ordered = [item for item in missing if item != "P5-CLOSE-01"]
    if not close_present and "P5-CLOSE-01" in missing:
        ordered.append("P5-CLOSE-01")
    inserted = serialized_open_rows(fields, ordered)
    if not close_present:
        separator = b"" if original.endswith((b"\n", b"\r")) else b"\n"
        return original + separator + inserted
    offset = close_insertion_point(original)
    return original[:offset] + inserted + original[offset:]


def refresh_open_phase_five_contracts(
    candidate: bytes, fields: list[str]
) -> bytes:
    lines = candidate.splitlines(keepends=True)
    requirement_index = fields.index("requirement")
    phase_index = fields.index("phase")
    result_index = fields.index("result")
    for index, line in enumerate(lines[1:], 1):
        record = next(csv.reader([line.decode("utf-8")]))
        requirement = record[requirement_index]
        if (
            record[phase_index] != "5"
            or record[result_index] != "OPEN"
            or requirement not in CONTRACTS
        ):
            continue
        current = dict(zip(fields, record, strict=True))
        expected = open_row(requirement)
        if (
            current["exact_command"] != "not-bound"
            and requirement not in INVALIDATED_PROOF_ROWS
        ):
            counter, amount = P5_COUNTERS[requirement]
            expected["structural_counters"] = f"{counter}={amount}"
        for field in CONTRACT_FIELDS:
            current[field] = expected[field]
        if requirement in INVALIDATED_PROOF_ROWS:
            for field in EXECUTION_BINDING_FIELDS:
                current[field] = expected[field]
            current["structural_counters"] = expected["structural_counters"]
        stream = io.StringIO(newline="")
        csv.DictWriter(stream, fieldnames=fields, lineterminator="\n").writerow(current)
        lines[index] = stream.getvalue().encode("utf-8")
    return b"".join(lines)


def validate_candidate(
    original: bytes,
    candidate: bytes,
    close_present: bool,
) -> None:
    first_phase_five = next(
        (original.index(line) for line in original.splitlines(keepends=True) if b"5,P5-" in line),
        len(original),
    )
    preserved = original[:first_phase_five]
    if not candidate.startswith(preserved):
        raise RuntimeError("append candidate rewrote historical ledger bytes")
    reader = csv.DictReader(io.StringIO(candidate.decode("utf-8"), newline=""))
    requirements = [row["requirement"] for row in reader]
    if len(requirements) != len(set(requirements)):
        raise RuntimeError("append candidate contains duplicate requirements")
    if requirements[-1] != "P5-CLOSE-01":
        raise RuntimeError("Phase 5 close sentinel is not the final candidate row")
    missing = set(P5_REQUIREMENTS).difference(requirements)
    if missing:
        raise RuntimeError(f"append candidate omits Phase 5 rows: {sorted(missing)}")
    rows = {
        row["requirement"]: row
        for row in csv.DictReader(io.StringIO(candidate.decode("utf-8"), newline=""))
    }
    for requirement in P5_REQUIREMENTS:
        row = rows[requirement]
        if row["result"] != "OPEN":
            continue
        expected = open_row(requirement)
        if (
            row["exact_command"] != "not-bound"
            and requirement not in INVALIDATED_PROOF_ROWS
        ):
            counter, amount = P5_COUNTERS[requirement]
            expected["structural_counters"] = f"{counter}={amount}"
        for field in CONTRACT_FIELDS:
            if row[field] != expected[field]:
                raise RuntimeError(
                    f"append candidate has stale {requirement} contract field {field}"
                )
        if requirement in INVALIDATED_PROOF_ROWS:
            for field in EXECUTION_BINDING_FIELDS:
                if row[field] != expected[field]:
                    raise RuntimeError(
                        f"append candidate retains stale {requirement} execution field {field}"
                    )


def atomic_replace(candidate: bytes) -> None:
    descriptor, temporary_name = tempfile.mkstemp(
        prefix=f".{LEDGER.name}.", suffix=".tmp", dir=LEDGER.parent
    )
    temporary = Path(temporary_name)
    try:
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(candidate)
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(temporary, LEDGER)
    finally:
        temporary.unlink(missing_ok=True)


def main() -> int:
    original = LEDGER.read_bytes()
    with LEDGER.open(encoding="utf-8", newline="") as stream:
        reader = csv.DictReader(stream)
        fields = list(reader.fieldnames or ())
        rows = list(reader)
    existing = {row["requirement"] for row in rows}
    missing = [requirement for requirement in P5_REQUIREMENTS if requirement not in existing]
    close = next((row for row in rows if row["requirement"] == "P5-CLOSE-01"), None)
    if close is not None:
        if close["result"] != "OPEN" or close["final_source"] != "false":
            raise RuntimeError("cannot insert a Phase 5 row before a closed close sentinel")
        if rows[-1]["requirement"] != "P5-CLOSE-01":
            raise RuntimeError("Phase 5 close sentinel is not the final ledger row")
    candidate = candidate_bytes(original, fields, missing, close is not None)
    candidate = refresh_open_phase_five_contracts(candidate, fields)
    validate_candidate(original, candidate, close is not None)
    if candidate == original:
        print("Phase 5 rows and OPEN contracts already current")
        return 0
    atomic_replace(candidate)
    print(f"appended {len(missing)} Phase 5 OPEN rows and refreshed OPEN contracts")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
