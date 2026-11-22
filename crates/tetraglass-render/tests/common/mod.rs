//! Shared helpers for `tetraglass-render` integration tests.

use std::path::PathBuf;
use std::sync::OnceLock;

use tetraglass_render::HeadlessContext;

/// Returns the workspace root directory.
pub fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root")
        .to_path_buf()
}

/// Returns a single process-wide [`HeadlessContext`] shared across parallel tests in this binary.
///
/// Creating independent `wgpu::Instance` / `HeadlessContext` instances concurrently on Linux
/// Vulkan (`libvulkan1` with Mesa lavapipe) triggers a `SIGSEGV` inside `libvulkan.so.1` when
/// one thread's `vkEnumeratePhysicalDevices` (`dlclose` of unused ICDs) races with another
/// thread's `vkSetDebugUtilsObjectNameEXT` during pipeline or resource creation (#5).
/// Initializing one context once via [`OnceLock`] avoids concurrent Vulkan instance/adapter
/// enumeration while keeping parallel test execution fast and deterministic.
pub fn shared_headless_context() -> &'static HeadlessContext {
    static CONTEXT: OnceLock<HeadlessContext> = OnceLock::new();
    CONTEXT.get_or_init(|| HeadlessContext::new().expect("create shared headless wgpu context"))
}
