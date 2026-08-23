from __future__ import annotations

import csv
import hashlib
import io
import os
import tempfile
from pathlib import Path

from worth_ui_3141_p6_contracts import P6_COUNTERS, P6_FAULT_BOUNDARIES, P6_MUTATIONS, P6_REQUIREMENTS, p6_construction_cost, p6_execution_cost
from worth_ui_3141_proof_plan import prepare_claim, proofs


LEDGER = Path("_docs/worth-ui/milestone-3.14.1-proof-ledger.csv")
FONT_DIGEST = "6f140249866e6815e9284fe1c8c959a8bb1b8cab252cfbe8c7c397f9a7eb9b01"
CONTRACTS = {
    "P6-PREDECESSOR-01": (
        "worth-ui-certification",
        "current Phase 1-5 source handoff",
        "phase-six-ledger-world",
        "operational-revalidation",
        "worth_ui_certification::phase_six_ledger",
    ),
    "P6-INPUT-AFFINITY-01": (
        "worth-ui-host-native",
        "native last-completed-presentation affinity and bounded retention",
        "native-lifecycle-protocol-world",
        "runtime-model",
        "worth_ui_host_native::native::input::observation",
    ),
    "P6-IME-01": (
        "worth-ui-host-native",
        "native IME composition phases and canonical ranges",
        "native-lifecycle-protocol-world",
        "ime-conformance",
        "worth_ui_host_native::native::input::ime",
    ),
    "P6-POINTER-TIME-01": (
        "worth-ui-host-native",
        "event-time pointer position witness",
        "windows-native-boundary-world",
        "event-time-input",
        "worth_ui_host_native::native::input::windows",
    ),
    "P6-PROFILE-ORDER-01": (
        "worth-ui-host-native",
        "event-time profile and resize ordering",
        "native-lifecycle-protocol-world",
        "runtime-model",
        "worth_ui_host_native::native::input::observation",
    ),
    "P6-READINESS-01": (
        "worth-ui-host-native",
        "retained-observation readiness delivery",
        "native-lifecycle-protocol-world",
        "lifecycle-model",
        "worth_ui_host_native::native::readiness",
    ),
    "P6-SETTLEMENT-01": (
        "worth-ui-runtime",
        "typed native observation ingress settlement",
        "native-lifecycle-protocol-world",
        "integration-model",
        "worth_ui_runtime::facade::entry::native_observation_settlement",
    ),
    "P6-PROTOCOL-WORLD-01": (
        "worth-ui-certification",
        "exhaustive native lifecycle schedule",
        "native-lifecycle-protocol-world",
        "exhaustive-oracle",
        "worth_ui_certification::application_contracts::phase6_native_lifecycle",
    ),
    "P6-WINDOWS-WORLD-01": (
        "worth-ui-certification",
        "serialized Windows native input boundary",
        "windows-native-boundary-world",
        "external-world",
        "worth_ui_platform_pulse::native_phase6",
    ),
    "P6-CLOSE-01": (
        "worth-ui-certification",
        "phase six final source closure",
        "phase-six-ledger-world",
        "ledger-closure",
        "worth_ui_certification::phase_six_ledger",
    ),
}


def open_row(requirement: str) -> dict[str, str]:
    owner, boundary, world, proof_kind, authority = CONTRACTS[requirement]
    family, case = P6_MUTATIONS[requirement]
    counter, _amount = P6_COUNTERS[requirement]
    artifact = f"_docs/worth-ui/milestone-3.14.1-evidence/{requirement.lower()}.json"
    return {
        "phase": "6",
        "requirement": requirement,
        "owner": owner,
        "production_boundary": boundary,
        "world_identity": world,
        "world_version": "1",
        "proof_kind": proof_kind,
        "evidence_schema": "worth-ui-ledger-evidence-v3",
        "baseline_digest": hashlib.sha256(f"not-applicable:{requirement}".encode()).hexdigest(),
        "scenario_delta": case,
        "generated_seed": "not-applicable",
        "authority_provenance": authority,
        "production_entry": "not-bound",
        "independent_oracle": "not-bound",
        "mutation_control": f"family={family};case={case}",
        "fault_injection_boundary": P6_FAULT_BOUNDARIES[requirement],
        "retained_failure_artifact": artifact,
        "teardown_result": "not-applicable",
        "construction_cost": p6_construction_cost(requirement),
        "execution_cost": p6_execution_cost(requirement),
        "exact_command": "not-bound",
        "matched_test_count": "0",
        "command_result": "not-run",
        "retained_result_artifact": artifact,
        "source_revision": "not-bound",
        "source_digest": "not-bound",
        "source_state_digest": "not-bound",
        "run_nonce": "not-bound",
        "source_identity": "not-bound",
        "font_profile_identity": "worth-ui-body-default-v1",
        "font_profile_digest": FONT_DIGEST,
        "native_profile_identity": "worth-ui-windows-dx12-v1",
        "native_profile_digest": "not-bound",
        "platform_versions": "not-bound",
        "structural_counters": f"{counter}=open",
        "presented_source_readback": "not-applicable",
        "client_area_observation": "not-applicable",
        "result": "OPEN",
        "reopen_lineage": "none",
        "final_source": "false",
        "result_artifact_digest": "not-bound",
    }


def serialized_rows(fields: list[str], requirements: list[str]) -> bytes:
    stream = io.StringIO(newline="")
    writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
    rows = []
    all_proofs = proofs()
    for requirement in requirements:
        row = open_row(requirement)
        prepare_claim(row, all_proofs[requirement])
        rows.append(row)
    writer.writerows(rows)
    return stream.getvalue().encode("utf-8")


def refresh_open_phase_six_contracts(candidate: bytes, fields: list[str]) -> bytes:
    lines = candidate.splitlines(keepends=True)
    requirement_index = fields.index("requirement")
    phase_index = fields.index("phase")
    result_index = fields.index("result")
    all_proofs = proofs()
    for index, line in enumerate(lines[1:], 1):
        record = next(csv.reader([line.decode("utf-8")]))
        requirement = record[requirement_index]
        if (
            record[phase_index] != "6"
            or record[result_index] != "OPEN"
            or requirement not in CONTRACTS
        ):
            continue
        current = dict(zip(fields, record, strict=True))
        expected = open_row(requirement)
        prepare_claim(expected, all_proofs[requirement])
        expected["reopen_lineage"] = current["reopen_lineage"]
        stream = io.StringIO(newline="")
        csv.DictWriter(stream, fieldnames=fields, lineterminator="\n").writerow(expected)
        lines[index] = stream.getvalue().encode("utf-8")
    return b"".join(lines)


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


def refresh_phase_six_profile_digest(original: bytes) -> None:
    old = b",worth-ui-global-text-v2,6f140249866e6815e9284fe1c8c959a8bb1b8cab252cfbe8c7c397f9a7eb9b01,"
    new = (
        b",worth-ui-body-default-v1,"
        + FONT_DIGEST.encode("ascii")
        + b","
    )
    lines = original.splitlines(keepends=True)
    refreshed = [
        (
            line.replace(old, new, 1).replace(b",requirements=90,", b",requirements=10,", 1)
            if line.startswith(b"6,P6-")
            else line
        )
        for line in lines
    ]
    changed = sum(before != after for before, after in zip(lines, refreshed))
    if changed == 0 or any(
        line.startswith(b"6,P6-") and new not in line for line in refreshed
    ):
        raise RuntimeError(
            "Phase 6 profile/counter refresh did not produce the canonical row posture"
        )
    atomic_replace(b"".join(refreshed))


def main() -> int:
    original = LEDGER.read_bytes()
    if os.environ.get("WORTH_UI_REFRESH_PHASE6_PROFILE_DIGEST") == "1":
        refresh_phase_six_profile_digest(original)
        print("refreshed Phase 6 text profile digests")
        return 0
    with LEDGER.open(encoding="utf-8", newline="") as stream:
        reader = csv.DictReader(stream)
        fields = list(reader.fieldnames or ())
        rows = list(reader)
    existing = {row["requirement"] for row in rows}
    missing = [requirement for requirement in P6_REQUIREMENTS if requirement not in existing]
    candidate = original
    if missing:
        close = next((row for row in rows if row["requirement"] == "P5-CLOSE-01"), None)
        if close is None or rows[-1]["requirement"] != "P5-CLOSE-01":
            raise RuntimeError(
                "Phase 6 rows require the existing Phase 5 close sentinel to be final"
            )
        if close["result"] != "PROVED" or close["final_source"] != "true":
            raise RuntimeError("cannot append Phase 6 rows without a certified Phase 5 sentinel")
        separator = b"" if original.endswith((b"\n", b"\r")) else b"\n"
        candidate += separator + serialized_rows(fields, missing)
    candidate = refresh_open_phase_six_contracts(candidate, fields)
    if candidate == original:
        print("Phase 6 rows and OPEN contracts already current")
        return 0
    atomic_replace(candidate)
    print(f"appended {len(missing)} Phase 6 OPEN rows and refreshed OPEN contracts")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
