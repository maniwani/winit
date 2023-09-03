//! Contains traits with platform-specific methods in them.
//!
//! Only the module corresponding to the platform you're compiling to will be available.

#[cfg(any(android_platform, all(doc, not(doctest))))]
pub mod android;
#[cfg(any(ios_platform, all(doc, not(doctest))))]
pub mod ios;
#[cfg(any(macos_platform, all(doc, not(doctest))))]
pub mod macos;
#[cfg(any(orbital_platform, all(doc, not(doctest))))]
pub mod orbital;
#[cfg(any(x11_platform, wayland_platform, all(doc, not(doctest))))]
pub mod startup_notify;
#[cfg(any(wayland_platform, all(doc, not(doctest))))]
pub mod wayland;
#[cfg(any(wasm_platform, all(doc, not(doctest))))]
pub mod web;
#[cfg(any(windows_platform, all(doc, not(doctest))))]
pub mod windows;
#[cfg(any(x11_platform, all(doc, not(doctest))))]
pub mod x11;

#[cfg(any(
    windows_platform,
    macos_platform,
    android_platform,
    x11_platform,
    wayland_platform,
    all(doc, not(doctest)),
))]
pub mod run_ondemand;

#[cfg(any(
    windows_platform,
    macos_platform,
    android_platform,
    x11_platform,
    wayland_platform,
    all(doc, not(doctest)),
))]
pub mod pump_events;

#[cfg(any(
    windows_platform,
    macos_platform,
    x11_platform,
    wayland_platform,
    all(doc, not(doctest))
))]
pub mod modifier_supplement;

#[cfg(any(
    windows_platform,
    macos_platform,
    x11_platform,
    wayland_platform,
    all(doc, not(doctest))
))]
pub mod scancode;
