import csv
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest import TestCase

from worth_ui_test_seam_inventory import (
    REQUIRED_FAMILIES,
    test_seam_inventory_violations,
)


class WorthUiTestSeamInventoryTests(TestCase):
    def test_new_test_only_production_seam_requires_a_disposition(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            source = root / "crate/src/authority.rs"
            source.parent.mkdir(parents=True)
            rows = [
                {
                    "family": family,
                    "path": "crate/src/authority.rs",
                    "symbol": f"{family}_for_test",
                    "disposition": "below_authority_injection",
                    "owner_phase": "3",
                    "reason": "hostile pressure remains below authority",
                }
                for family in sorted(REQUIRED_FAMILIES)
            ]
            source.write_text(
                "\n".join(f"fn {row['symbol']}() {{}}" for row in rows) + "\n",
                encoding="utf-8",
            )
            inventory = root / "inventory.csv"
            write_inventory(inventory, rows)
            config = {
                "test_seam_inventory": "inventory.csv",
                "test_seam_roots": ["crate/src"],
                "closed_phase": 2,
            }
            self.assertEqual(test_seam_inventory_violations(root, config), [])
            source.write_text(
                source.read_text(encoding="utf-8")
                + "fn authority_bypass_for_test() {}\n"
                + "enum OperationalPath {\n"
                + "    Real,\n"
                + "    #[cfg(test)]\n"
                + "    FakeAuthority,\n"
                + "}\n",
                encoding="utf-8",
            )

            violations = test_seam_inventory_violations(root, config)

            self.assertIn("authority_bypass_for_test", "\n".join(item.detail for item in violations))
            self.assertIn("FakeAuthority", "\n".join(item.detail for item in violations))

    def test_clean_family_is_explicit_and_new_seams_still_fail_discovery(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            source = root / "crate/src/authority.rs"
            source.parent.mkdir(parents=True)
            seam_free_family = "virtualized_data"
            rows = [
                {
                    "family": family,
                    "path": "crate/src/authority.rs",
                    "symbol": f"{family}_for_test",
                    "disposition": "below_authority_injection",
                    "owner_phase": "3",
                    "reason": "hostile pressure remains below authority",
                }
                for family in sorted(REQUIRED_FAMILIES - {seam_free_family})
            ]
            source.write_text(
                "\n".join(f"fn {row['symbol']}() {{}}" for row in rows) + "\n",
                encoding="utf-8",
            )
            write_inventory(root / "inventory.csv", rows)
            config = {
                "test_seam_inventory": "inventory.csv",
                "test_seam_roots": ["crate/src"],
                "seam_free_families": [seam_free_family],
                "closed_phase": 2,
            }
            self.assertEqual(test_seam_inventory_violations(root, config), [])

            source.write_text(
                source.read_text(encoding="utf-8")
                + "fn virtualized_authority_bypass_for_test() {}\n",
                encoding="utf-8",
            )

            violations = test_seam_inventory_violations(root, config)
            self.assertIn(
                "virtualized_authority_bypass_for_test",
                "\n".join(item.detail for item in violations),
            )

    def test_delete_disposition_cannot_survive_its_closed_owner_phase(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            source = root / "crate/src/authority.rs"
            source.parent.mkdir(parents=True)
            rows = [
                {
                    "family": family,
                    "path": "crate/src/authority.rs",
                    "symbol": f"{family}_for_test",
                    "disposition": (
                        "delete" if family == "ordinary" else "below_authority_injection"
                    ),
                    "owner_phase": "5" if family == "ordinary" else "9",
                    "reason": "the assigned lifecycle disposition is explicit",
                }
                for family in sorted(REQUIRED_FAMILIES)
            ]
            source.write_text(
                "\n".join(f"fn {row['symbol']}() {{}}" for row in rows) + "\n",
                encoding="utf-8",
            )
            write_inventory(root / "inventory.csv", rows)
            config = {
                "test_seam_inventory": "inventory.csv",
                "test_seam_roots": ["crate/src"],
                "closed_phase": 8,
            }

            violations = test_seam_inventory_violations(root, config)

            self.assertIn(
                "ordinary_for_test: deletion assigned to closed phase 5 still exists",
                "\n".join(item.detail for item in violations),
            )


def write_inventory(path: Path, rows: list[dict[str, str]]) -> None:
    with path.open("w", encoding="utf-8", newline="") as destination:
        writer = csv.DictWriter(destination, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)
