use super::comparison::WorthUiQueryBindingAuthorityDrift;

pub(super) fn authority_drifts(
    active: &super::evidence::WorthUiQueryBindingEvidence,
    candidate: &super::evidence::WorthUiQueryBindingEvidence,
) -> Vec<WorthUiQueryBindingAuthorityDrift> {
    installed_authority_drifts(
        active.installed_reference(),
        candidate.installed_reference(),
        active
            .settled()
            .map(|settled| settled.query_binding_identity()),
        candidate
            .settled()
            .map(|settled| settled.query_binding_identity()),
        active
            .exact_live_resource()
            .map(|evidence| evidence.installed_reference()),
    )
}

fn installed_authority_drifts(
    active_reference: Option<&worth_ui_query_binding::WorthUiInstalledQueryBindingReference>,
    candidate_reference: Option<&worth_ui_query_binding::WorthUiInstalledQueryBindingReference>,
    active_binding_identity: Option<&str>,
    candidate_binding_identity: Option<&str>,
    active_exact_live_reference: Option<
        &worth_ui_query_binding::WorthUiInstalledQueryBindingReference,
    >,
) -> Vec<WorthUiQueryBindingAuthorityDrift> {
    let mut drifts = Vec::new();
    let (active_reference, candidate_reference) = match (active_reference, candidate_reference) {
        (Some(active), Some(candidate)) if active == candidate => (active, candidate),
        _ => {
            drifts.push(WorthUiQueryBindingAuthorityDrift::InstalledAuthority);
            return drifts;
        }
    };
    if !active_reference.installation_is_current() || !candidate_reference.installation_is_current()
    {
        drifts.push(WorthUiQueryBindingAuthorityDrift::InstallationCurrentness);
    }
    let binding_identity_preserves = match (active_binding_identity, candidate_binding_identity) {
        (Some(active), Some(candidate)) => active == candidate,
        (Some(_), None) => true,
        (None, None) => active_exact_live_reference == Some(active_reference),
        (None, Some(_)) => false,
    };
    if !binding_identity_preserves {
        drifts.push(WorthUiQueryBindingAuthorityDrift::BindingIdentity);
    }
    drifts
}

#[cfg(test)]
mod tests {
    use super::{installed_authority_drifts, WorthUiQueryBindingAuthorityDrift};
    use worth_ui_query_binding::WorthUiQueryBindingPlan;

    #[test]
    fn missing_or_foreign_query_authority_never_preserves() {
        assert_eq!(
            installed_authority_drifts(None, None, None, None, None),
            vec![WorthUiQueryBindingAuthorityDrift::InstalledAuthority]
        );
        let active = reference("replacement-authority-active");
        let foreign = reference("replacement-authority-foreign");
        assert_eq!(
            installed_authority_drifts(Some(&active), Some(&foreign), None, None, None),
            vec![WorthUiQueryBindingAuthorityDrift::InstalledAuthority]
        );
    }

    #[test]
    fn exact_current_authority_still_compares_query_minted_binding_identity() {
        let reference = reference("replacement-authority-exact");
        assert!(installed_authority_drifts(
            Some(&reference),
            Some(&reference),
            Some("query-binding-a"),
            Some("query-binding-a"),
            None,
        )
        .is_empty());
        assert_eq!(
            installed_authority_drifts(Some(&reference), Some(&reference), None, None, None),
            vec![WorthUiQueryBindingAuthorityDrift::BindingIdentity]
        );
        assert_eq!(
            installed_authority_drifts(
                Some(&reference),
                Some(&reference),
                Some("query-binding-a"),
                Some("query-binding-b"),
                None,
            ),
            vec![WorthUiQueryBindingAuthorityDrift::BindingIdentity]
        );
    }

    fn reference(label: &str) -> worth_ui_query_binding::WorthUiInstalledQueryBindingReference {
        let installed =
            worth_ui_query_binding::certification::worth_ui_installed_test_domain(label);
        let view = installed
            .measurement_view("replacement.measurements")
            .expect("measurement view installs");
        let view_identity = view.definition().identity().clone();
        WorthUiQueryBindingPlan::default()
            .register_view(view)
            .expect("view registers")
            .resolve_definition(
                &view_identity,
                worth_ui_query_binding::WorthUiQueryViewShape::Collection,
            )
            .expect("reference resolves")
    }
}
