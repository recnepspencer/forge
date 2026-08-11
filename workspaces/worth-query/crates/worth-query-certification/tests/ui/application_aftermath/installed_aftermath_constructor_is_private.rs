//! Installed aftermath struct literals are not constructible outside installation.

use worth_query_host::facade::domain::WorthQueryInstalledAftermathContract;

fn main() {
    let _ = WorthQueryInstalledAftermathContract {};
}
