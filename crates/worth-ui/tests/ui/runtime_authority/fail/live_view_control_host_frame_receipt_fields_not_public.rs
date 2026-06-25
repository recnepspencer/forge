use worth_ui::facade::WorthUiLiveViewControlHostFrameReceipt;

fn main() {
    let _forged = WorthUiLiveViewControlHostFrameReceipt {
        subject: panic!("fixture only checks receipt field privacy"),
        kind: panic!("fixture only checks receipt field privacy"),
        control_id: "first_name_input".to_owned(),
        label: "First name".to_owned(),
        value_text: String::new(),
        options: Vec::new(),
        editability: panic!("fixture only checks receipt field privacy"),
        participation: None,
        style: panic!("fixture only checks receipt field privacy"),
        consumed_facts: Vec::new(),
        frame_digest: 1,
    };
}
