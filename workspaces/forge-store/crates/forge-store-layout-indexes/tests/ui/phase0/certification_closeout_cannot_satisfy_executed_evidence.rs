use forge_store_certification::s8_layout_closeout::S8LayoutCloseoutCourtroom;
use forge_store_layout_indexes::S8ExecutedAccessEvidence;

fn require_executed(_: S8ExecutedAccessEvidence) {}

fn main() {
    require_executed(S8LayoutCloseoutCourtroom);
}
