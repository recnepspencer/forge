from __future__ import annotations

import json
import pathlib
import subprocess
import sys
import tempfile
import unittest

from rust_function_scrutiny import (
    count_explicit_parameters,
    dirty_rust_files,
    rust_files_for_paths,
    scan_source,
)


class RustFunctionScrutinyTests(unittest.TestCase):
    def test_reports_long_and_many_parameter_functions_without_literal_false_positives(self) -> None:
        long_body = "\n".join("    let _value = 1;" for _ in range(60))
        source = f'''// fn commented(a: u8, b: u8, c: u8, d: u8, e: u8) {{}}
const TEXT: &str = "fn stringly(a: u8, b: u8, c: u8, d: u8, e: u8) {{}}";
fn long_and_wide(
    first: u8,
    second: Option<(u8, u8)>,
    third: impl Fn(u8, u8),
    fourth: [u8; 2],
    fifth: u8,
) {{
{long_body}
}}
'''

        candidates, errors = scan_source(source, "sample.rs")

        self.assertEqual(errors, [])
        self.assertEqual(len(candidates), 1)
        self.assertEqual(candidates[0].name, "long_and_wide")
        self.assertEqual(candidates[0].parameter_count, 5)
        self.assertEqual(candidates[0].reasons, ("long-function", "many-parameters"))

    def test_receiver_is_not_counted_as_an_explicit_parameter(self) -> None:
        source = "impl Value { fn inspect(&mut self, a:u8,b:u8,c:u8,d:u8,e:u8) {} }"

        candidates, errors = scan_source(source, "receiver.rs")

        self.assertEqual(errors, [])
        self.assertEqual(candidates[0].parameter_count, 5)
        self.assertEqual(candidates[0].reasons, ("many-parameters",))
        self.assertEqual(count_explicit_parameters("self: Box<Self>, a: u8"), 1)

    def test_threshold_edges_are_advisory_candidates_only_after_crossing(self) -> None:
        sixty_lines = "fn exact() {\n" + ("let x = 1;\n" * 58) + "}"
        sixty_one_lines = "fn over() {\n" + ("let x = 1;\n" * 59) + "}"

        exact, _ = scan_source(sixty_lines, "exact.rs")
        over, _ = scan_source(sixty_one_lines, "over.rs")

        self.assertEqual(exact, [])
        self.assertEqual(over[0].line_count, 61)
        self.assertEqual(over[0].reasons, ("long-function",))

    def test_folder_discovery_skips_build_products(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            (root / "src").mkdir()
            (root / "target").mkdir()
            included = root / "src" / "lib.rs"
            excluded = root / "target" / "generated.rs"
            included.write_text("fn included() {}", encoding="utf-8")
            excluded.write_text("fn excluded() {}", encoding="utf-8")

            files = rust_files_for_paths([root])

            self.assertEqual(files, [included.resolve()])

    def test_dirty_mode_unions_staged_unstaged_and_untracked_rust_files(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            self.git(root, "init")
            self.git(root, "config", "user.email", "qa@example.invalid")
            self.git(root, "config", "user.name", "QA")
            staged = root / "staged.rs"
            unstaged = root / "unstaged.rs"
            staged.write_text("fn staged() {}", encoding="utf-8")
            unstaged.write_text("fn unstaged() {}", encoding="utf-8")
            self.git(root, "add", "staged.rs", "unstaged.rs")
            self.git(root, "commit", "-m", "baseline")
            staged.write_text("fn staged_changed() {}", encoding="utf-8")
            self.git(root, "add", "staged.rs")
            unstaged.write_text("fn unstaged_changed() {}", encoding="utf-8")
            untracked = root / "untracked.rs"
            untracked.write_text("fn untracked() {}", encoding="utf-8")

            worktree, files = dirty_rust_files(root)

            self.assertEqual(worktree, root.resolve())
            self.assertEqual(
                files,
                sorted(path.resolve() for path in (staged, unstaged, untracked)),
            )

    def test_json_output_is_machine_composable(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            source = root / "wide.rs"
            source.write_text("fn wide(a:u8,b:u8,c:u8,d:u8,e:u8) {}", encoding="utf-8")
            output = subprocess.run(
                [
                    sys.executable,
                    str(pathlib.Path(__file__).with_name("scrutinize_rust_functions.py")),
                    str(root),
                    "--format",
                    "json",
                ],
                check=True,
                capture_output=True,
                text=True,
            )

            report = json.loads(output.stdout)
            self.assertEqual(report["candidate_count"], 1)
            self.assertEqual(report["candidates"][0]["path"], "wide.rs")

    def test_candidates_are_advisory_unless_gating_is_explicit(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            source = root / "wide.rs"
            source.write_text(
                "fn wide(a:u8,b:u8,c:u8,d:u8,e:u8) {}", encoding="utf-8"
            )
            command = [
                sys.executable,
                str(pathlib.Path(__file__).with_name("scrutinize_rust_functions.py")),
                str(root),
            ]

            advisory = subprocess.run(command, check=False, capture_output=True, text=True)
            gated = subprocess.run(
                [*command, "--fail-on-candidates"],
                check=False,
                capture_output=True,
                text=True,
            )

            self.assertEqual(advisory.returncode, 0)
            self.assertEqual(gated.returncode, 1)

    @staticmethod
    def git(root: pathlib.Path, *arguments: str) -> None:
        subprocess.run(
            ["git", "-C", str(root), *arguments],
            check=True,
            capture_output=True,
            text=True,
        )


if __name__ == "__main__":
    unittest.main()
