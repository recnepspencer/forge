from pathlib import Path
from tempfile import TemporaryDirectory
from unittest import TestCase, main

from check_worth_ui_test_topology import rust_test_sources, source_violations
from worth_ui_ci_contract import ci_contract_violations


class WorthUiTestTopologySourceTests(TestCase):
    def test_ci_contract_accepts_required_and_rejects_no_forbidden_fragments(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            workflow = root / ".github/workflows/ci.yml"
            setup = root / ".github/actions/setup-worth-ui/action.yml"
            workflow.parent.mkdir(parents=True)
            setup.parent.mkdir(parents=True)
            workflow.write_text("parallel-proof\ntimeout-minutes:\n", encoding="utf-8")
            setup.write_text("pinned-toolchain\nsccache\n", encoding="utf-8")
            config = self.ci_contract_config()

            violations = ci_contract_violations(root, config)

            self.assertEqual(violations, [])

    def test_ci_contract_localizes_missing_and_forbidden_configuration(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            workflow = root / ".github/workflows/ci.yml"
            setup = root / ".github/actions/setup-worth-ui/action.yml"
            workflow.parent.mkdir(parents=True)
            setup.parent.mkdir(parents=True)
            workflow.write_text("serial-full\n", encoding="utf-8")
            setup.write_text("target-cache\n", encoding="utf-8")

            violations = ci_contract_violations(root, self.ci_contract_config())

            self.assertEqual({violation.rule for violation in violations}, {"ci-contract"})
            details = "\n".join(violation.detail for violation in violations)
            self.assertIn("parallel-proof", details)
            self.assertIn("serial-full", details)
            self.assertIn("pinned-toolchain", details)
            self.assertIn("target-cache", details)

    @staticmethod
    def ci_contract_config() -> dict[str, object]:
        return {
            "ci_contract": {
                "workflow": ".github/workflows/ci.yml",
                "setup_action": ".github/actions/setup-worth-ui/action.yml",
                "required_workflow_fragments": ["parallel-proof", "timeout-minutes:"],
                "forbidden_workflow_fragments": ["serial-full"],
                "required_setup_fragments": ["pinned-toolchain", "sccache"],
                "forbidden_setup_fragments": ["target-cache"],
            }
        }

    def test_nested_test_modules_are_enforced_but_embedded_repositories_are_not(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            package = root / "package"
            nested = package / "tests" / "nested" / "module.rs"
            embedded = (
                package
                / "tests"
                / "fixtures"
                / "topology_negative"
                / "fake.rs"
            )
            nested.parent.mkdir(parents=True)
            embedded.parent.mkdir(parents=True)
            (package / "Cargo.toml").write_text(
                '[package]\nname = "topology-test"\nversion = "0.1.0"\n',
                encoding="utf-8",
            )
            nested.write_text('const RUSTFLAGS: &str = "-Awarnings";\n', encoding="utf-8")
            embedded.write_text('const RUSTFLAGS: &str = "fixture data";\n', encoding="utf-8")
            config = {
                "packages": {"topology-test": {"manifest": "package/Cargo.toml"}},
                "allowed_trybuild_sessions": [],
                "max_trybuild_sessions": 0,
            }

            sources = rust_test_sources(root, config)
            violations = source_violations(root, config)

            self.assertEqual(sources, [nested])
            self.assertEqual(len(violations), 1)
            self.assertEqual(violations[0].rule, "warning-fingerprint")
            self.assertIn("tests/nested/module.rs", violations[0].detail)


if __name__ == "__main__":
    main()
