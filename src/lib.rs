#![feature(c_variadic)]
#![warn(elided_lifetimes_in_paths)]

use {
	core::{
		ffi::{c_void, VaList},
		mem::zeroed,
		ptr::{copy, null, null_mut},
	},
	rust_libretro_sys::{
		retro_audio_sample_batch_t, retro_audio_sample_t, retro_environment_t, retro_game_info,
		retro_input_poll_t, retro_input_state_t, retro_log_callback,
		retro_log_level::{self, RETRO_LOG_ERROR, RETRO_LOG_INFO},
		retro_system_av_info, retro_system_info, retro_video_refresh_t, size_t, RETRO_API_VERSION,
		RETRO_ENVIRONMENT_GET_LOG_INTERFACE, RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME, RETRO_REGION_PAL,
	},
	std::os::raw::{c_char, c_int, c_uint},
};

extern "C" {
	static stderr: *const c_void;
	fn vfprintf(_: *const c_void, _: *const c_char, _: VaList<'_, '_>) -> c_int;
}

#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn fallback_log(_level: retro_log_level, fmt: *const c_char, mut args: ...) {
	vfprintf(stderr, fmt, args.as_va_list());
}

type RetroLogPrintf = unsafe extern "C" fn(_: retro_log_level, _: *const c_char, ...);

static mut LOG_CB: RetroLogPrintf = fallback_log;
static mut ENVIRON_CB: retro_environment_t = None;
static mut VIDEO_CB: retro_video_refresh_t = None;
static mut INPUT_POLL_CB: retro_input_poll_t = None;

macro_rules! log_cb {
	( $level:expr, $fmt:expr $(, $arg:expr)* $(,)? ) => {
		LOG_CB(
			$level,
			$fmt.as_ptr() as *const c_char,
			$( $arg ),*
		);
	};
}

#[no_mangle]
pub unsafe extern "C" fn retro_init() {
	log_cb!(RETRO_LOG_INFO, "Hello, log\n\0");
}

#[no_mangle]
pub unsafe extern "C" fn retro_deinit() {}

#[no_mangle]
pub unsafe extern "C" fn retro_api_version() -> c_uint {
	RETRO_API_VERSION
}

#[no_mangle]
pub unsafe extern "C" fn retro_get_system_info(info: *mut retro_system_info) {
	const NAME: &str = env!("CARGO_PKG_NAME");
	const NAME_LEN: usize = NAME.len();
	static mut CNAME: [c_char; NAME_LEN + 1] = [0; NAME_LEN + 1];
	if CNAME[0] == 0 {
		copy(
			NAME.as_ptr() as *const c_char,
			&mut CNAME as *mut c_char,
			NAME_LEN,
		)
	}
	const VER: &str = env!("CARGO_PKG_VERSION");
	const VER_LEN: usize = VER.len();
	static mut CVER: [c_char; VER_LEN + 1] = [0; VER_LEN + 1];
	if CVER[0] == 0 {
		copy(
			VER.as_ptr() as *const c_char,
			&mut CVER as *mut c_char,
			VER_LEN,
		)
	}
	*info = retro_system_info {
		library_name: &CNAME as *const c_char,
		library_version: &CVER as *const c_char,
		valid_extensions: null(),
		need_fullpath: false,
		block_extract: true,
		..zeroed()
	};
}

#[no_mangle]
pub unsafe extern "C" fn retro_get_system_av_info(_info: *mut retro_system_av_info) {}

#[no_mangle]
pub unsafe extern "C" fn retro_set_environment(cb: retro_environment_t) {
	ENVIRON_CB = cb;
	let ecb = ENVIRON_CB.unwrap();
	ecb(
		RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME,
		&true as *const bool as *mut c_void,
	);
	let mut logging = retro_log_callback { log: None };
	if ecb(
		RETRO_ENVIRONMENT_GET_LOG_INTERFACE,
		&mut logging as *mut retro_log_callback as *mut c_void,
	) {
		LOG_CB = logging.log.unwrap();
	}
}

#[no_mangle]
pub unsafe extern "C" fn retro_set_video_refresh(cb: retro_video_refresh_t) {
	VIDEO_CB = cb;
}

#[no_mangle]
pub unsafe extern "C" fn retro_set_audio_sample(_cb: retro_audio_sample_t) {}

#[no_mangle]
pub unsafe extern "C" fn retro_set_audio_sample_batch(_cb: retro_audio_sample_batch_t) {}

#[no_mangle]
pub unsafe extern "C" fn retro_set_input_poll(cb: retro_input_poll_t) {
	INPUT_POLL_CB = cb;
}

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
pub unsafe extern "C" fn retro_load_game(info: *const retro_game_info) -> bool {
	if info != null() {
		log_cb!(
			RETRO_LOG_ERROR,
			"Content file is given, but this core doesn't support any !!!\n\0",
		);
		return false;
	}
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
