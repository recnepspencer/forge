use worth_ui::facade::app::{
    WorthUiActiveApplicationSession, WorthUiHostMeasurementSessionInput,
};
use worth_ui::facade::host::WorthUiOperationalHostAdapter;

fn submit_raw_adapter<Adapter: WorthUiOperationalHostAdapter>(
    session: &mut WorthUiActiveApplicationSession,
    adapter: &Adapter,
    input: WorthUiHostMeasurementSessionInput,
) {
    session.execute_framework_turn(|turn| {
        turn.host_measurement(|source| {
            let _ = source.collect_and_submit(adapter, input);
        });
    });
}

fn main() {}
