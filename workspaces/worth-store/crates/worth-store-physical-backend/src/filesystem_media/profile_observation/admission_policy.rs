use crate::filesystem_media::{
    FilesystemBackendProfile, FilesystemLocation, MediaCounterSnapshot, MediaQualificationDenial,
};

pub(in crate::filesystem_media) fn deny_profile(
    profile: &FilesystemBackendProfile,
    counters: MediaCounterSnapshot,
) -> Option<MediaQualificationDenial> {
    let counters = || Box::new(counters);
    if profile.location() == FilesystemLocation::Remote {
        return Some(MediaQualificationDenial::RemoteFilesystem {
            counters: counters(),
        });
    }
    if profile.location() == FilesystemLocation::Unknown {
        return Some(MediaQualificationDenial::UnknownFilesystem {
            counters: counters(),
        });
    }
    if profile.is_removable() {
        return Some(MediaQualificationDenial::RemovableFilesystem {
            counters: counters(),
        });
    }
    if profile.is_read_only() {
        return Some(MediaQualificationDenial::ReadOnlyFilesystem {
            counters: counters(),
        });
    }
    let filesystem = profile.filesystem_type().to_ascii_lowercase();
    if filesystem.is_empty() {
        return Some(MediaQualificationDenial::UnknownFilesystem {
            counters: counters(),
        });
    }
    if filesystem.contains("fuse") || filesystem == "9p" {
        return Some(MediaQualificationDenial::UserspaceFilesystem {
            filesystem: filesystem.into_boxed_str(),
            counters: counters(),
        });
    }
    None
}
