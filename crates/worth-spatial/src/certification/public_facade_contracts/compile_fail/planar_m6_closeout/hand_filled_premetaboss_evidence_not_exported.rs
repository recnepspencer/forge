use worth_spatial::facade::planar_m6_closeout::{
    M6PremetabossEvidenceRow, M6PremetabossFamily,
};

fn main() {
    let _ = M6PremetabossEvidenceRow::passed(
        M6PremetabossFamily::BooleanReadinessFinalBoss,
        "hand-filled-mb-evidence",
    );
}
