pub const VALIDATION_SAMPLE_MODULE_PATH: &str = "validation/header.wui";

pub const VALIDATION_SAMPLE_SOURCE: &str = r#"
app ValidationHeaderApp {
    theme ValidationHeaderTheme
    workspace ValidationHeaderWorkspace
}

workspace ValidationHeaderWorkspace {
    shell {
        topbar ValidationHeaderTopbar
        rail ValidationHeaderRail
        page_host ValidationHeaderPageHost
        inspector ValidationHeaderInspector
        status ValidationHeaderStatus
        overlays []
        toasts ValidationHeaderToasts
    }

    pages [HeaderProofPage]
}

page HeaderProofPage {
    title "Header Proof"
    runtime HeaderProofRuntime
    layout HeaderProofLayout
    content HeaderProofContent
}

runtime HeaderProofRuntime {}

layout HeaderProofLayout {
    column {
        row height fill {
            slot proof
        }
    }
}

content HeaderProofContent {
    proof -> validation.surface.header.proof
}

appearance ValidationHeaderTheme {}
"#;
