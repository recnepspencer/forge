from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

from runner.generation import ScaffoldRequest, generate_scaffold
from runner.authority.run_identity import CANONICAL_RUNTIME_ROOT
from runner.authority.config import load_config, validate_config


class GenerationTests(unittest.TestCase):
    def test_generation_is_deterministic_and_denies_overwrite(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "spec.md").write_text("spec", encoding="utf-8")
            request = ScaffoldRequest("milestone", "demo", root, "spec.md")
            first = generate_scaffold(request)
            payload = json.loads(first.config_path.read_text(encoding="utf-8"))
            self.assertEqual(payload["project"]["name"], "demo")
            self.assertEqual(validate_config(load_config(first.config_path), first.config_path), [])
            with self.assertRaises(FileExistsError): generate_scaffold(request)

    def test_generation_denies_unknown_kind(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            with self.assertRaises(ValueError): ScaffoldRequest("legacy", "demo", Path(temp), "spec.md")

    def test_generation_denies_runtime_root(self) -> None:
        with self.assertRaises(ValueError):
            generate_scaffold(ScaffoldRequest("milestone", "demo", CANONICAL_RUNTIME_ROOT, "spec.md"))

    def test_every_advertised_kind_validates(self) -> None:
        for kind in ("milestone", "single_prompt", "handoff"):
            with tempfile.TemporaryDirectory() as temp:
                root = Path(temp); (root / "spec.md").write_text("spec", encoding="utf-8")
                result = generate_scaffold(ScaffoldRequest(kind, "demo", root, "spec.md"))
                self.assertEqual(validate_config(load_config(result.config_path), result.config_path), [])

    def test_telegram_generation_uses_package_local_command_hook(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); (root / "spec.md").write_text("spec", encoding="utf-8")
            result = generate_scaffold(ScaffoldRequest("milestone", "demo", root, "spec.md", telegram=True))
            payload = json.loads(result.config_path.read_text(encoding="utf-8"))
            self.assertEqual(payload["notification_policy"]["command_hook"], ["python", "-m", "runner.telegram_bridge", "send"])
            self.assertTrue((result.prompt_root / "assets" / "recovery" / "operator_injection_overlay.md").exists())
            self.assertEqual(validate_config(load_config(result.config_path), result.config_path), [])


if __name__ == "__main__": unittest.main()
