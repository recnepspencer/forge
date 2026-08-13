use super::{derive_family_at, fixture_family, Fixture};

#[test]
fn private_dependency_aliases_cannot_hide_public_module_reexports() {
    let fixture = Fixture::new("private-dependency-alias");
    fixture.write(
        "workspaces/worth-store/crates/worth-store-wal/src/lib.rs",
        concat!(
            "pub mod artifact_store { pub struct Artifact; }\n",
            "pub mod checkpoint { pub struct Checkpoint; }\n",
        ),
    );
    for binding in [
        "use worth_store_wal as dep;\n",
        "extern crate worth_store_wal as dep;\n",
    ] {
        for target in ["artifact_store", "checkpoint"] {
            fixture.write(
                "lib.rs",
                &format!("{binding}pub use dep::{target} as wal_namespace;\n"),
            );
            let denial = derive_family_at(&fixture.root, &fixture_family())
                .expect_err("private dependency alias must not hide a namespace re-export");
            assert!(denial.contains("private import alias"));
        }

        fixture.write(
            "lib.rs",
            &format!(
                "{binding}pub mod nested {{ pub use dep::artifact_store as wal_namespace; }}\n"
            ),
        );
        let denial = derive_family_at(&fixture.root, &fixture_family())
            .expect_err("unqualified ancestor alias must fail closed in a child module");
        assert!(denial.contains("private import alias"));
    }

    for qualified in ["super::dep", "crate::dep"] {
        fixture.write(
            "lib.rs",
            &format!(
                "use worth_store_wal as dep;\npub mod nested {{ pub use {qualified}::artifact_store as wal_namespace; }}\n"
            ),
        );
        let denial = derive_family_at(&fixture.root, &fixture_family())
            .expect_err("ancestor private dependency alias must fail closed");
        assert!(denial.contains("private import alias"));
    }
}

#[test]
fn cfg_exclusive_local_types_cannot_mask_private_dependency_aliases() {
    let fixture = Fixture::new("cfg-private-dependency-alias");
    fixture.write(
        "workspaces/worth-store/crates/worth-store-wal/src/lib.rs",
        "pub struct VerifiedWalSegment; pub struct VerifiedWalActiveTail;\n",
    );
    for target in ["VerifiedWalSegment", "VerifiedWalActiveTail"] {
        fixture.write(
            "lib.rs",
            &format!(
                concat!(
                    "#[cfg(unix)] pub mod dep {{ ",
                    "pub struct VerifiedWalSegment; pub struct VerifiedWalActiveTail; }}\n",
                    "#[cfg(windows)] use worth_store_wal as dep;\n",
                    "#[cfg(windows)] pub use dep::{target} as Twin;\n",
                ),
                target = target
            ),
        );
        let denial = derive_family_at(&fixture.root, &fixture_family())
            .expect_err("cfg-exclusive private alias must not inherit a local type's provenance");
        assert!(denial.contains("private import alias"));
    }
}
