from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "workspaces/worth-ui/contracts/milestone-3.16-removal-inventory.json"
REQUIRED_FAMILIES = {
    "static-paint authority": ("workspaces/worth-ui/**/*.rs", "ComponentStaticPaintContract"),
    "bootstrap component token dependencies": ("workspaces/worth-ui/**/*.rs", "with_theme_token_dependency"),
    "string-backed ThemeColorValue": ("workspaces/worth-ui/**/*.rs", "ThemeColorValue"),
    "Pulse Unicode icon text": ("workspaces/worth-ui/**/*.rs", "portal_icon_text"),
    "direct token publication": ("workspaces/worth-ui/**/*.rs", "UiNativeThemeTokenValueChange"),
    "legacy changed-node selection": ("workspaces/worth-ui/**/*.rs", "changed_graph_nodes"),
}


def validate(root: Path, manifest: Path) -> None:
    inventory = json.loads(manifest.read_text(encoding="utf-8"))
    families: set[str] = set()
    for entry in inventory["entries"]:
        family = entry["family"]
        if family in families:
            raise ValueError(f"duplicate inventory family: {family}")
        families.add(family)
        expected = REQUIRED_FAMILIES.get(family)
        if expected is None or (entry["glob"], entry["literal"]) != expected:
            raise ValueError(f"unexpected removal inventory contract for {family}")
        files = [path for path in root.glob(entry["glob"]) if path.is_file()]
        observed = sum(path.read_text(encoding="utf-8").count(entry["literal"]) for path in files)
        if observed != entry["baseline"]:
            raise ValueError(f"{family}: observed {observed}, expected exact baseline {entry['baseline']}")
    if inventory.get("cutover_target") != 0:
        raise ValueError("cutover target must be exactly zero")
    if families != set(REQUIRED_FAMILIES):
        raise ValueError("removal inventory must contain every required legacy family exactly once")


def main() -> int:
    try:
        validate(ROOT, MANIFEST)
    except (OSError, KeyError, TypeError, ValueError, json.JSONDecodeError) as error:
        print(f"Worth UI removal inventory gate failed: {error}", file=sys.stderr)
        return 1
    print("Worth UI removal inventory gate passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
