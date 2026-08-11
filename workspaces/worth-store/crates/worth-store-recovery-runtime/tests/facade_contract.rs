use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

const FILE_OWNERS: &[(&str, &str)] = &[
    ("src/entry/request.rs", "entry/request"),
    ("src/entry/configuration.rs", "entry/configuration"),
    ("src/entry/limits.rs", "entry/limits"),
    ("src/entry/counters.rs", "entry/counters"),
    ("src/entry/session.rs", "entry/session"),
    ("src/entry/authority.rs", "entry/authority"),
    ("src/entry/authority_binding.rs", "entry/authority-binding"),
    ("src/entry/outcome.rs", "entry/outcome"),
    ("src/progression/admitted.rs", "progression/admitted"),
    ("src/progression/discovered.rs", "progression/discovered"),
    ("src/progression/selected.rs", "progression/selected"),
    (
        "../worth-store-physical-backend/src/recovery_media/generation.rs",
        "worth-store-physical-backend/recovery-media/generation",
    ),
    (
        "../worth-store-physical-backend/src/recovery_media/profile.rs",
        "worth-store-physical-backend/recovery-media/profile",
    ),
    (
        "../worth-store-physical-backend/src/recovery_media/qualified.rs",
        "worth-store-physical-backend/recovery-media/qualified",
    ),
    (
        "../worth-store-physical-backend/src/recovery_media/qualification.rs",
        "worth-store-physical-backend/recovery-media/qualification",
    ),
    (
        "../worth-store-physical-backend/src/recovery_media/admitted.rs",
        "worth-store-physical-backend/recovery-media/admitted",
    ),
    (
        "../worth-store-physical-backend/src/recovery_media/discovery.rs",
        "worth-store-physical-backend/recovery-media/discovery",
    ),
    (
        "../worth-store-physical-backend/src/recovery_media/discovery/artifact.rs",
        "worth-store-physical-backend/recovery-media/discovery/artifact",
    ),
    (
        "../worth-store/src/physical_runtime/recovery_freshness/authority.rs",
        "worth-store/recovery-freshness/authority",
    ),
    (
        "../worth-store/src/physical_runtime/recovery_freshness/port.rs",
        "worth-store/recovery-freshness/port",
    ),
    (
        "../worth-store/src/physical_runtime/recovery_freshness/registration.rs",
        "worth-store/recovery-freshness/registration",
    ),
    (
        "../worth-store/src/physical_runtime/recovery_coordination/capacity.rs",
        "worth-store/recovery-coordination/capacity",
    ),
    (
        "../worth-store/src/physical_runtime/recovery_coordination/owner.rs",
        "worth-store/recovery-coordination/owner",
    ),
];

#[test]
fn delivered_phase_two_through_seven_facades_equal_the_destination_inventory() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest.ancestors().nth(4).expect("repository root");
    let expected = expected_surfaces(root);
    let mut actual = BTreeMap::<String, BTreeSet<String>>::new();
    for (relative, owner) in FILE_OWNERS {
        let path = manifest.join(relative);
        let source = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        actual.insert((*owner).to_owned(), public_surfaces(&source));
    }
    assert_eq!(actual, expected);
}

fn expected_surfaces(root: &Path) -> BTreeMap<String, BTreeSet<String>> {
    let source = std::fs::read_to_string(
        root.join("_docs/worth-store/physical-reconstruction-c8-public-api.csv"),
    )
    .expect("C.8 API inventory");
    let owners = FILE_OWNERS
        .iter()
        .map(|(_, owner)| *owner)
        .collect::<BTreeSet<_>>();
    let mut expected = owners
        .iter()
        .map(|owner| ((*owner).to_owned(), BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    for line in source.lines().skip(1) {
        let cells = line.split(',').collect::<Vec<_>>();
        assert_eq!(cells.len(), 6, "malformed API row {line}");
        if cells[0] == "destination"
            && matches!(
                cells[5],
                "phase-2" | "phase-3" | "phase-4" | "phase-5" | "phase-6" | "phase-7"
            )
            && owners.contains(cells[2])
        {
            assert!(
                expected
                    .get_mut(cells[2])
                    .expect("known owner")
                    .insert(cells[1].to_owned()),
                "duplicate destination surface {}",
                cells[1]
            );
        }
    }
    expected
}

fn public_surfaces(source: &str) -> BTreeSet<String> {
    let syntax = syn::parse_file(source).expect("parse facade owner");
    let mut surfaces = BTreeSet::new();
    for item in syntax.items {
        match item {
            syn::Item::Struct(item)
                if is_public(&item.vis) && !is_certification_only(&item.attrs) =>
            {
                surfaces.insert(item.ident.to_string());
            }
            syn::Item::Enum(item)
                if is_public(&item.vis) && !is_certification_only(&item.attrs) =>
            {
                surfaces.insert(item.ident.to_string());
            }
            syn::Item::Type(item)
                if is_public(&item.vis) && !is_certification_only(&item.attrs) =>
            {
                surfaces.insert(item.ident.to_string());
            }
            syn::Item::Impl(item) => collect_public_methods(&mut surfaces, item),
            syn::Item::Macro(item)
                if item
                    .mac
                    .path
                    .segments
                    .last()
                    .is_some_and(|segment| segment.ident == "binding_axes")
                    && item
                        .mac
                        .tokens
                        .to_string()
                        .contains("drift pub enum PhysicalRecoveryEntryBindingDrift") =>
            {
                surfaces.insert("PhysicalRecoveryEntryBindingDrift".to_owned());
            }
            _ => {}
        }
    }
    surfaces
}

fn collect_public_methods(surfaces: &mut BTreeSet<String>, item: syn::ItemImpl) {
    let syn::Type::Path(owner) = item.self_ty.as_ref() else {
        return;
    };
    let Some(owner) = owner
        .path
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
    else {
        return;
    };
    for method in item.items {
        if let syn::ImplItem::Fn(method) = method {
            if is_public(&method.vis) && !is_certification_only(&method.attrs) {
                surfaces.insert(format!("{owner}::{}", method.sig.ident));
            }
        }
    }
}

fn is_public(visibility: &syn::Visibility) -> bool {
    matches!(visibility, syn::Visibility::Public(_))
}

fn is_certification_only(attributes: &[syn::Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        attribute.path().is_ident("cfg")
            && matches!(
                &attribute.meta,
                syn::Meta::List(list)
                    if list.tokens.to_string().contains("certification-test-authority")
            )
    })
}
