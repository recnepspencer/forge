from pathlib import Path

def ensure_import(text: str, names: list[str]) -> str:
    if all(n in text for n in names):
        return text
    needle = "install_application_aftermath, AftermathDeclaredReadCoverage"
    if needle in text:
        extra = ", ".join(names)
        return text.replace(
            needle,
            f"install_application_aftermath, AftermathDeclaredReadCoverage, {extra}",
            1,
        )
    return text

files = {
    "workspaces/worth-query/crates/worth-query-execution/src/domain_computation/application_aftermath/undo_admission_tests.rs": {
        "imports": ["AftermathLoweringCorrespondenceCatalog", "InstalledLoweringCorrespondence"],
        "inverse_slot": "account-freeze-inverse",
        "inverse_graph": "0xD2",
    },
    "workspaces/worth-query/crates/worth-query-execution/src/domain_computation/provider_session/protocol/declared_closure_tests.rs": {
        "imports": ["AftermathLoweringCorrespondenceCatalog", "InstalledLoweringCorrespondence"],
        "inverse_slot": "inverse-v2",
        "inverse_graph": "0x32",
    },
    "workspaces/worth-query/crates/worth-query-execution/src/domain_computation/application_aftermath/recovery_handle/tests.rs": {
        "imports": ["AftermathLoweringCorrespondenceCatalog"],
    },
    "workspaces/worth-query-bank-world/crates/bank-server/tests/ordinary_mutations/estate_operations/phase8_recovery_expiry.rs": {
        "imports": ["AftermathLoweringCorrespondenceCatalog"],
    },
    "workspaces/worth-query-bank-world/crates/bank-server/tests/ordinary_mutations/estate_operations/phase8_recovery_mechanism.rs": {
        "imports": ["AftermathLoweringCorrespondenceCatalog", "InstalledLoweringCorrespondence"],
        "inverse_slot": "estate-death-inverse",
        "inverse_graph": "0x52",
    },
    "workspaces/worth-query/crates/worth-query/tests/installed_operating_world/installed_operation_fixture/operation_semantics.rs": {
        "domain_prefix": True,
    },
}

for path_str, cfg in files.items():
    path = Path(path_str)
    text = path.read_text(encoding="utf-8")
    if cfg.get("domain_prefix"):
        text = text.replace(
            "&AftermathLoweringCorrespondenceCatalog::empty()",
            "&domain::AftermathLoweringCorrespondenceCatalog::empty()",
        )
    else:
        text = ensure_import(text, cfg["imports"])

    if "inverse_slot" in cfg:
        slot = cfg["inverse_slot"]
        graph = cfg["inverse_graph"]
        catalog = (
            "&AftermathLoweringCorrespondenceCatalog::new([\n"
            "            InstalledLoweringCorrespondence::new(\n"
            f'                "{slot}",\n'
            "                CanonicalDigestId::new([0xCC; 32]),\n"
            "                1,\n"
            f"                CanonicalDigestId::new([{graph}; 32]),\n"
            "            )\n"
            "            .unwrap(),\n"
            "        ])"
        )
        parts = text.split("&AftermathLoweringCorrespondenceCatalog::empty()")
        rebuilt = [parts[0]]
        for part in parts[1:]:
            window = rebuilt[-1][-900:]
            if any(
                marker in window
                for marker in (
                    "DeclaredRecordedInverse",
                    "DeclaredLoweringCorrespondenceRef",
                    "recorded_inverse",
                    "RecordedInverse",
                )
            ):
                rebuilt.append(catalog + part)
            else:
                rebuilt.append("&AftermathLoweringCorrespondenceCatalog::empty()" + part)
        text = "".join(rebuilt)

    path.write_text(text, encoding="utf-8")
    print("updated", path_str)
print("done")
