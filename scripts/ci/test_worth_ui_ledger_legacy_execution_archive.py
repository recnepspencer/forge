from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parent))

from worth_ui_ledger_execution_observation_migration import LEGACY_ROOT
from worth_ui_ledger_legacy_execution_archive import archive, audit


class LegacyExecutionArchiveTests(unittest.TestCase):
    def test_audit_is_read_only_and_archive_is_recoverable(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            ledger = root / "ledger.csv"
            ledger.write_text("phase,requirement\n", encoding="utf-8")
            portfolio = root / "portfolio.json"
            portfolio.write_text("{}", encoding="utf-8")
            first = legacy(root, "a" * 64, b"first")
            second = legacy(root, "b" * 64, b"second")
            retained = {
                "source_revision": "c" * 40,
                "source_state_digest": "d" * 64,
                "execution_observation_migrations": [],
            }
            with (
                patch(
                    "worth_ui_ledger_legacy_execution_archive.portfolio_identity",
                    return_value="portfolio.json",
                ),
                patch(
                    "worth_ui_ledger_legacy_execution_archive.validate",
                    return_value=retained,
                ),
                patch(
                    "worth_ui_ledger_legacy_execution_archive.ARCHIVE_ROOT",
                    Path("archive"),
                ),
            ):
                manifest, destination = audit(root, ledger, 3)
                self.assertTrue(first.is_file())
                self.assertTrue(second.is_file())
                self.assertEqual(manifest["unreachable_execution_count"], 2)
                result = archive(root, ledger, 3)
            self.assertFalse((root / LEGACY_ROOT).exists())
            archived = root / str(result["archive_root"])
            self.assertEqual(destination, archived)
            self.assertEqual((archived / "executions" / "aa" / first.name).read_bytes(), b"first")
            self.assertEqual((archived / "executions" / "bb" / second.name).read_bytes(), b"second")
            self.assertTrue((archived / "manifest.json").is_file())

    def test_active_identity_requires_an_embedded_validated_migration(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            ledger = root / "ledger.csv"
            ledger.write_text("phase,requirement\n", encoding="utf-8")
            (root / "portfolio.json").write_text("{}", encoding="utf-8")
            key = "a" * 64
            legacy(root, key, b"active")
            migration = (
                root
                / "_docs/worth-ui/milestone-3.14.1-evidence/"
                "execution-observation-migrations"
                / key[:2]
                / f"{key}.json"
            )
            migration.parent.mkdir(parents=True, exist_ok=True)
            migration.write_text("{}", encoding="utf-8")
            retained = {
                "source_revision": "c" * 40,
                "source_state_digest": "d" * 64,
                "execution_observation_migrations": [
                    {"legacy_execution_key": key}
                ],
            }
            with (
                patch(
                    "worth_ui_ledger_legacy_execution_archive.portfolio_identity",
                    return_value="portfolio.json",
                ),
                patch(
                    "worth_ui_ledger_legacy_execution_archive.validate",
                    return_value=retained,
                ),
            ):
                with self.assertRaisesRegex(RuntimeError, "no embedded legacy envelope"):
                    audit(root, ledger, 3)
            self.assertTrue((root / LEGACY_ROOT).exists())


def legacy(root: Path, key: str, content: bytes) -> Path:
    identity = root / LEGACY_ROOT / key[:2] / f"{key}.json"
    identity.parent.mkdir(parents=True, exist_ok=True)
    identity.write_bytes(content)
    return identity


if __name__ == "__main__":
    unittest.main()
