from pathlib import Path
from tempfile import TemporaryDirectory
from unittest import TestCase

from run_worth_ui_compile_contracts import (
    Case,
    canonical_diagnostics,
    diagnostic_sources,
    included_sources,
)


class WorthUiCompileContractRunnerTests(TestCase):
    def test_canonical_diagnostics_retain_exact_error_meaning(self) -> None:
        source = Path(
            "workspaces/worth-ui/crates/worth-ui/tests/ui/example/fail.rs"
        ).resolve()
        case = Case("fail", "example", source, source.with_suffix(".stderr"), "product")
        messages = [
            {
                "level": "error",
                "code": {"code": "E0451"},
                "message": "field `sealed` is private",
                "spans": [{"file_name": str(source), "is_primary": True}],
                "children": [{"level": "help", "message": "use the public constructor"}],
            }
        ]

        rendered = canonical_diagnostics(messages, case)

        self.assertIn("error[E0451]: field `sealed` is private", rendered)
        self.assertIn("$WORKSPACE/crates/worth-ui/tests/ui/example/fail.rs", rendered)
        self.assertIn("help: use the public constructor", rendered)

    def test_included_source_without_a_primary_error_is_observable(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            aggregate = root / "aggregate.rs"
            covered = root / "covered.rs"
            aggregate.write_text(
                'mod covered { include!("covered.rs"); }\nfn main() {}\n',
                encoding="utf-8",
            )
            covered.write_text('compile_error!("covered");\n', encoding="utf-8")
            messages = [
                {
                    "level": "error",
                    "spans": [{"file_name": str(aggregate), "is_primary": True}],
                }
            ]

            missing = included_sources(aggregate) - diagnostic_sources(messages)

            self.assertEqual(missing, {covered.resolve()})
