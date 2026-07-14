from __future__ import annotations

import sys
import tempfile
import unittest
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from check_worth_ui_phase_10_authority_boundaries import check_manifest


class Phase10AuthorityBoundaryGuardTests(unittest.TestCase):
    def test_exact_manifest_passes(self) -> None:
        with fixture_workspace() as fixture:
            self.assertEqual(check_manifest(fixture.root, fixture.manifest), [])

    def test_new_constructor_path_fails(self) -> None:
        with fixture_workspace() as fixture:
            extra = fixture.root / "src" / "adapter.rs"
            extra.write_text("fn dirty() { canonical_mint(); }\n", encoding="utf-8")
            violations = check_manifest(fixture.root, fixture.manifest)
            self.assertTrue(any("adapter.rs" in item.message for item in violations))

    def test_growth_in_allowed_path_fails(self) -> None:
        with fixture_workspace() as fixture:
            owner = fixture.root / "src" / "owner.rs"
            owner.write_text("canonical_mint();\ncanonical_mint();\n", encoding="utf-8")
            violations = check_manifest(fixture.root, fixture.manifest)
            self.assertTrue(any("expected 1" in item.message for item in violations))

    def test_deletion_requires_manifest_update(self) -> None:
        with fixture_workspace() as fixture:
            (fixture.root / "src" / "legacy.rs").write_text("", encoding="utf-8")
            violations = check_manifest(fixture.root, fixture.manifest)
            self.assertTrue(any("expected 1" in item.message for item in violations))

    def test_fallible_work_inside_live_commit_body_fails(self) -> None:
        with fixture_workspace() as fixture:
            prepared = fixture.root / "src" / "prepared.rs"
            prepared.write_text(
                "impl UiPreparedCommittedAllocationActivation<'_> {\n"
                "    fn commit_once(self) { let _ = risky()?; }\n"
                "}\n",
                encoding="utf-8",
            )
            violations = check_manifest(fixture.root, fixture.manifest)
            self.assertTrue(
                any(item.rule_id == "infallible_commit_once_body" for item in violations)
            )


class fixture_workspace:
    def __init__(self) -> None:
        self._temporary = tempfile.TemporaryDirectory()
        self.root = Path(self._temporary.name)
        self.manifest = {
            "roots": ["src"],
            "rules": [
                {
                    "id": "mint",
                    "pattern": "canonical_mint",
                    "allowed": {"src/owner.rs": 1},
                },
                {
                    "id": "legacy",
                    "pattern": "legacy_publish",
                    "allowed": {"src/legacy.rs": 1},
                },
                {
                    "id": "infallible_commit_once_body",
                    "pattern": r"(?s)impl UiPreparedCommittedAllocationActivation<'_> \{.*?fn commit_once.*?\{.*?(?:Result<|Option<|assert!?|\.expect\(|\.unwrap\(|borrow_mut\(|try_borrow|\?)",
                    "allowed": {},
                },
            ],
        }

    def __enter__(self) -> fixture_workspace:
        source = self.root / "src"
        source.mkdir()
        (source / "owner.rs").write_text("canonical_mint();\n", encoding="utf-8")
        (source / "legacy.rs").write_text("legacy_publish();\n", encoding="utf-8")
        (source / "prepared.rs").write_text(
            "impl UiPreparedCommittedAllocationActivation<'_> {\n"
            "    fn commit_once(self) { publish(); }\n"
            "}\n",
            encoding="utf-8",
        )
        return self

    def __exit__(self, *args: object) -> None:
        self._temporary.cleanup()


if __name__ == "__main__":
    unittest.main()
