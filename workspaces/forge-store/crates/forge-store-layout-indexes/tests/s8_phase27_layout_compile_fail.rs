#[test]
fn phase27_layout_surfaces_reject_forgeable_security_rule_shortcuts() {
    for fixture in fixtures() {
        assert_compile_fails(fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct CompileFailFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

const fn fixtures() -> [CompileFailFixture; 15] {
    [
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_tenant_scope_layout.rs",
            expected_stderr: &["AdmittedTenantScopeLayoutRule", "private field"],
        },
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_key_scope_layout.rs",
            expected_stderr: &["AdmittedKeyScopeLayoutRule", "private field"],
        },
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_authenticity_layout.rs",
            expected_stderr: &["AdmittedAuthenticityLayoutRule", "private field"],
        },
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_custody_layout.rs",
            expected_stderr: &["AdmittedCustodyLayoutRule", "private field"],
        },
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_repair_blast_radius_layout.rs",
            expected_stderr: &["AdmittedRepairBlastRadiusLayoutRule", "private field"],
        },
        CompileFailFixture {
            name: "admitted_tenant_scope_layout_rule_constructor_is_not_public.rs",
            expected_stderr: &["AdmittedTenantScopeLayoutRule", "phase27"],
        },
        CompileFailFixture {
            name: "admitted_key_scope_layout_rule_constructor_is_not_public.rs",
            expected_stderr: &["AdmittedKeyScopeLayoutRule", "phase27"],
        },
        CompileFailFixture {
            name: "admitted_authenticity_layout_rule_constructor_is_not_public.rs",
            expected_stderr: &["AdmittedAuthenticityLayoutRule", "phase27"],
        },
        CompileFailFixture {
            name: "admitted_custody_layout_rule_constructor_is_not_public.rs",
            expected_stderr: &["AdmittedCustodyLayoutRule", "phase27"],
        },
        CompileFailFixture {
            name: "admitted_repair_blast_radius_layout_rule_constructor_is_not_public.rs",
            expected_stderr: &["AdmittedRepairBlastRadiusLayoutRule", "phase27"],
        },
        CompileFailFixture {
            name: "caller_defined_report_cannot_open_tenant_scope_layout.rs",
            expected_stderr: &["TenantScopeLayoutReport", "private field"],
        },
        CompileFailFixture {
            name: "caller_defined_report_cannot_open_key_scope_layout.rs",
            expected_stderr: &["KeyScopeLayoutReport", "private field"],
        },
        CompileFailFixture {
            name: "caller_defined_report_cannot_open_authenticity_layout.rs",
            expected_stderr: &["AuthenticityLayoutReport", "private field"],
        },
        CompileFailFixture {
            name: "caller_defined_report_cannot_open_custody_layout.rs",
            expected_stderr: &["CustodyLayoutReport", "private field"],
        },
        CompileFailFixture {
            name: "caller_defined_report_cannot_open_repair_blast_radius_layout.rs",
            expected_stderr: &["RepairBlastRadiusLayoutReport", "private field"],
        },
    ]
}

fn assert_compile_fails(fixture: CompileFailFixture) {
    let case_root = std::env::temp_dir()
        .join("forge-store-phase27-ui")
        .join(fixture.name.replace('.', "_"));
    if case_root.exists() {
        std::fs::remove_dir_all(&case_root).unwrap();
    }
    let src_dir = case_root.join("src");
    std::fs::create_dir_all(&src_dir).unwrap();
    std::fs::write(case_root.join("Cargo.toml"), compile_fail_manifest()).unwrap();
    std::fs::copy(fixture_path(fixture.name), src_dir.join("main.rs")).unwrap();

    let output = std::process::Command::new("cargo")
        .arg("check")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(case_root.join("Cargo.toml"))
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "{} unexpectedly compiled",
        fixture.name
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    for expected in fixture.expected_stderr {
        assert!(
            stderr.contains(expected),
            "{} failed for the wrong reason; missing stderr fragment {expected:?}\nstderr:\n{stderr}",
            fixture.name
        );
    }
}

fn fixture_path(fixture_name: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ui")
        .join("phase27")
        .join(fixture_name)
}

fn compile_fail_manifest() -> String {
    format!(
        "[package]\nname = \"phase27-ui-case\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nforge-store-security = {{ path = {:?} }}\n",
        workspace_root()
            .join("workspaces")
            .join("forge-store")
            .join("crates")
            .join("forge-store-security"),
    )
}

fn workspace_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("layout-indexes crate lives under forge/workspaces/forge-store/crates")
        .to_path_buf()
}
