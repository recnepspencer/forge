fn main() {
    let _ = forge_foundational::FoundationalSelectedNodeScopeLocator::new(
        forge_foundational::FoundationalBranchId::new("feature").unwrap(),
        forge_foundational::FoundationalBranchId::new("main").unwrap(),
        "gear",
    );

    let _ = forge_foundational::FoundationalSelectedAspectScopeLocator::new(
        forge_foundational::FoundationalBranchId::new("feature").unwrap(),
        forge_foundational::FoundationalBranchId::new("main").unwrap(),
        "gear.teeth",
    );
}
