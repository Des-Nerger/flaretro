#![feature(c_variadic)]

use {
	core::{
		ffi::{c_void, VaList, VaListImpl},
		ptr::null_mut,
	},
	rust_libretro_sys::{
		retro_audio_sample_batch_t, retro_audio_sample_t, retro_environment_t, retro_game_info,
		retro_input_poll_t, retro_input_state_t, retro_log_level::RETRO_LOG_INFO, retro_system_av_info,
		retro_system_info, retro_video_refresh_t, size_t, RETRO_API_VERSION, RETRO_REGION_PAL,
	},
	std::os::raw::{c_char, c_int, c_uint},
};

extern "C" {
	static mut stderr: *mut c_void;
	fn vfprintf(_: *mut c_void, _: *const c_char, _: VaList) -> c_int;
}

unsafe extern "C" fn fallback_log(_level: u32, fmt: *const c_char, args: ...) {
	let mut va: VaListImpl;
	va = args.clone();
	vfprintf(stderr, fmt, va.as_va_list());
}

#[no_mangle]
pub unsafe extern "C" fn retro_init() {
	fallback_log(
		RETRO_LOG_INFO as u32,
		b"Hello, log\n\x00" as *const u8 as *const i8,
	);
}

#[no_mangle]
pub unsafe extern "C" fn retro_deinit() {}

#[no_mangle]
pub unsafe extern "C" fn retro_api_version() -> c_uint {
	RETRO_API_VERSION
}

#[no_mangle]
pub unsafe extern "C" fn retro_get_system_info(_info: *mut retro_system_info) {}

#[no_mangle]
pub unsafe extern "C" fn retro_get_system_av_info(_info: *mut retro_system_av_info) {}

#[no_mangle]
pub unsafe extern "C" fn retro_set_environment(_cb: retro_environment_t) {}

#[no_mangle]
pub unsafe extern "C" fn retro_set_video_refresh(_cb: retro_video_refresh_t) {}

#[no_mangle]
pub unsafe extern "C" fn retro_set_audio_sample(_cb: retro_audio_sample_t) {}

#[no_mangle]
pub unsafe extern "C" fn retro_set_audio_sample_batch(_cb: retro_audio_sample_batch_t) {}

#[no_mangle]
pub unsafe extern "C" fn retro_set_input_poll(_cb: retro_input_poll_t) {}

#[no_mangle]
pub unsafe extern "C" fn retro_set_input_state(_cb: retro_input_state_t) {}

#[no_mangle]
pub unsafe extern "C" fn retro_set_controller_port_device(_port: c_uint, _device: c_uint) {}

#[no_mangle]
pub unsafe extern "C" fn retro_reset() {}

#[no_mangle]
pub unsafe extern "C" fn retro_run() {}

#[no_mangle]
pub unsafe extern "C" fn retro_serialize_size() -> size_t {
	0
}

#[no_mangle]
pub unsafe extern "C" fn retro_serialize(_data: *mut c_void, _size: size_t) -> bool {
	false
}

#[no_mangle]
pub unsafe extern "C" fn retro_unserialize(_data: *const c_void, _size: size_t) -> bool {
	false
}

#[no_mangle]
pub unsafe extern "C" fn retro_cheat_reset() {}

#[no_mangle]
pub unsafe extern "C" fn retro_cheat_set(_index: c_uint, _enabled: bool, _code: *const c_char) {}

#[no_mangle]
pub unsafe extern "C" fn retro_load_game(_info: *const retro_game_info) -> bool {
	false
}

#[no_mangle]
pub unsafe extern "C" fn retro_load_game_special(
	_type: c_uint,
	_info: *const retro_game_info,
	_num: size_t,
) -> bool {
	false
}

#[no_mangle]
pub unsafe extern "C" fn retro_unload_game() {}

#[no_mangle]
pub unsafe extern "C" fn retro_get_region() -> c_uint {
	RETRO_REGION_PAL
}

#[no_mangle]
pub unsafe extern "C" fn retro_get_memory_data(_id: c_uint) -> *mut c_void {
	null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn retro_get_memory_size(_id: c_uint) -> size_t {
	0
}
