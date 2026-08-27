from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "workspaces/worth-ui/Cargo.toml"
MODES = (
    ("default", ()),
    ("all-features", ("--all-features",)),
    ("windows", ("--filter-platform", "x86_64-pc-windows-msvc")),
)
EXPECTED_FEATURES = {
    ("wgpu", "29.0.4"): {"dx12", "parking_lot", "std", "wgsl"},
    ("winit", "0.30.13"): {"rwh_06"},
}
EXPECTED_DEPENDENCIES = {
    ("worth-ui-host-native", "0.1.0"): {
        "pollster",
        "rustybuzz",
        "sha2",
        "swash",
        "toml",
        "wgpu",
        "winit",
        "winsafe",
        "worth_proof",
        "worth_signal",
        "worth_ui_host_contract",
        "worth_ui_retained_order",
    },
    ("worth-ui-native-platform", "0.1.0"): {"worth_ui_runtime"},
    ("worth-ui-host-headless", "0.1.0"): {
        "worth_ui_host_contract",
        "worth_ui_retained_order",
        "worth_ui_test_support",
    },
}


def metadata(extra: tuple[str, ...]) -> dict[str, object]:
    result = subprocess.run(
        [
            "cargo",
            "metadata",
            "--format-version",
            "1",
            "--locked",
            "--manifest-path",
            str(MANIFEST),
            *extra,
        ],
        cwd=ROOT,
        capture_output=True,
        text=True,
        encoding="utf-8",
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(result.stderr)
    return json.loads(result.stdout)


def resolved_node(graph: dict[str, object], name: str, version: str) -> dict[str, object]:
    packages = graph.get("packages")
    resolve = graph.get("resolve")
    if not isinstance(packages, list) or not isinstance(resolve, dict):
        raise ValueError("Cargo metadata omits packages or resolution")
    identity = next(
        (
            package.get("id")
            for package in packages
            if isinstance(package, dict)
            and package.get("name") == name
            and package.get("version") == version
        ),
        None,
    )
    nodes = resolve.get("nodes")
    if identity is None or not isinstance(nodes, list):
        raise ValueError(f"missing package {name} {version}")
    node = next(
        (
            candidate
            for candidate in nodes
            if isinstance(candidate, dict) and candidate.get("id") == identity
        ),
        None,
    )
    if not isinstance(node, dict):
        raise ValueError(f"missing resolved node {name} {version}")
    return node


def validate(graph: dict[str, object], mode: str) -> None:
    for (name, version), expected in EXPECTED_FEATURES.items():
        node = resolved_node(graph, name, version)
        observed = set(node.get("features", ()))
        if observed != expected:
            raise ValueError(
                f"{mode}: {name} features drifted: {sorted(observed)} != {sorted(expected)}"
            )
    for (name, version), expected in EXPECTED_DEPENDENCIES.items():
        node = resolved_node(graph, name, version)
        dependencies = node.get("deps")
        if not isinstance(dependencies, list):
            raise ValueError(f"{mode}: {name} resolved dependencies are absent")
        observed = {
            dependency.get("name")
            for dependency in dependencies
            if isinstance(dependency, dict) and isinstance(dependency.get("name"), str)
        }
        if observed != expected:
            raise ValueError(
                f"{mode}: {name} dependencies drifted: "
                f"{sorted(observed)} != {sorted(expected)}"
            )


def main() -> int:
    try:
        for mode, extra in MODES:
            validate(metadata(extra), mode)
    except (json.JSONDecodeError, OSError, RuntimeError, ValueError) as error:
        print(f"native resolved-graph audit failed: {error}", file=sys.stderr)
        return 1
    print("Worth UI native resolved graphs are exact in default, all-feature, and Windows modes")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
