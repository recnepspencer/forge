use worth_ui::facade::declaration::{
    UiDeclarationAspectDigest, UiDeclarationFamilyDigest, UiDeclarationIdentity,
    UiDeclarationPostureDigest, UiDeclarationStructuralDigest,
};

fn main() {
    let _ = UiDeclarationIdentity::new(
        unsafe { std::mem::transmute::<u64, UiDeclarationFamilyDigest>(1) },
        unsafe { std::mem::transmute::<u64, UiDeclarationAspectDigest>(2) },
        unsafe { std::mem::transmute::<u64, UiDeclarationStructuralDigest>(3) },
        unsafe { std::mem::transmute::<u64, UiDeclarationPostureDigest>(4) },
        "Worthd.identity",
    );
}
