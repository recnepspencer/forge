use worth_spatial::facade::blocker_provenance::WorkloadBlockerProvenance;
use worth_spatial::facade::open_planar_posture::OpenPlanarPostureError;

fn main() {
    let error = Option::<OpenPlanarPostureError>::None.unwrap();
    let _provenance = WorkloadBlockerProvenance::unsupported_surface_open_topology_mismatch(&error);
}
