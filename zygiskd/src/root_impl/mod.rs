// src/root_impl/mod.rs

//! A module for detecting and interfacing with the underlying root solution.
//!
//! It supports APatch, KernelSU, and Magisk. The active root solution is detected
//! once at startup and cached for all subsequent calls.

mod apatch;
mod kernelsu;
mod magisk;

use std::sync::OnceLock;

/// Represents the detected root solution on the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootImpl {
    /// No supported root solution was found.
    None,
    /// A supported root solution was found, but its version is too old.
    TooOld,
    /// Multiple conflicting root solutions were detected.
    Multiple,
    /// ShiroSU is the active root solution.
    ShiroSU,
    /// APatch is the active root solution.
    APatch,
    /// KernelSU is the active root solution.
    KernelSU,
    /// Magisk is the active root solution.
    Magisk,
}

/// A thread-safe, lazily initialized static variable holding the detected `RootImpl`.
static ROOT_IMPL: OnceLock<RootImpl> = OnceLock::new();

/// Probes the system to detect which root solution is active.
///
/// This function should only be called once. `get()` handles this logic automatically.
fn detect_root() -> RootImpl { RootImpl::ShiroSU }

/// Performs the root detection and caches the result.
/// This must be called once near startup before any other functions in this module are used.
pub fn setup() {
    ROOT_IMPL
        .set(detect_root())
        .expect("setup() should only be called once");
}

/// Returns a reference to the detected root implementation.
/// Panics if `setup()` has not been called first.
pub fn get() -> &'static RootImpl {
    ROOT_IMPL
        .get()
        .expect("root_impl::setup() must be called before get()")
}

/// Checks if a given UID has been granted root privileges by the active root manager.
pub fn uid_granted_root(uid: i32) -> bool {
    match get() {
        RootImpl::APatch => apatch::uid_granted_root(uid),
        RootImpl::KernelSU => kernelsu::uid_granted_root(uid),
        RootImpl::Magisk => magisk::uid_granted_root(uid),
        _ => false,
    }
}

/// Checks if mounts should be hidden (unmounted) for a given UID.
pub fn uid_should_umount(uid: i32) -> bool {
    match get() {
        RootImpl::APatch => apatch::uid_should_umount(uid),
        RootImpl::KernelSU => kernelsu::uid_should_umount(uid),
        RootImpl::Magisk => magisk::uid_should_umount(uid),
        _ => false,
    }
}

/// Checks if a given UID belongs to the active root manager application.
pub fn uid_is_manager(uid: i32) -> bool {
    match get() {
        RootImpl::APatch => apatch::uid_is_manager(uid),
        RootImpl::KernelSU => kernelsu::uid_is_manager(uid),
        RootImpl::Magisk => magisk::uid_is_manager(uid),
        _ => false,
    }
}
