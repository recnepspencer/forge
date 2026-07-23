import csv
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest import TestCase

from worth_ui_real_boundary_proof_ledger import (
    EXTERNAL_CLAIMS,
    REQUIRED_CLAIMS,
    REQUIRED_PHASES,
    real_boundary_ledger_violations,
)


class WorthUiRealBoundaryProofLedgerTests(TestCase):
    def test_fake_receipt_only_and_fast_external_claims_are_rejected(self) -> None:
        mutations = {
            "fake filesystem": ("filesystem_source_acquisition", "proof_class", "synthetic"),
            "fake egui": ("egui_host_execution", "proof_class", "synthetic"),
            "receipt allocator": (
                "executor_allocator_observation",
                "independent_observation",
                "receipt only",
            ),
            "fast external": ("filesystem_watcher_settlement", "lane", "fast"),
        }
        for name, mutation in mutations.items():
            with self.subTest(name=name), TemporaryDirectory() as temporary:
                root = Path(temporary)
                rows, config = real_boundary_fixture(root)
                claim, field, value = mutation
                self.assertEqual(real_boundary_ledger_violations(root, config), [])
                next(row for row in rows if row["claim"] == claim)[field] = value
                write_ledger(root, config, rows)

                violations = real_boundary_ledger_violations(root, config)

                self.assertTrue(violations, f"{name} must be rejected")
                self.assertEqual(violations[0].rule, "real-boundary-ledger")
                self.assertIn(claim, "\n".join(item.detail for item in violations))

    def test_comment_cannot_promote_an_assigned_claim_to_proven(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            rows, config = real_boundary_fixture(root)
            claim = "filesystem_source_acquisition"
            row = next(row for row in rows if row["claim"] == claim)
            row["status"] = "proven"
            (root / row["module_path"]).write_text("// #[test]\n", encoding="utf-8")
            write_ledger(root, config, rows)

            violations = real_boundary_ledger_violations(root, config)

            self.assertIn("proven module contains no test", "\n".join(item.detail for item in violations))


def real_boundary_fixture(root: Path):
    rows = []
    for claim in sorted(REQUIRED_CLAIMS):
        external = claim in EXTERNAL_CLAIMS
        module_path = (
            f"workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/{claim}.rs"
            if external
            else "workspaces/worth-ui/crates/worth-ui-runtime/src/runtime/tests/mod.rs"
        )
        module = root / module_path
        module.parent.mkdir(parents=True, exist_ok=True)
        module.write_text("//! assigned proof home\n", encoding="utf-8")
        rows.append(
            {
                "claim": claim,
                "phase": REQUIRED_PHASES[claim],
                "proof_class": "real_boundary" if external else "local_algorithm",
                "compiled_owner": (
                    "worth-ui-certification:application_contracts"
                    if external
                    else "worth-ui-runtime:lib"
                ),
                "lane": "hostile-certification" if external else "fast",
                "module_path": module_path,
                "production_entry_point": "production entry",
                "independent_observation": "external state",
                "fake_implementation_rejected": "plausible fake",
                "status": "assigned",
            }
        )
    suite = root / "application_contracts.rs"
    suite.write_text(
        "".join(
            f'#[path = "{row["module_path"]}"]\nmod proof;\n'
            for row in rows
            if row["claim"] in EXTERNAL_CLAIMS
        ),
        encoding="utf-8",
    )
    config = {
        "real_boundary_proof_ledger": "ledger.csv",
        "application_contracts_suite": "application_contracts.rs",
    }
    write_ledger(root, config, rows)
    return rows, config


def write_ledger(root, config, rows) -> None:
    with (root / config["real_boundary_proof_ledger"]).open(
        "w", encoding="utf-8", newline=""
    ) as destination:
        writer = csv.DictWriter(destination, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)
