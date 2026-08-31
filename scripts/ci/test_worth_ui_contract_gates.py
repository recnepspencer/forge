from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

import check_worth_ui_appearance_native_matrix as matrix_gate
import check_worth_ui_docs_links as docs_gate
import check_worth_ui_protocol_manifest as protocol_gate
import check_worth_ui_removal_inventory as removal_gate


class WorthUiContractGateTests(unittest.TestCase):
    def test_removal_inventory_detects_exact_count_drift(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "workspaces/worth-ui/sample.rs"
            source.parent.mkdir(parents=True)
            source.write_text("ComponentStaticPaintContract ComponentStaticPaintContract", encoding="utf-8")
            manifest = root / "inventory.json"
            manifest.write_text(json.dumps({"cutover_target": 0, "entries": [{
                "family": "static-paint authority", "glob": "workspaces/worth-ui/**/*.rs",
                "literal": "ComponentStaticPaintContract", "baseline": 1,
            }]}), encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "observed 2"):
                removal_gate.validate(root, manifest)

    def test_protocol_manifest_detects_live_source_drift(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            protocol = root / "workspaces/worth-ui/crates/worth-ui-host-contract/src/mounted_frame/protocol.rs"
            protocol.parent.mkdir(parents=True)
            protocol.write_text("""
                COMPATIBLE_FLOOR: u16 = 6; CURRENT: u16 = 9;
                CURRENT_FRAME_SCHEMA: u16 = 5; CURRENT_PRESENTATION_SCHEMA: u16 = 5;
                CURRENT_OBSERVATION_SCHEMA: u16 = 7; CURRENT_MEASUREMENT_SCHEMA: u16 = 5;
                CURRENT_SOLICITED_EFFECT_SCHEMA: u16 = 1;
            """, encoding="utf-8")
            text = root / "workspaces/worth-ui/crates/worth-ui-host-contract/src/mounted_projection/semantic_text.rs"
            text.parent.mkdir(parents=True)
            text.write_text("pub const fn current() -> Self { Self(3) }", encoding="utf-8")
            profile = root / "workspaces/worth-ui/crates/worth-ui-host-native/profiles/worth-ui-windows-dx12-v1.toml"
            profile.parent.mkdir(parents=True)
            profile.write_text("identity='v1'", encoding="utf-8")
            manifest = root / "protocol.json"
            manifest.write_text(json.dumps({"live": protocol_gate.EXPECTED_LIVE,
                "intended_next": protocol_gate.EXPECTED_NEXT}), encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "protocol_current drifted"):
                protocol_gate.validate(root, manifest)

    def test_protocol_manifest_rejects_joint_source_and_manifest_advancement(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            protocol = root / "workspaces/worth-ui/crates/worth-ui-host-contract/src/mounted_frame/protocol.rs"
            protocol.parent.mkdir(parents=True)
            protocol.write_text("""
                COMPATIBLE_FLOOR: u16 = 7; CURRENT: u16 = 7;
                CURRENT_FRAME_SCHEMA: u16 = 6; CURRENT_PRESENTATION_SCHEMA: u16 = 6;
                CURRENT_OBSERVATION_SCHEMA: u16 = 7; CURRENT_MEASUREMENT_SCHEMA: u16 = 5;
                CURRENT_SOLICITED_EFFECT_SCHEMA: u16 = 1;
            """, encoding="utf-8")
            text = root / "workspaces/worth-ui/crates/worth-ui-host-contract/src/mounted_projection/semantic_text.rs"
            text.parent.mkdir(parents=True)
            text.write_text("pub const fn current() -> Self { Self(4) }", encoding="utf-8")
            profile = root / "workspaces/worth-ui/crates/worth-ui-host-native/profiles/worth-ui-windows-dx12-v2.toml"
            profile.parent.mkdir(parents=True)
            profile.write_text("identity='v2'", encoding="utf-8")
            manifest = root / "protocol.json"
            advanced = dict(protocol_gate.EXPECTED_NEXT)
            manifest.write_text(json.dumps({"live": advanced,
                "intended_next": protocol_gate.EXPECTED_NEXT}), encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "live manifest must remain exact"):
                protocol_gate.validate(root, manifest)

    def test_document_gate_rejects_manifest_omission(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            manifest = root / "docs.json"
            manifest.write_text(json.dumps({
                "continuing_documents": docs_gate.EXPECTED_CONTINUING_DOCUMENTS[:-1],
                "planned_documents": docs_gate.EXPECTED_PLANNED_DOCUMENTS,
            }), encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "document set must remain exact"):
                docs_gate.validate(root, manifest)

    def test_document_gate_rejects_broken_relative_link(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            document = root / "_docs/worth-ui/readme.md"
            document.parent.mkdir(parents=True)
            document.write_text("[missing](missing.md)", encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "broken local link"):
                docs_gate.validate(root)

    def test_native_matrix_rejects_prefix_and_comment_symbol_forgeries(self) -> None:
        source = """
            // pub struct UiMountedPointerAffordanceMechanic;
            /* pub enum UiMountedBackdropMechanic {} */
            pub struct UiMountedPointerAffordanceMechanicSuffix;
        """
        self.assertEqual(
            matrix_gate.missing_declared_contract_symbols(
                source, ["UiMountedPointerAffordanceMechanic", "UiMountedBackdropMechanic"]
            ),
            ["UiMountedPointerAffordanceMechanic", "UiMountedBackdropMechanic"],
        )


if __name__ == "__main__":
    unittest.main()
