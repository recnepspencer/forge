use worth_ui::facade::app::WorthUiPreparedApplicationAuthority;

fn extract(authority: WorthUiPreparedApplicationAuthority) {
    let _ = authority.into_canonical_artifact();
}

fn main() {}
