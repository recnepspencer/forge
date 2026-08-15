from __future__ import annotations

import csv
import hashlib
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
        "separate bounded alpha and RGBA atlas lifecycle",
        "qualified-text-world",
        "atlas-lifecycle",
        "worth_ui_host_native::atlas",
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
        "worth-ui-text",
        "ordinary versus reconstructive text raster cost",
        "qualified-text-world",
        "slope-model",
        "worth_ui_text::raster::cost",
    ),
    "P5-CLOSE-01": (
        "worth-ui-certification",
        "phase five final source closure",
        "phase-five-ledger-world",
        "ledger-closure",
        "worth_ui_certification::phase_five_ledger",
    ),
}


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


def main() -> int:
    original = LEDGER.read_bytes()
    with LEDGER.open(encoding="utf-8", newline="") as stream:
        reader = csv.DictReader(stream)
        fields = list(reader.fieldnames or ())
        existing = {row["requirement"] for row in reader}
    missing = [requirement for requirement in P5_REQUIREMENTS if requirement not in existing]
    if not missing:
        print("Phase 5 rows already present")
        return 0
    with LEDGER.open("a", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
        writer.writerows(open_row(requirement) for requirement in missing)
    updated = LEDGER.read_bytes()
    if not updated.startswith(original):
        raise RuntimeError("append rewrote historical ledger bytes")
    print(f"appended {len(missing)} Phase 5 OPEN rows")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
