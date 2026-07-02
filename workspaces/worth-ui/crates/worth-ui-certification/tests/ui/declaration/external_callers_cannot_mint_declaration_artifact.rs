use worth_ui::facade::declaration::{
    UiDeclarationArtifact, UiDeclarationDigestProjection, UiDeclarationIdentity,
    UiDeclarationProvenance,
};

fn main() {
    let _ = UiDeclarationArtifact::new(
        unsafe { std::mem::MaybeUninit::<UiDeclarationIdentity>::zeroed().assume_init() },
        unsafe { std::mem::MaybeUninit::<UiDeclarationDigestProjection>::zeroed().assume_init() },
        unsafe { std::mem::MaybeUninit::<UiDeclarationProvenance>::zeroed().assume_init() },
    );
}
