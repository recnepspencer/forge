from __future__ import annotations

import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "workspaces/worth-ui/Cargo.toml"
REQUIRED = {
    'wgpu feature "dx12"',
    'wgpu feature "parking_lot"',
    'wgpu feature "std"',
    'wgpu feature "wgsl"',
    'winit feature "rwh_06"',
}
def resolved_features() -> str:
    result = subprocess.run(
        [
            "cargo",
            "tree",
            "--manifest-path",
            str(MANIFEST),
            "-p",
            "worth-ui-host-native",
            "-e",
            "features",
            "--prefix",
            "none",
        ],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(result.stderr)
    return result.stdout


def main() -> int:
    try:
        features = resolved_features()
    except (OSError, RuntimeError) as error:
        print(f"native feature audit failed: {error}", file=sys.stderr)
        return 2
    resolved = {
        line
        for line in features.splitlines()
        if line.startswith('wgpu feature "') or line.startswith('winit feature "')
    }
    missing = sorted(REQUIRED.difference(resolved))
    unexpected = sorted(resolved.difference(REQUIRED))
    if missing or unexpected:
        print(
            f"native feature posture mismatch: missing={missing}; unexpected={unexpected}",
            file=sys.stderr,
        )
        return 1
    print("Worth UI native resolved features are DX12-only and profile-exact")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
