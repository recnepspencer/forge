use worth_spatial::facade::projection_workload::ProjectedPlanarWorkload;

fn main() {
    let loose_points = vec![(0.0, 0.0), (1.0, 0.0)];
    consume_projected_workload(loose_points);
}

fn consume_projected_workload(_workload: ProjectedPlanarWorkload) {}
