pub struct MinimalTopologyLabels {
    pub model: String,
    pub body: String,
    pub lump: String,
    pub region: String,
    pub shell: String,
    pub face: String,
    pub outer_loop: String,
    pub wire: String,
    pub half_edge: String,
    pub edge: String,
    pub vertex: String,
}

impl MinimalTopologyLabels {
    pub fn new(stem: &str) -> Self {
        Self {
            model: format!("{stem}.model"),
            body: format!("{stem}.body"),
            lump: format!("{stem}.lump"),
            region: format!("{stem}.region"),
            shell: format!("{stem}.shell"),
            face: format!("{stem}.face"),
            outer_loop: format!("{stem}.outer_loop"),
            wire: format!("{stem}.wire"),
            half_edge: format!("{stem}.half_edge"),
            edge: format!("{stem}.edge"),
            vertex: format!("{stem}.vertex"),
        }
    }
}
