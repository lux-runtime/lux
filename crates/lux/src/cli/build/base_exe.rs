use std::path::PathBuf;

use crate::standalone::metadata::CURRENT_EXE;

use super::{
    result::{BuildError, BuildResult},
    target::BuildTarget,
};

/**
    Discovers the path to the base executable to use for cross-compilation.

    For Lux micro-kernel, cross-compilation downloads are disabled.
    Only the current system target is supported. For other targets,
    manually place the binary in the cache directory.
*/
pub async fn get_or_download_base_executable(target: BuildTarget) -> BuildResult<PathBuf> {
    if target.is_current_system() {
        return Ok(CURRENT_EXE.to_path_buf());
    }

    // Check if manually cached
    if target.cache_path().exists() {
        return Ok(target.cache_path());
    }

    // Cross-compilation downloads disabled in Lux micro-kernel
    Err(BuildError::ReleaseTargetNotFound(target))
}
