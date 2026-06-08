use forge_server::ForgeServerPreparedQueryHandoffIntent;

fn main() {
    let _ = ForgeServerPreparedQueryHandoffIntent {
        kind: panic!("sealed"),
        operation_name: "users.profile".to_string(),
    };
}
