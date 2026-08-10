use std::path::PathBuf;

use worth_ui_dsl::{
    certification_support::with_unsupported_protocol, WorthUiAuthoredSourceInput,
    WorthUiDslCompiler,
};

use crate::facade::WorthUi;

use super::{prepare_semantic_handoff, WorthUiSemanticHandoffPreparationStop};

#[test]
fn unsupported_protocol_stops_before_candidate_material_can_exist() {
    let capability_app = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("capability authority should prepare");
    let package = WorthUiDslCompiler::compile_source(
        WorthUiAuthoredSourceInput::rooted_at(PathBuf::from("workspace"))
            .with_module("app/main.wui", "component Dashboard {}"),
    )
    .expect("otherwise valid source should seal");
    let expected_identity = package.identity().clone();
    let unsupported = with_unsupported_protocol(package);
    let unsupported_protocol = unsupported.protocol();

    let denial = match prepare_semantic_handoff(unsupported, capability_app.capabilities()) {
        Ok(_) => panic!("unsupported package protocol must stop before runtime lowering"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.stop(),
        WorthUiSemanticHandoffPreparationStop::UnsupportedProtocol
    );
    assert_eq!(denial.handoff().identity(), &expected_identity);
    assert_eq!(denial.handoff().protocol(), unsupported_protocol);
    assert!(!denial.handoff().protocol().is_current());
}
