use livesplit_core::HotkeySystem;
use shared_timer::OwnedSharedTimer;
use super::{acc, alloc, own, own_drop};
use std::ptr;

pub type OwnedHotkeySystem = *mut HotkeySystem;
pub type NullableOwnedHotkeySystem = OwnedHotkeySystem;

#[no_mangle]
pub unsafe extern "C" fn HotkeySystem_new(
    shared_timer: OwnedSharedTimer,
) -> NullableOwnedHotkeySystem {
    if let Ok(hotkey_system) = HotkeySystem::new(own(shared_timer)) {
        alloc(hotkey_system)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn HotkeySystem_drop(this: OwnedHotkeySystem) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn HotkeySystem_deactivate(this: *const HotkeySystem) {
    acc(this).deactivate();
}

#[no_mangle]
pub unsafe extern "C" fn HotkeySystem_activate(this: *const HotkeySystem) {
    acc(this).activate();
}
