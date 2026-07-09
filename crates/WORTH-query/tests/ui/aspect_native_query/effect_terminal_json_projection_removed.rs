use worth_query::facade::{WorthQueryEffectDelivery, WorthQueryEffectPayload};

fn payload_projection(payload: WorthQueryEffectPayload) {
    let _ = payload.terminal_json_projection();
}

fn delivery_projection(delivery: WorthQueryEffectDelivery) {
    let _ = delivery.terminal_json_payload_projection();
}

fn main() {}
