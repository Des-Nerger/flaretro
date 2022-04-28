#![feature(c_variadic)]
#![warn(elided_lifetimes_in_paths)]

use {
	core::{
		ffi::{c_void, VaList},
		ptr::{copy, null, null_mut},
	},
	libc::FILE,
	rust_libretro_sys::{
		retro_audio_sample_batch_t, retro_audio_sample_t, retro_environment_t, retro_game_geometry,
		retro_game_info, retro_input_poll_t, retro_input_state_t, retro_log_callback,
		retro_log_level::{self, RETRO_LOG_ERROR},
		retro_pixel_format::{self, RETRO_PIXEL_FORMAT_XRGB8888},
		retro_system_av_info, retro_system_info, retro_system_timing, retro_video_refresh_t, size_t,
		RETRO_API_VERSION, RETRO_ENVIRONMENT_GET_LOG_INTERFACE, RETRO_ENVIRONMENT_SET_PIXEL_FORMAT,
		RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME, RETRO_REGION_PAL,
	},
	std::os::raw::{c_char, c_int, c_uint},
};

extern "C" {
	static stderr: *mut FILE;
	fn vfprintf(_: *mut FILE, _: *const c_char, _: VaList<'_, '_>) -> c_int;
}

#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn fallback_log(_level: retro_log_level, fmt: *const c_char, mut args: ...) {
	vfprintf(stderr, fmt, args.as_va_list());
}

const VIDEO_WIDTH: u32 = 1280;
const VIDEO_HEIGHT: u32 = 720;
type RetroLogPrintf = unsafe extern "C" fn(_: retro_log_level, _: *const c_char, ...);

static mut LOG_CB: RetroLogPrintf = fallback_log;
static mut ENVIRON_CB: retro_environment_t = None;
static mut FRAME_BUF: Vec<u32> = Vec::new();
static mut VIDEO_CB: retro_video_refresh_t = None;
static mut INPUT_POLL_CB: retro_input_poll_t = None;
static mut INPUT_STATE_CB: retro_input_state_t = None;
static mut AUDIO_CB: retro_audio_sample_t = None;
static mut AUDIO_BATCH_CB: retro_audio_sample_batch_t = None;

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
	FRAME_BUF.resize((VIDEO_WIDTH * VIDEO_HEIGHT) as usize, 0);
}

#[no_mangle]
pub unsafe extern "C" fn retro_deinit() {
	FRAME_BUF.shrink_to(0);
}

#[no_mangle]
pub unsafe extern "C" fn retro_api_version() -> c_uint {
	RETRO_API_VERSION
}

#[no_mangle]
pub unsafe extern "C" fn retro_get_system_info(info: *mut retro_system_info) {
	const NAME: &'static str = env!("CARGO_PKG_NAME");
	const NAME_LEN: usize = NAME.len();
	static mut CNAME: [c_char; NAME_LEN + 1] = [0; NAME_LEN + 1];
	const VER: &'static str = env!("CARGO_PKG_VERSION");
	const VER_LEN: usize = VER.len();
	static mut CVER: [c_char; VER_LEN + 1] = [0; VER_LEN + 1];
	if CNAME[0] == 0 {
		copy(
			NAME.as_ptr() as *const c_char,
			&mut CNAME as *mut c_char,
			NAME_LEN,
		);
		copy(
			VER.as_ptr() as *const c_char,
			&mut CVER as *mut c_char,
			VER_LEN,
		);
	}
	*info = retro_system_info {
		library_name: &CNAME as *const c_char,
		library_version: &CVER as *const c_char,
		valid_extensions: null(),
		need_fullpath: false,
		block_extract: true,
	};
}

#[no_mangle]
pub unsafe extern "C" fn retro_get_system_av_info(info: *mut retro_system_av_info) {
	*info = retro_system_av_info {
		timing: retro_system_timing {
			fps: 1.0,
			sample_rate: 0.0,
		},
		geometry: retro_game_geometry {
			base_width: VIDEO_WIDTH,
			base_height: VIDEO_HEIGHT,
			max_width: VIDEO_WIDTH,
			max_height: VIDEO_HEIGHT,
			aspect_ratio: 0.0,
		},
	};
}

#[no_mangle]
pub unsafe extern "C" fn retro_set_environment(cb: retro_environment_t) {
	ENVIRON_CB = cb;
	let environ_cb = ENVIRON_CB.unwrap();
	environ_cb(
		RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME,
		&true as *const bool as *mut c_void,
	);
	let mut logging = retro_log_callback { log: None };
	if environ_cb(
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
pub unsafe extern "C" fn retro_set_audio_sample(cb: retro_audio_sample_t) {
	AUDIO_CB = cb;
}

#[no_mangle]
pub unsafe extern "C" fn retro_set_audio_sample_batch(cb: retro_audio_sample_batch_t) {
	AUDIO_BATCH_CB = cb;
}

#[no_mangle]
pub unsafe extern "C" fn retro_set_input_poll(cb: retro_input_poll_t) {
	INPUT_POLL_CB = cb;
}

#[no_mangle]
pub unsafe extern "C" fn retro_set_input_state(cb: retro_input_state_t) {
	INPUT_STATE_CB = cb;
}

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
	if !ENVIRON_CB.unwrap()(
		RETRO_ENVIRONMENT_SET_PIXEL_FORMAT,
		&RETRO_PIXEL_FORMAT_XRGB8888 as *const retro_pixel_format as *mut c_void,
	) {
		log_cb!(RETRO_LOG_ERROR, "XRGB8888 is not supported.\n\0");
		return false;
	}
	true
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
