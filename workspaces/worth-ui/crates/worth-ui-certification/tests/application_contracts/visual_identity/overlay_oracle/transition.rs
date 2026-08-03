pub(super) fn publish(
    context: &egui::Context,
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    pending: worth_ui::facade::inspection::UiPendingVisualOverlay,
    deadline: u64,
    now: u64,
) -> worth_ui::facade::inspection::UiPublishedVisualOverlay {
    settle_publication(context, session, pending, deadline, now)
        .0
        .expect("overlay publication presents a distinct successor")
}

pub(super) fn publish_with_output(
    context: &egui::Context,
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    pending: worth_ui::facade::inspection::UiPendingVisualOverlay,
    deadline: u64,
    now: u64,
) -> (
    worth_ui::facade::inspection::UiPublishedVisualOverlay,
    egui::FullOutput,
) {
    let (outcome, output) = settle_publication(context, session, pending, deadline, now);
    (
        outcome.expect("overlay publication presents a distinct successor"),
        output,
    )
}

pub(super) fn fail_publication(
    context: &egui::Context,
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    pending: worth_ui::facade::inspection::UiPendingVisualOverlay,
    deadline: u64,
    now: u64,
) -> worth_ui::facade::inspection::UiVisualOverlayPublicationFailure {
    settle_publication(context, session, pending, deadline, now)
        .0
        .expect_err("the elapsed deadline cannot claim overlay publication")
}

pub(super) fn clear(
    context: &egui::Context,
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    published: worth_ui::facade::inspection::UiPublishedVisualOverlay,
    deadline: u64,
    now: u64,
) {
    settle_clear(context, session, published, deadline, now)
        .0
        .expect("overlay clear presents a distinct successor");
}

pub(super) fn clear_with_output(
    context: &egui::Context,
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    published: worth_ui::facade::inspection::UiPublishedVisualOverlay,
    deadline: u64,
    now: u64,
) -> (
    worth_ui::facade::inspection::UiClearedVisualOverlayReceipt,
    egui::FullOutput,
) {
    let (outcome, output) = settle_clear(context, session, published, deadline, now);
    (
        outcome.expect("overlay clear presents a distinct successor"),
        output,
    )
}

pub(super) fn fail_clear(
    context: &egui::Context,
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    published: worth_ui::facade::inspection::UiPublishedVisualOverlay,
    deadline: u64,
    now: u64,
) -> worth_ui::facade::inspection::UiVisualOverlayClearFailure {
    settle_clear(context, session, published, deadline, now)
        .0
        .expect_err("the elapsed deadline cannot claim overlay clear")
}

fn settle_publication(
    context: &egui::Context,
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    pending: worth_ui::facade::inspection::UiPendingVisualOverlay,
    deadline: u64,
    now: u64,
) -> (
    Result<
        worth_ui::facade::inspection::UiPublishedVisualOverlay,
        worth_ui::facade::inspection::UiVisualOverlayPublicationFailure,
    >,
    egui::FullOutput,
) {
    let mut pending = Some(pending);
    let mut outcome = None;
    let output = context.run_ui(super::super::raw_input(), |_| {
        outcome = Some(
            session.present_visual_overlay(
                pending
                    .take()
                    .expect("one pending handle enters presentation"),
                deadline,
                now,
            ),
        );
    });
    (
        outcome.expect("egui callback settles overlay publication"),
        output,
    )
}

fn settle_clear(
    context: &egui::Context,
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    published: worth_ui::facade::inspection::UiPublishedVisualOverlay,
    deadline: u64,
    now: u64,
) -> (
    Result<
        worth_ui::facade::inspection::UiClearedVisualOverlayReceipt,
        worth_ui::facade::inspection::UiVisualOverlayClearFailure,
    >,
    egui::FullOutput,
) {
    let mut published = Some(published);
    let mut outcome = None;
    let output = context.run_ui(super::super::raw_input(), |_| {
        outcome = Some(
            session.clear_visual_overlay(
                published
                    .take()
                    .expect("one published handle enters clear presentation"),
                deadline,
                now,
            ),
        );
    });
    (
        outcome.expect("egui callback settles overlay clear"),
        output,
    )
}
