#![warn(clippy::pedantic, elided_lifetimes_in_paths, explicit_outlives_requirements)]

mod glad;

use {
	core::{
		ffi::c_void,
		ptr::{copy, null, null_mut},
	},
	glad::gl::*,
	libc::{fprintf, FILE},
	rust_libretro_sys::{retro_hw_context_type::*, retro_log_level::*, retro_pixel_format::*, *},
	std::os::raw::{c_char, c_uint},
};

extern "C" {
	static stderr: *mut FILE;
}

#[repr(C)]
struct RetroHwRenderCallback {
	context_type: retro_hw_context_type,
	context_reset: unsafe extern "C" fn(),
	get_current_framebuffer: unsafe extern "C" fn() -> usize,
	get_proc_address: unsafe extern "C" fn(sym: *const c_char) -> *const c_void,
	depth: bool,
	stencil: bool,
	bottom_left_origin: bool,
	version_major: c_uint,
	version_minor: c_uint,
	cache_context: bool,
	context_destroy: unsafe extern "C" fn(),
	debug_context: bool,
}

unsafe extern "C" fn environ_cb(_: c_uint, _: *mut c_void) -> bool {
	unimplemented!()
}
unsafe extern "C" fn video_cb(_: *const c_void, _: c_uint, _: c_uint, _: size_t) {
	unimplemented!()
}
unsafe extern "C" fn input_poll_cb() {
	unimplemented!()
}
unsafe extern "C" fn input_state_cb(_: c_uint, _: c_uint, _: c_uint, _: c_uint) -> i16 {
	unimplemented!()
}
unsafe extern "C" fn audio_cb(_: i16, _: i16) {
	unimplemented!()
}
unsafe extern "C" fn audio_batch_cb(_: *const i16, _: size_t) -> size_t {
	unimplemented!()
}
unsafe extern "C" fn get_current_framebuffer() -> usize {
	unimplemented!()
}
unsafe extern "C" fn get_proc_address(_: *const c_char) -> *const c_void {
	unimplemented!()
}

static mut LOG_CB: retro_log_printf_t = None;
static mut ENVIRON_CB: unsafe extern "C" fn(c_uint, *mut c_void) -> bool = environ_cb;
static mut VIDEO_CB: unsafe extern "C" fn(*const c_void, c_uint, c_uint, size_t) = video_cb;
static mut INPUT_POLL_CB: unsafe extern "C" fn() = input_poll_cb;
static mut INPUT_STATE_CB: unsafe extern "C" fn(c_uint, c_uint, c_uint, c_uint) -> i16 =
	input_state_cb;
static mut AUDIO_CB: unsafe extern "C" fn(i16, i16) = audio_cb;
static mut AUDIO_BATCH_CB: unsafe extern "C" fn(*const i16, size_t) -> size_t = audio_batch_cb;
static mut HW_RENDER: RetroHwRenderCallback = RetroHwRenderCallback {
	context_type: RETRO_HW_CONTEXT_OPENGL,
	version_major: 2,
	version_minor: 1,
	depth: false,
	stencil: false,
	bottom_left_origin: true,
	cache_context: false,
	debug_context: false,
	get_current_framebuffer,
	get_proc_address,
	context_reset,
	context_destroy,
};

macro_rules! ptr {
	($e: expr) => {
		$e as *const _ as *const _
	};
}

macro_rules! log_cb {
	( $level:expr, $fmt:expr $(, $arg:expr)* $(,)? ) => {
		{
			const FMT_PTR: *const c_char = ptr!($fmt);
			if let Some(log_cb) = LOG_CB {
				log_cb($level, FMT_PTR, $( $arg ),*);
			} else {
				fprintf(stderr, FMT_PTR, $( $arg ),*);
			}
		}
	};
}

const VIDEO_WIDTH: u32 = 683;
const VIDEO_HEIGHT: u32 = 383;

unsafe extern "C" fn context_reset() {
	gl_load(|e| (HW_RENDER.get_proc_address)(ptr!(e)));
}
unsafe extern "C" fn context_destroy() {}

#[no_mangle]
pub unsafe extern "C" fn retro_init() {}

#[no_mangle]
pub unsafe extern "C" fn retro_deinit() {}

#[no_mangle]
pub unsafe extern "C" fn retro_api_version() -> c_uint {
	RETRO_API_VERSION
}

#[no_mangle]
pub unsafe extern "C" fn retro_get_system_info(info: *mut retro_system_info) {
	const NAME: &'static str = env!("CARGO_PKG_NAME");
	static mut CNAME: [c_char; NAME.len() + 1] = [0; NAME.len() + 1];
	const VER: &'static str = env!("CARGO_PKG_VERSION");
	static mut CVER: [c_char; VER.len() + 1] = [0; VER.len() + 1];
	if CNAME[0] == 0 {
		copy(NAME.as_ptr() as *const _, &mut CNAME as *mut _ as *mut _, NAME.len());
		copy(VER.as_ptr() as *const _, &mut CVER as *mut _ as *mut _, VER.len());
	}
	*info = retro_system_info {
		library_name: &CNAME as *const _,
		library_version: &CVER as *const _,
		valid_extensions: null(),
		need_fullpath: false,
		block_extract: true,
	};
}

#[no_mangle]
pub unsafe extern "C" fn retro_get_system_av_info(info: *mut retro_system_av_info) {
	*info = retro_system_av_info {
		timing: retro_system_timing { fps: 60.0, sample_rate: 0.0 },
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
	ENVIRON_CB = cb.unwrap();
	ENVIRON_CB(RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME, &true as *const _ as *mut _);
	let mut logging = retro_log_callback { log: None };
	LOG_CB = if ENVIRON_CB(RETRO_ENVIRONMENT_GET_LOG_INTERFACE, &mut logging as *mut _ as *mut _) {
		logging.log
	} else {
		None
	};
}

#[no_mangle]
pub unsafe extern "C" fn retro_set_video_refresh(cb: retro_video_refresh_t) {
	VIDEO_CB = cb.unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn retro_set_audio_sample(cb: retro_audio_sample_t) {
	AUDIO_CB = cb.unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn retro_set_audio_sample_batch(cb: retro_audio_sample_batch_t) {
	AUDIO_BATCH_CB = cb.unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn retro_set_input_poll(cb: retro_input_poll_t) {
	INPUT_POLL_CB = cb.unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn retro_set_input_state(cb: retro_input_state_t) {
	INPUT_STATE_CB = cb.unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn retro_set_controller_port_device(_port: c_uint, _device: c_uint) {}

#[no_mangle]
pub unsafe extern "C" fn retro_reset() {}

#[no_mangle]
pub unsafe extern "C" fn retro_run() {
	INPUT_POLL_CB();
	glBindFramebuffer(GL_FRAMEBUFFER, (HW_RENDER.get_current_framebuffer)() as _);
	static mut FRAME_COUNT: u8 = 0;
	let f: f32 = if FRAME_COUNT <= 127 { 0.33 } else { 0.67 };
	FRAME_COUNT += 2;
	glClearColor(f, f, f, f);
	glViewport(0, 0, VIDEO_WIDTH as _, VIDEO_HEIGHT as _);
	glClear(GL_COLOR_BUFFER_BIT);
	const IRRELEVANT: size_t = size_t::MAX;
	VIDEO_CB(RETRO_HW_FRAME_BUFFER_VALID, VIDEO_WIDTH as _, VIDEO_HEIGHT as _, IRRELEVANT);
}

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
	/*
		if info != null() {
			log_cb!(
				RETRO_LOG_ERROR,
				"Content file is given, but this core doesn't support any !!!\n\0",
			);
			return false;
		}
	*/
	if !ENVIRON_CB(
		RETRO_ENVIRONMENT_SET_PIXEL_FORMAT,
		&RETRO_PIXEL_FORMAT_XRGB8888 as *const _ as *mut _,
	) {
		log_cb!(RETRO_LOG_ERROR, "XRGB8888 is not supported.\n\0");
		return false;
	}
	if !ENVIRON_CB(RETRO_ENVIRONMENT_SET_HW_RENDER, &mut HW_RENDER as *mut _ as *mut _) {
		log_cb!(RETRO_LOG_ERROR, "HW Context could not be initialized.\n\0");
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
	RETRO_REGION_NTSC
}

#[no_mangle]
pub unsafe extern "C" fn retro_get_memory_data(_id: c_uint) -> *mut c_void {
	null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn retro_get_memory_size(_id: c_uint) -> size_t {
	0
}
