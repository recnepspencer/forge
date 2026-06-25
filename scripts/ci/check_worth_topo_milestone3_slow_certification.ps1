Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

cargo test -p worth-topo --features slow-certification topology_operator_closeout --lib -- --format terse
