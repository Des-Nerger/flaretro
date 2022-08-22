/*


	The glad source code:

		The MIT License (MIT)

		Copyright
			(c) 2013-2022 David Herberth
			(c)      2022 Des-Nerger <mixerator@rambler.ru>

		Permission is hereby granted, free of charge, to any person obtaining a copy of
		this software and associated documentation files (the "Software"), to deal in
		the Software without restriction, including without limitation the rights to
		use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
		the Software, and to permit persons to whom the Software is furnished to do so,
		subject to the following conditions:

		The above copyright notice and this permission notice shall be included in all
		copies or substantial portions of the Software.

		THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
		IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
		FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
		COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
		IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
		CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.


	The Khronos Specifications:

		Copyright (c) 2013-2020 The Khronos Group Inc.

		Licensed under the Apache License, Version 2.0 (the "License");
		you may not use this file except in compliance with the License.
		You may obtain a copy of the License at

			http://www.apache.org/licenses/LICENSE-2.0

		Unless required by applicable law or agreed to in writing, software
		distributed under the License is distributed on an "AS IS" BASIS,
		WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
		See the License for the specific language governing permissions and
		limitations under the License.


	The EGL Specification and various headers:

		Copyright (c) 2007-2016 The Khronos Group Inc.

		Permission is hereby granted, free of charge, to any person obtaining a
		copy of this software and/or associated documentation files (the
		"Materials"), to deal in the Materials without restriction, including
		without limitation the rights to use, copy, modify, merge, publish,
		distribute, sublicense, and/or sell copies of the Materials, and to
		permit persons to whom the Materials are furnished to do so, subject to
		the following conditions:

		The above copyright notice and this permission notice shall be included
		in all copies or substantial portions of the Materials.

		THE MATERIALS ARE PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
		EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
		MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
		IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
		CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
		TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
		MATERIALS OR THE USE OR OTHER DEALINGS IN THE MATERIALS.


*/

pub use self::enumerations::*;
pub use self::functions::*;
pub use self::types::*;

use std::os::raw::c_void;

#[derive(Copy, Clone)]
struct FnPtr {
	ptr: *const c_void,
	is_loaded: bool,
}

#[allow(dead_code)]
impl FnPtr {
	fn new(ptr: *const c_void) -> FnPtr {
		if !ptr.is_null() {
			FnPtr { ptr, is_loaded: true }
		} else {
			FnPtr { ptr: FnPtr::not_initialized as *const c_void, is_loaded: false }
		}
	}

	fn set_ptr(&mut self, ptr: *const c_void) {
		*self = Self::new(ptr);
	}

	fn aliased(&mut self, other: &FnPtr) {
		if !self.is_loaded && other.is_loaded {
			*self = *other;
		}
	}

	#[inline(never)]
	fn not_initialized() -> ! {
		panic!("glad: function not initialized")
	}
}

unsafe impl Sync for FnPtr {}
unsafe impl Send for FnPtr {}

pub mod types {
	#![allow(dead_code, non_snake_case, non_camel_case_types)]

	use std::os::raw;

	pub type GLvoid = raw::c_void;

	pub type GLbyte = raw::c_char;
	pub type GLubyte = raw::c_uchar;
	pub type GLchar = raw::c_char;
	pub type GLboolean = raw::c_uchar;

	pub type GLshort = raw::c_short;
	pub type GLushort = raw::c_ushort;

	pub type GLint = raw::c_int;
	pub type GLuint = raw::c_uint;
	pub type GLint64 = i64;
	pub type GLuint64 = u64;

	pub type GLintptr = isize;
	pub type GLsizeiptr = isize;
	pub type GLintptrARB = isize;
	pub type GLsizeiptrARB = isize;
	pub type GLint64EXT = i64;
	pub type GLuint64EXT = u64;

	pub type GLsizei = GLint;
	pub type GLclampx = raw::c_int;
	pub type GLfixed = GLint;
	pub type GLhalf = raw::c_ushort;
	pub type GLhalfNV = raw::c_ushort;
	pub type GLhalfARB = raw::c_ushort;

	pub type GLenum = raw::c_uint;
	pub type GLbitfield = raw::c_uint;

	pub type GLfloat = raw::c_float;
	pub type GLdouble = raw::c_double;
	pub type GLclampf = raw::c_float;
	pub type GLclampd = raw::c_double;

	pub type GLcharARB = raw::c_char;

	#[cfg(target_os = "macos")]
	pub type GLhandleARB = *const raw::c_void;
	#[cfg(not(target_os = "macos"))]
	pub type GLhandleARB = raw::c_uint;

	pub enum __GLsync {}

	pub type GLsync = *const __GLsync;

	pub enum _cl_context {}

	pub enum _cl_event {}

	pub type GLvdpauSurfaceNV = GLintptr;
	pub type GLeglClientBufferEXT = *const raw::c_void;
	pub type GLeglImageOES = *const raw::c_void;

	pub type GLDEBUGPROC = extern "system" fn(
		source: GLenum,
		type_: GLenum,
		id: GLuint,
		severity: GLenum,
		length: GLsizei,
		message: *const GLchar,
		userParam: *mut raw::c_void,
	);
	pub type GLDEBUGPROCARB = extern "system" fn(
		source: GLenum,
		type_: GLenum,
		id: GLuint,
		severity: GLenum,
		length: GLsizei,
		message: *const GLchar,
		userParam: *mut raw::c_void,
	);
	pub type GLDEBUGPROCKHR = extern "system" fn(
		source: GLenum,
		type_: GLenum,
		id: GLuint,
		severity: GLenum,
		length: GLsizei,
		message: *const GLchar,
		userParam: *mut GLvoid,
	);
	pub type GLDEBUGPROCAMD = extern "system" fn(
		id: GLuint,
		category: GLenum,
		severity: GLenum,
		length: GLsizei,
		message: *const GLchar,
		userParam: *mut GLvoid,
	);
	pub type GLVULKANPROCNV = extern "system" fn();
}

pub mod enumerations {
	#![allow(dead_code, non_upper_case_globals, unused_imports)]

	use super::types::*;
	use std::os::raw::*;

	pub const GL_ACTIVE_ATTRIBUTES: c_uint = 0x8B89;
	pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: c_uint = 0x8B8A;
	pub const GL_ACTIVE_TEXTURE: c_uint = 0x84E0;
	pub const GL_ACTIVE_UNIFORMS: c_uint = 0x8B86;
	pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: c_uint = 0x8B87;
	pub const GL_ALIASED_LINE_WIDTH_RANGE: c_uint = 0x846E;
	pub const GL_ALIASED_POINT_SIZE_RANGE: c_uint = 0x846D;
	pub const GL_ALPHA: c_uint = 0x1906;
	pub const GL_ALPHA_BITS: c_uint = 0x0D55;
	pub const GL_ALWAYS: c_uint = 0x0207;
	pub const GL_ARRAY_BUFFER: c_uint = 0x8892;
	pub const GL_ARRAY_BUFFER_BINDING: c_uint = 0x8894;
	pub const GL_ATTACHED_SHADERS: c_uint = 0x8B85;
	pub const GL_BACK: c_uint = 0x0405;
	pub const GL_BLEND: c_uint = 0x0BE2;
	pub const GL_BLEND_COLOR: c_uint = 0x8005;
	pub const GL_BLEND_DST_ALPHA: c_uint = 0x80CA;
	pub const GL_BLEND_DST_RGB: c_uint = 0x80C8;
	pub const GL_BLEND_EQUATION: c_uint = 0x8009;
	pub const GL_BLEND_EQUATION_ALPHA: c_uint = 0x883D;
	pub const GL_BLEND_EQUATION_RGB: c_uint = 0x8009;
	pub const GL_BLEND_SRC_ALPHA: c_uint = 0x80CB;
	pub const GL_BLEND_SRC_RGB: c_uint = 0x80C9;
	pub const GL_BLUE_BITS: c_uint = 0x0D54;
	pub const GL_BOOL: c_uint = 0x8B56;
	pub const GL_BOOL_VEC2: c_uint = 0x8B57;
	pub const GL_BOOL_VEC3: c_uint = 0x8B58;
	pub const GL_BOOL_VEC4: c_uint = 0x8B59;
	pub const GL_BUFFER_SIZE: c_uint = 0x8764;
	pub const GL_BUFFER_USAGE: c_uint = 0x8765;
	pub const GL_BYTE: c_uint = 0x1400;
	pub const GL_CCW: c_uint = 0x0901;
	pub const GL_CLAMP_TO_EDGE: c_uint = 0x812F;
	pub const GL_COLOR_ATTACHMENT0: c_uint = 0x8CE0;
	pub const GL_COLOR_BUFFER_BIT: c_uint = 0x00004000;
	pub const GL_COLOR_CLEAR_VALUE: c_uint = 0x0C22;
	pub const GL_COLOR_WRITEMASK: c_uint = 0x0C23;
	pub const GL_COMPILE_STATUS: c_uint = 0x8B81;
	pub const GL_COMPRESSED_TEXTURE_FORMATS: c_uint = 0x86A3;
	pub const GL_CONSTANT_ALPHA: c_uint = 0x8003;
	pub const GL_CONSTANT_COLOR: c_uint = 0x8001;
	pub const GL_CULL_FACE: c_uint = 0x0B44;
	pub const GL_CULL_FACE_MODE: c_uint = 0x0B45;
	pub const GL_CURRENT_PROGRAM: c_uint = 0x8B8D;
	pub const GL_CURRENT_VERTEX_ATTRIB: c_uint = 0x8626;
	pub const GL_CW: c_uint = 0x0900;
	pub const GL_DECR: c_uint = 0x1E03;
	pub const GL_DECR_WRAP: c_uint = 0x8508;
	pub const GL_DELETE_STATUS: c_uint = 0x8B80;
	pub const GL_DEPTH_ATTACHMENT: c_uint = 0x8D00;
	pub const GL_DEPTH_BITS: c_uint = 0x0D56;
	pub const GL_DEPTH_BUFFER_BIT: c_uint = 0x00000100;
	pub const GL_DEPTH_CLEAR_VALUE: c_uint = 0x0B73;
	pub const GL_DEPTH_COMPONENT: c_uint = 0x1902;
	pub const GL_DEPTH_COMPONENT16: c_uint = 0x81A5;
	pub const GL_DEPTH_FUNC: c_uint = 0x0B74;
	pub const GL_DEPTH_RANGE: c_uint = 0x0B70;
	pub const GL_DEPTH_TEST: c_uint = 0x0B71;
	pub const GL_DEPTH_WRITEMASK: c_uint = 0x0B72;
	pub const GL_DITHER: c_uint = 0x0BD0;
	pub const GL_DONT_CARE: c_uint = 0x1100;
	pub const GL_DST_ALPHA: c_uint = 0x0304;
	pub const GL_DST_COLOR: c_uint = 0x0306;
	pub const GL_DYNAMIC_DRAW: c_uint = 0x88E8;
	pub const GL_ELEMENT_ARRAY_BUFFER: c_uint = 0x8893;
	pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: c_uint = 0x8895;
	pub const GL_EQUAL: c_uint = 0x0202;
	pub const GL_EXTENSIONS: c_uint = 0x1F03;
	pub const GL_FALSE: c_uchar = 0;
	pub const GL_FASTEST: c_uint = 0x1101;
	pub const GL_FLOAT: c_uint = 0x1406;
	pub const GL_FLOAT_MAT2: c_uint = 0x8B5A;
	pub const GL_FLOAT_MAT3: c_uint = 0x8B5B;
	pub const GL_FLOAT_MAT4: c_uint = 0x8B5C;
	pub const GL_FLOAT_VEC2: c_uint = 0x8B50;
	pub const GL_FLOAT_VEC3: c_uint = 0x8B51;
	pub const GL_FLOAT_VEC4: c_uint = 0x8B52;
	pub const GL_FRAGMENT_SHADER: c_uint = 0x8B30;
	pub const GL_FRAMEBUFFER: c_uint = 0x8D40;
	pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: c_uint = 0x8CD1;
	pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: c_uint = 0x8CD0;
	pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: c_uint = 0x8CD3;
	pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: c_uint = 0x8CD2;
	pub const GL_FRAMEBUFFER_BINDING: c_uint = 0x8CA6;
	pub const GL_FRAMEBUFFER_COMPLETE: c_uint = 0x8CD5;
	pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: c_uint = 0x8CD6;
	pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: c_uint = 0x8CD7;
	pub const GL_FRAMEBUFFER_UNSUPPORTED: c_uint = 0x8CDD;
	pub const GL_FRONT: c_uint = 0x0404;
	pub const GL_FRONT_AND_BACK: c_uint = 0x0408;
	pub const GL_FRONT_FACE: c_uint = 0x0B46;
	pub const GL_FUNC_ADD: c_uint = 0x8006;
	pub const GL_FUNC_REVERSE_SUBTRACT: c_uint = 0x800B;
	pub const GL_FUNC_SUBTRACT: c_uint = 0x800A;
	pub const GL_GENERATE_MIPMAP_HINT: c_uint = 0x8192;
	pub const GL_GEQUAL: c_uint = 0x0206;
	pub const GL_GREATER: c_uint = 0x0204;
	pub const GL_GREEN_BITS: c_uint = 0x0D53;
	pub const GL_INCR: c_uint = 0x1E02;
	pub const GL_INCR_WRAP: c_uint = 0x8507;
	pub const GL_INFO_LOG_LENGTH: c_uint = 0x8B84;
	pub const GL_INT: c_uint = 0x1404;
	pub const GL_INT_VEC2: c_uint = 0x8B53;
	pub const GL_INT_VEC3: c_uint = 0x8B54;
	pub const GL_INT_VEC4: c_uint = 0x8B55;
	pub const GL_INVALID_ENUM: c_uint = 0x0500;
	pub const GL_INVALID_FRAMEBUFFER_OPERATION: c_uint = 0x0506;
	pub const GL_INVALID_OPERATION: c_uint = 0x0502;
	pub const GL_INVALID_VALUE: c_uint = 0x0501;
	pub const GL_INVERT: c_uint = 0x150A;
	pub const GL_KEEP: c_uint = 0x1E00;
	pub const GL_LEQUAL: c_uint = 0x0203;
	pub const GL_LESS: c_uint = 0x0201;
	pub const GL_LINEAR: c_uint = 0x2601;
	pub const GL_LINEAR_MIPMAP_LINEAR: c_uint = 0x2703;
	pub const GL_LINEAR_MIPMAP_NEAREST: c_uint = 0x2701;
	pub const GL_LINES: c_uint = 0x0001;
	pub const GL_LINE_LOOP: c_uint = 0x0002;
	pub const GL_LINE_STRIP: c_uint = 0x0003;
	pub const GL_LINE_WIDTH: c_uint = 0x0B21;
	pub const GL_LINK_STATUS: c_uint = 0x8B82;
	pub const GL_LUMINANCE: c_uint = 0x1909;
	pub const GL_LUMINANCE_ALPHA: c_uint = 0x190A;
	pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: c_uint = 0x8B4D;
	pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: c_uint = 0x851C;
	pub const GL_MAX_RENDERBUFFER_SIZE: c_uint = 0x84E8;
	pub const GL_MAX_TEXTURE_IMAGE_UNITS: c_uint = 0x8872;
	pub const GL_MAX_TEXTURE_SIZE: c_uint = 0x0D33;
	pub const GL_MAX_VERTEX_ATTRIBS: c_uint = 0x8869;
	pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: c_uint = 0x8B4C;
	pub const GL_MAX_VIEWPORT_DIMS: c_uint = 0x0D3A;
	pub const GL_MIRRORED_REPEAT: c_uint = 0x8370;
	pub const GL_NEAREST: c_uint = 0x2600;
	pub const GL_NEAREST_MIPMAP_LINEAR: c_uint = 0x2702;
	pub const GL_NEAREST_MIPMAP_NEAREST: c_uint = 0x2700;
	pub const GL_NEVER: c_uint = 0x0200;
	pub const GL_NICEST: c_uint = 0x1102;
	pub const GL_NONE: c_uint = 0;
	pub const GL_NOTEQUAL: c_uint = 0x0205;
	pub const GL_NO_ERROR: c_uint = 0;
	pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: c_uint = 0x86A2;
	pub const GL_ONE: c_uint = 1;
	pub const GL_ONE_MINUS_CONSTANT_ALPHA: c_uint = 0x8004;
	pub const GL_ONE_MINUS_CONSTANT_COLOR: c_uint = 0x8002;
	pub const GL_ONE_MINUS_DST_ALPHA: c_uint = 0x0305;
	pub const GL_ONE_MINUS_DST_COLOR: c_uint = 0x0307;
	pub const GL_ONE_MINUS_SRC_ALPHA: c_uint = 0x0303;
	pub const GL_ONE_MINUS_SRC_COLOR: c_uint = 0x0301;
	pub const GL_OUT_OF_MEMORY: c_uint = 0x0505;
	pub const GL_PACK_ALIGNMENT: c_uint = 0x0D05;
	pub const GL_POINTS: c_uint = 0x0000;
	pub const GL_POLYGON_OFFSET_FACTOR: c_uint = 0x8038;
	pub const GL_POLYGON_OFFSET_FILL: c_uint = 0x8037;
	pub const GL_POLYGON_OFFSET_UNITS: c_uint = 0x2A00;
	pub const GL_RED_BITS: c_uint = 0x0D52;
	pub const GL_RENDERBUFFER: c_uint = 0x8D41;
	pub const GL_RENDERBUFFER_ALPHA_SIZE: c_uint = 0x8D53;
	pub const GL_RENDERBUFFER_BINDING: c_uint = 0x8CA7;
	pub const GL_RENDERBUFFER_BLUE_SIZE: c_uint = 0x8D52;
	pub const GL_RENDERBUFFER_DEPTH_SIZE: c_uint = 0x8D54;
	pub const GL_RENDERBUFFER_GREEN_SIZE: c_uint = 0x8D51;
	pub const GL_RENDERBUFFER_HEIGHT: c_uint = 0x8D43;
	pub const GL_RENDERBUFFER_INTERNAL_FORMAT: c_uint = 0x8D44;
	pub const GL_RENDERBUFFER_RED_SIZE: c_uint = 0x8D50;
	pub const GL_RENDERBUFFER_STENCIL_SIZE: c_uint = 0x8D55;
	pub const GL_RENDERBUFFER_WIDTH: c_uint = 0x8D42;
	pub const GL_RENDERER: c_uint = 0x1F01;
	pub const GL_REPEAT: c_uint = 0x2901;
	pub const GL_REPLACE: c_uint = 0x1E01;
	pub const GL_RGB: c_uint = 0x1907;
	pub const GL_RGB5_A1: c_uint = 0x8057;
	pub const GL_RGBA: c_uint = 0x1908;
	pub const GL_RGBA4: c_uint = 0x8056;
	pub const GL_SAMPLER_2D: c_uint = 0x8B5E;
	pub const GL_SAMPLER_CUBE: c_uint = 0x8B60;
	pub const GL_SAMPLES: c_uint = 0x80A9;
	pub const GL_SAMPLE_ALPHA_TO_COVERAGE: c_uint = 0x809E;
	pub const GL_SAMPLE_BUFFERS: c_uint = 0x80A8;
	pub const GL_SAMPLE_COVERAGE: c_uint = 0x80A0;
	pub const GL_SAMPLE_COVERAGE_INVERT: c_uint = 0x80AB;
	pub const GL_SAMPLE_COVERAGE_VALUE: c_uint = 0x80AA;
	pub const GL_SCISSOR_BOX: c_uint = 0x0C10;
	pub const GL_SCISSOR_TEST: c_uint = 0x0C11;
	pub const GL_SHADER_SOURCE_LENGTH: c_uint = 0x8B88;
	pub const GL_SHADER_TYPE: c_uint = 0x8B4F;
	pub const GL_SHADING_LANGUAGE_VERSION: c_uint = 0x8B8C;
	pub const GL_SHORT: c_uint = 0x1402;
	pub const GL_SRC_ALPHA: c_uint = 0x0302;
	pub const GL_SRC_ALPHA_SATURATE: c_uint = 0x0308;
	pub const GL_SRC_COLOR: c_uint = 0x0300;
	pub const GL_STATIC_DRAW: c_uint = 0x88E4;
	pub const GL_STENCIL_ATTACHMENT: c_uint = 0x8D20;
	pub const GL_STENCIL_BACK_FAIL: c_uint = 0x8801;
	pub const GL_STENCIL_BACK_FUNC: c_uint = 0x8800;
	pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: c_uint = 0x8802;
	pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: c_uint = 0x8803;
	pub const GL_STENCIL_BACK_REF: c_uint = 0x8CA3;
	pub const GL_STENCIL_BACK_VALUE_MASK: c_uint = 0x8CA4;
	pub const GL_STENCIL_BACK_WRITEMASK: c_uint = 0x8CA5;
	pub const GL_STENCIL_BITS: c_uint = 0x0D57;
	pub const GL_STENCIL_BUFFER_BIT: c_uint = 0x00000400;
	pub const GL_STENCIL_CLEAR_VALUE: c_uint = 0x0B91;
	pub const GL_STENCIL_FAIL: c_uint = 0x0B94;
	pub const GL_STENCIL_FUNC: c_uint = 0x0B92;
	pub const GL_STENCIL_INDEX8: c_uint = 0x8D48;
	pub const GL_STENCIL_PASS_DEPTH_FAIL: c_uint = 0x0B95;
	pub const GL_STENCIL_PASS_DEPTH_PASS: c_uint = 0x0B96;
	pub const GL_STENCIL_REF: c_uint = 0x0B97;
	pub const GL_STENCIL_TEST: c_uint = 0x0B90;
	pub const GL_STENCIL_VALUE_MASK: c_uint = 0x0B93;
	pub const GL_STENCIL_WRITEMASK: c_uint = 0x0B98;
	pub const GL_STREAM_DRAW: c_uint = 0x88E0;
	pub const GL_SUBPIXEL_BITS: c_uint = 0x0D50;
	pub const GL_TEXTURE: c_uint = 0x1702;
	pub const GL_TEXTURE0: c_uint = 0x84C0;
	pub const GL_TEXTURE1: c_uint = 0x84C1;
	pub const GL_TEXTURE10: c_uint = 0x84CA;
	pub const GL_TEXTURE11: c_uint = 0x84CB;
	pub const GL_TEXTURE12: c_uint = 0x84CC;
	pub const GL_TEXTURE13: c_uint = 0x84CD;
	pub const GL_TEXTURE14: c_uint = 0x84CE;
	pub const GL_TEXTURE15: c_uint = 0x84CF;
	pub const GL_TEXTURE16: c_uint = 0x84D0;
	pub const GL_TEXTURE17: c_uint = 0x84D1;
	pub const GL_TEXTURE18: c_uint = 0x84D2;
	pub const GL_TEXTURE19: c_uint = 0x84D3;
	pub const GL_TEXTURE2: c_uint = 0x84C2;
	pub const GL_TEXTURE20: c_uint = 0x84D4;
	pub const GL_TEXTURE21: c_uint = 0x84D5;
	pub const GL_TEXTURE22: c_uint = 0x84D6;
	pub const GL_TEXTURE23: c_uint = 0x84D7;
	pub const GL_TEXTURE24: c_uint = 0x84D8;
	pub const GL_TEXTURE25: c_uint = 0x84D9;
	pub const GL_TEXTURE26: c_uint = 0x84DA;
	pub const GL_TEXTURE27: c_uint = 0x84DB;
	pub const GL_TEXTURE28: c_uint = 0x84DC;
	pub const GL_TEXTURE29: c_uint = 0x84DD;
	pub const GL_TEXTURE3: c_uint = 0x84C3;
	pub const GL_TEXTURE30: c_uint = 0x84DE;
	pub const GL_TEXTURE31: c_uint = 0x84DF;
	pub const GL_TEXTURE4: c_uint = 0x84C4;
	pub const GL_TEXTURE5: c_uint = 0x84C5;
	pub const GL_TEXTURE6: c_uint = 0x84C6;
	pub const GL_TEXTURE7: c_uint = 0x84C7;
	pub const GL_TEXTURE8: c_uint = 0x84C8;
	pub const GL_TEXTURE9: c_uint = 0x84C9;
	pub const GL_TEXTURE_2D: c_uint = 0x0DE1;
	pub const GL_TEXTURE_BINDING_2D: c_uint = 0x8069;
	pub const GL_TEXTURE_BINDING_CUBE_MAP: c_uint = 0x8514;
	pub const GL_TEXTURE_CUBE_MAP: c_uint = 0x8513;
	pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: c_uint = 0x8516;
	pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: c_uint = 0x8518;
	pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: c_uint = 0x851A;
	pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: c_uint = 0x8515;
	pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: c_uint = 0x8517;
	pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: c_uint = 0x8519;
	pub const GL_TEXTURE_MAG_FILTER: c_uint = 0x2800;
	pub const GL_TEXTURE_MIN_FILTER: c_uint = 0x2801;
	pub const GL_TEXTURE_WRAP_S: c_uint = 0x2802;
	pub const GL_TEXTURE_WRAP_T: c_uint = 0x2803;
	pub const GL_TRIANGLES: c_uint = 0x0004;
	pub const GL_TRIANGLE_FAN: c_uint = 0x0006;
	pub const GL_TRIANGLE_STRIP: c_uint = 0x0005;
	pub const GL_TRUE: c_uchar = 1;
	pub const GL_UNPACK_ALIGNMENT: c_uint = 0x0CF5;
	pub const GL_UNSIGNED_BYTE: c_uint = 0x1401;
	pub const GL_UNSIGNED_INT: c_uint = 0x1405;
	pub const GL_UNSIGNED_SHORT: c_uint = 0x1403;
	pub const GL_UNSIGNED_SHORT_4_4_4_4: c_uint = 0x8033;
	pub const GL_UNSIGNED_SHORT_5_5_5_1: c_uint = 0x8034;
	pub const GL_UNSIGNED_SHORT_5_6_5: c_uint = 0x8363;
	pub const GL_VALIDATE_STATUS: c_uint = 0x8B83;
	pub const GL_VENDOR: c_uint = 0x1F00;
	pub const GL_VERSION: c_uint = 0x1F02;
	pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: c_uint = 0x889F;
	pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: c_uint = 0x8622;
	pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: c_uint = 0x886A;
	pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: c_uint = 0x8645;
	pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: c_uint = 0x8623;
	pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: c_uint = 0x8624;
	pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: c_uint = 0x8625;
	pub const GL_VERTEX_SHADER: c_uint = 0x8B31;
	pub const GL_VIEWPORT: c_uint = 0x0BA2;
	pub const GL_ZERO: c_uint = 0;
}

pub mod functions {
	#![allow(non_snake_case, unused_variables, dead_code, unused_imports)]

	use super::types::*;
	use super::*;
	use std::mem::transmute;
	use std::os::raw::*;

	macro_rules! func {
		($fun:ident, $ret:ty, $($name:ident: $typ:ty),*) => {
			#[inline] pub unsafe fn $fun($($name: $typ),*) -> $ret {
				transmute::<_, extern "system" fn($($typ),*) -> $ret>(storage::$fun.ptr)($($name),*)
			}
		}
	}

	func!(glActiveTexture, (), texture: GLenum);
	func!(glAttachShader, (), program: GLuint, shader: GLuint);
	func!(glBindAttribLocation, (), program: GLuint, index: GLuint, name: *const GLchar);
	func!(glBindBuffer, (), target: GLenum, buffer: GLuint);
	func!(glBindFramebuffer, (), target: GLenum, framebuffer: GLuint);
	func!(glBindRenderbuffer, (), target: GLenum, renderbuffer: GLuint);
	func!(glBindTexture, (), target: GLenum, texture: GLuint);
	func!(glBlendColor, (), red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
	func!(glBlendEquation, (), mode: GLenum);
	func!(glBlendEquationSeparate, (), modeRGB: GLenum, modeAlpha: GLenum);
	func!(glBlendFunc, (), sfactor: GLenum, dfactor: GLenum);
	func!(
		glBlendFuncSeparate,
		(),
		sfactorRGB: GLenum,
		dfactorRGB: GLenum,
		sfactorAlpha: GLenum,
		dfactorAlpha: GLenum
	);
	func!(glBufferData, (), target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum);
	func!(glBufferSubData, (), target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void);
	func!(glCheckFramebufferStatus, GLenum, target: GLenum);
	func!(glClear, (), mask: GLbitfield);
	func!(glClearColor, (), red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
	func!(glClearStencil, (), s: GLint);
	func!(glColorMask, (), red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);
	func!(glCompileShader, (), shader: GLuint);
	func!(
		glCompressedTexImage2D,
		(),
		target: GLenum,
		level: GLint,
		internalformat: GLenum,
		width: GLsizei,
		height: GLsizei,
		border: GLint,
		imageSize: GLsizei,
		data: *const c_void
	);
	func!(
		glCompressedTexSubImage2D,
		(),
		target: GLenum,
		level: GLint,
		xoffset: GLint,
		yoffset: GLint,
		width: GLsizei,
		height: GLsizei,
		format: GLenum,
		imageSize: GLsizei,
		data: *const c_void
	);
	func!(
		glCopyTexImage2D,
		(),
		target: GLenum,
		level: GLint,
		internalformat: GLenum,
		x: GLint,
		y: GLint,
		width: GLsizei,
		height: GLsizei,
		border: GLint
	);
	func!(
		glCopyTexSubImage2D,
		(),
		target: GLenum,
		level: GLint,
		xoffset: GLint,
		yoffset: GLint,
		x: GLint,
		y: GLint,
		width: GLsizei,
		height: GLsizei
	);
	func!(glCreateProgram, GLuint,);
	func!(glCreateShader, GLuint, type_: GLenum);
	func!(glCullFace, (), mode: GLenum);
	func!(glDeleteBuffers, (), n: GLsizei, buffers: *const GLuint);
	func!(glDeleteFramebuffers, (), n: GLsizei, framebuffers: *const GLuint);
	func!(glDeleteProgram, (), program: GLuint);
	func!(glDeleteRenderbuffers, (), n: GLsizei, renderbuffers: *const GLuint);
	func!(glDeleteShader, (), shader: GLuint);
	func!(glDeleteTextures, (), n: GLsizei, textures: *const GLuint);
	func!(glDepthFunc, (), func: GLenum);
	func!(glDepthMask, (), flag: GLboolean);
	func!(glDetachShader, (), program: GLuint, shader: GLuint);
	func!(glDisable, (), cap: GLenum);
	func!(glDisableVertexAttribArray, (), index: GLuint);
	func!(glDrawArrays, (), mode: GLenum, first: GLint, count: GLsizei);
	func!(glDrawElements, (), mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void);
	func!(glEnable, (), cap: GLenum);
	func!(glEnableVertexAttribArray, (), index: GLuint);
	func!(glFinish, (),);
	func!(glFlush, (),);
	func!(
		glFramebufferRenderbuffer,
		(),
		target: GLenum,
		attachment: GLenum,
		renderbuffertarget: GLenum,
		renderbuffer: GLuint
	);
	func!(
		glFramebufferTexture2D,
		(),
		target: GLenum,
		attachment: GLenum,
		textarget: GLenum,
		texture: GLuint,
		level: GLint
	);
	func!(glFrontFace, (), mode: GLenum);
	func!(glGenBuffers, (), n: GLsizei, buffers: *mut GLuint);
	func!(glGenFramebuffers, (), n: GLsizei, framebuffers: *mut GLuint);
	func!(glGenRenderbuffers, (), n: GLsizei, renderbuffers: *mut GLuint);
	func!(glGenTextures, (), n: GLsizei, textures: *mut GLuint);
	func!(glGenerateMipmap, (), target: GLenum);
	func!(
		glGetActiveAttrib,
		(),
		program: GLuint,
		index: GLuint,
		bufSize: GLsizei,
		length: *mut GLsizei,
		size: *mut GLint,
		type_: *mut GLenum,
		name: *mut GLchar
	);
	func!(
		glGetActiveUniform,
		(),
		program: GLuint,
		index: GLuint,
		bufSize: GLsizei,
		length: *mut GLsizei,
		size: *mut GLint,
		type_: *mut GLenum,
		name: *mut GLchar
	);
	func!(
		glGetAttachedShaders,
		(),
		program: GLuint,
		maxCount: GLsizei,
		count: *mut GLsizei,
		shaders: *mut GLuint
	);
	func!(glGetAttribLocation, GLint, program: GLuint, name: *const GLchar);
	func!(glGetBooleanv, (), pname: GLenum, data: *mut GLboolean);
	func!(glGetBufferParameteriv, (), target: GLenum, pname: GLenum, params: *mut GLint);
	func!(glGetError, GLenum,);
	func!(glGetFloatv, (), pname: GLenum, data: *mut GLfloat);
	func!(
		glGetFramebufferAttachmentParameteriv,
		(),
		target: GLenum,
		attachment: GLenum,
		pname: GLenum,
		params: *mut GLint
	);
	func!(glGetIntegerv, (), pname: GLenum, data: *mut GLint);
	func!(
		glGetProgramInfoLog,
		(),
		program: GLuint,
		bufSize: GLsizei,
		length: *mut GLsizei,
		infoLog: *mut GLchar
	);
	func!(glGetProgramiv, (), program: GLuint, pname: GLenum, params: *mut GLint);
	func!(glGetRenderbufferParameteriv, (), target: GLenum, pname: GLenum, params: *mut GLint);
	func!(
		glGetShaderInfoLog,
		(),
		shader: GLuint,
		bufSize: GLsizei,
		length: *mut GLsizei,
		infoLog: *mut GLchar
	);
	func!(
		glGetShaderSource,
		(),
		shader: GLuint,
		bufSize: GLsizei,
		length: *mut GLsizei,
		source: *mut GLchar
	);
	func!(glGetShaderiv, (), shader: GLuint, pname: GLenum, params: *mut GLint);
	func!(glGetString, *const GLubyte, name: GLenum);
	func!(glGetTexParameterfv, (), target: GLenum, pname: GLenum, params: *mut GLfloat);
	func!(glGetTexParameteriv, (), target: GLenum, pname: GLenum, params: *mut GLint);
	func!(glGetUniformLocation, GLint, program: GLuint, name: *const GLchar);
	func!(glGetUniformfv, (), program: GLuint, location: GLint, params: *mut GLfloat);
	func!(glGetUniformiv, (), program: GLuint, location: GLint, params: *mut GLint);
	func!(glGetVertexAttribPointerv, (), index: GLuint, pname: GLenum, pointer: *mut *mut c_void);
	func!(glGetVertexAttribfv, (), index: GLuint, pname: GLenum, params: *mut GLfloat);
	func!(glGetVertexAttribiv, (), index: GLuint, pname: GLenum, params: *mut GLint);
	func!(glHint, (), target: GLenum, mode: GLenum);
	func!(glIsBuffer, GLboolean, buffer: GLuint);
	func!(glIsEnabled, GLboolean, cap: GLenum);
	func!(glIsFramebuffer, GLboolean, framebuffer: GLuint);
	func!(glIsProgram, GLboolean, program: GLuint);
	func!(glIsRenderbuffer, GLboolean, renderbuffer: GLuint);
	func!(glIsShader, GLboolean, shader: GLuint);
	func!(glIsTexture, GLboolean, texture: GLuint);
	func!(glLineWidth, (), width: GLfloat);
	func!(glLinkProgram, (), program: GLuint);
	func!(glPixelStorei, (), pname: GLenum, param: GLint);
	func!(glPolygonOffset, (), factor: GLfloat, units: GLfloat);
	func!(
		glReadPixels,
		(),
		x: GLint,
		y: GLint,
		width: GLsizei,
		height: GLsizei,
		format: GLenum,
		type_: GLenum,
		pixels: *mut c_void
	);
	func!(
		glRenderbufferStorage,
		(),
		target: GLenum,
		internalformat: GLenum,
		width: GLsizei,
		height: GLsizei
	);
	func!(glSampleCoverage, (), value: GLfloat, invert: GLboolean);
	func!(glScissor, (), x: GLint, y: GLint, width: GLsizei, height: GLsizei);
	func!(
		glShaderSource,
		(),
		shader: GLuint,
		count: GLsizei,
		string: *const *const GLchar,
		length: *const GLint
	);
	func!(glStencilFunc, (), func: GLenum, ref_: GLint, mask: GLuint);
	func!(glStencilFuncSeparate, (), face: GLenum, func: GLenum, ref_: GLint, mask: GLuint);
	func!(glStencilMask, (), mask: GLuint);
	func!(glStencilMaskSeparate, (), face: GLenum, mask: GLuint);
	func!(glStencilOp, (), fail: GLenum, zfail: GLenum, zpass: GLenum);
	func!(glStencilOpSeparate, (), face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum);
	func!(
		glTexImage2D,
		(),
		target: GLenum,
		level: GLint,
		internalformat: GLint,
		width: GLsizei,
		height: GLsizei,
		border: GLint,
		format: GLenum,
		type_: GLenum,
		pixels: *const c_void
	);
	func!(glTexParameterf, (), target: GLenum, pname: GLenum, param: GLfloat);
	func!(glTexParameterfv, (), target: GLenum, pname: GLenum, params: *const GLfloat);
	func!(glTexParameteri, (), target: GLenum, pname: GLenum, param: GLint);
	func!(glTexParameteriv, (), target: GLenum, pname: GLenum, params: *const GLint);
	func!(
		glTexSubImage2D,
		(),
		target: GLenum,
		level: GLint,
		xoffset: GLint,
		yoffset: GLint,
		width: GLsizei,
		height: GLsizei,
		format: GLenum,
		type_: GLenum,
		pixels: *const c_void
	);
	func!(glUniform1f, (), location: GLint, v0: GLfloat);
	func!(glUniform1fv, (), location: GLint, count: GLsizei, value: *const GLfloat);
	func!(glUniform1i, (), location: GLint, v0: GLint);
	func!(glUniform1iv, (), location: GLint, count: GLsizei, value: *const GLint);
	func!(glUniform2f, (), location: GLint, v0: GLfloat, v1: GLfloat);
	func!(glUniform2fv, (), location: GLint, count: GLsizei, value: *const GLfloat);
	func!(glUniform2i, (), location: GLint, v0: GLint, v1: GLint);
	func!(glUniform2iv, (), location: GLint, count: GLsizei, value: *const GLint);
	func!(glUniform3f, (), location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
	func!(glUniform3fv, (), location: GLint, count: GLsizei, value: *const GLfloat);
	func!(glUniform3i, (), location: GLint, v0: GLint, v1: GLint, v2: GLint);
	func!(glUniform3iv, (), location: GLint, count: GLsizei, value: *const GLint);
	func!(glUniform4f, (), location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
	func!(glUniform4fv, (), location: GLint, count: GLsizei, value: *const GLfloat);
	func!(glUniform4i, (), location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
	func!(glUniform4iv, (), location: GLint, count: GLsizei, value: *const GLint);
	func!(
		glUniformMatrix2fv,
		(),
		location: GLint,
		count: GLsizei,
		transpose: GLboolean,
		value: *const GLfloat
	);
	func!(
		glUniformMatrix3fv,
		(),
		location: GLint,
		count: GLsizei,
		transpose: GLboolean,
		value: *const GLfloat
	);
	func!(
		glUniformMatrix4fv,
		(),
		location: GLint,
		count: GLsizei,
		transpose: GLboolean,
		value: *const GLfloat
	);
	func!(glUseProgram, (), program: GLuint);
	func!(glValidateProgram, (), program: GLuint);
	func!(glVertexAttrib1f, (), index: GLuint, x: GLfloat);
	func!(glVertexAttrib1fv, (), index: GLuint, v: *const GLfloat);
	func!(glVertexAttrib2f, (), index: GLuint, x: GLfloat, y: GLfloat);
	func!(glVertexAttrib2fv, (), index: GLuint, v: *const GLfloat);
	func!(glVertexAttrib3f, (), index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
	func!(glVertexAttrib3fv, (), index: GLuint, v: *const GLfloat);
	func!(glVertexAttrib4f, (), index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
	func!(glVertexAttrib4fv, (), index: GLuint, v: *const GLfloat);
	func!(
		glVertexAttribPointer,
		(),
		index: GLuint,
		size: GLint,
		type_: GLenum,
		normalized: GLboolean,
		stride: GLsizei,
		pointer: *const c_void
	);
	func!(glViewport, (), x: GLint, y: GLint, width: GLsizei, height: GLsizei);
}

mod storage {
	#![allow(non_snake_case, non_upper_case_globals)]

	use super::FnPtr;
	use std::os::raw::*;

	macro_rules! store {
		($name:ident) => {
			pub(super) static mut $name: FnPtr =
				FnPtr { ptr: FnPtr::not_initialized as *const c_void, is_loaded: false };
		};
	}

	store!(glActiveTexture);
	store!(glAttachShader);
	store!(glBindAttribLocation);
	store!(glBindBuffer);
	store!(glBindFramebuffer);
	store!(glBindRenderbuffer);
	store!(glBindTexture);
	store!(glBlendColor);
	store!(glBlendEquation);
	store!(glBlendEquationSeparate);
	store!(glBlendFunc);
	store!(glBlendFuncSeparate);
	store!(glBufferData);
	store!(glBufferSubData);
	store!(glCheckFramebufferStatus);
	store!(glClear);
	store!(glClearColor);
	store!(glClearStencil);
	store!(glColorMask);
	store!(glCompileShader);
	store!(glCompressedTexImage2D);
	store!(glCompressedTexSubImage2D);
	store!(glCopyTexImage2D);
	store!(glCopyTexSubImage2D);
	store!(glCreateProgram);
	store!(glCreateShader);
	store!(glCullFace);
	store!(glDeleteBuffers);
	store!(glDeleteFramebuffers);
	store!(glDeleteProgram);
	store!(glDeleteRenderbuffers);
	store!(glDeleteShader);
	store!(glDeleteTextures);
	store!(glDepthFunc);
	store!(glDepthMask);
	store!(glDetachShader);
	store!(glDisable);
	store!(glDisableVertexAttribArray);
	store!(glDrawArrays);
	store!(glDrawElements);
	store!(glEnable);
	store!(glEnableVertexAttribArray);
	store!(glFinish);
	store!(glFlush);
	store!(glFramebufferRenderbuffer);
	store!(glFramebufferTexture2D);
	store!(glFrontFace);
	store!(glGenBuffers);
	store!(glGenFramebuffers);
	store!(glGenRenderbuffers);
	store!(glGenTextures);
	store!(glGenerateMipmap);
	store!(glGetActiveAttrib);
	store!(glGetActiveUniform);
	store!(glGetAttachedShaders);
	store!(glGetAttribLocation);
	store!(glGetBooleanv);
	store!(glGetBufferParameteriv);
	store!(glGetError);
	store!(glGetFloatv);
	store!(glGetFramebufferAttachmentParameteriv);
	store!(glGetIntegerv);
	store!(glGetProgramInfoLog);
	store!(glGetProgramiv);
	store!(glGetRenderbufferParameteriv);
	store!(glGetShaderInfoLog);
	store!(glGetShaderSource);
	store!(glGetShaderiv);
	store!(glGetString);
	store!(glGetTexParameterfv);
	store!(glGetTexParameteriv);
	store!(glGetUniformLocation);
	store!(glGetUniformfv);
	store!(glGetUniformiv);
	store!(glGetVertexAttribPointerv);
	store!(glGetVertexAttribfv);
	store!(glGetVertexAttribiv);
	store!(glHint);
	store!(glIsBuffer);
	store!(glIsEnabled);
	store!(glIsFramebuffer);
	store!(glIsProgram);
	store!(glIsRenderbuffer);
	store!(glIsShader);
	store!(glIsTexture);
	store!(glLineWidth);
	store!(glLinkProgram);
	store!(glPixelStorei);
	store!(glPolygonOffset);
	store!(glReadPixels);
	store!(glRenderbufferStorage);
	store!(glSampleCoverage);
	store!(glScissor);
	store!(glShaderSource);
	store!(glStencilFunc);
	store!(glStencilFuncSeparate);
	store!(glStencilMask);
	store!(glStencilMaskSeparate);
	store!(glStencilOp);
	store!(glStencilOpSeparate);
	store!(glTexImage2D);
	store!(glTexParameterf);
	store!(glTexParameterfv);
	store!(glTexParameteri);
	store!(glTexParameteriv);
	store!(glTexSubImage2D);
	store!(glUniform1f);
	store!(glUniform1fv);
	store!(glUniform1i);
	store!(glUniform1iv);
	store!(glUniform2f);
	store!(glUniform2fv);
	store!(glUniform2i);
	store!(glUniform2iv);
	store!(glUniform3f);
	store!(glUniform3fv);
	store!(glUniform3i);
	store!(glUniform3iv);
	store!(glUniform4f);
	store!(glUniform4fv);
	store!(glUniform4i);
	store!(glUniform4iv);
	store!(glUniformMatrix2fv);
	store!(glUniformMatrix3fv);
	store!(glUniformMatrix4fv);
	store!(glUseProgram);
	store!(glValidateProgram);
	store!(glVertexAttrib1f);
	store!(glVertexAttrib1fv);
	store!(glVertexAttrib2f);
	store!(glVertexAttrib2fv);
	store!(glVertexAttrib3f);
	store!(glVertexAttrib3fv);
	store!(glVertexAttrib4f);
	store!(glVertexAttrib4fv);
	store!(glVertexAttribPointer);
	store!(glViewport);
}

pub fn gl_load<F>(mut loadfn: F)
where
	F: FnMut(&'static str) -> *const c_void,
{
	unsafe {
		storage::glActiveTexture.set_ptr(loadfn("glActiveTexture\0"));
		storage::glAttachShader.set_ptr(loadfn("glAttachShader\0"));
		storage::glBindAttribLocation.set_ptr(loadfn("glBindAttribLocation\0"));
		storage::glBindBuffer.set_ptr(loadfn("glBindBuffer\0"));
		storage::glBindFramebuffer.set_ptr(loadfn("glBindFramebuffer\0"));
		storage::glBindRenderbuffer.set_ptr(loadfn("glBindRenderbuffer\0"));
		storage::glBindTexture.set_ptr(loadfn("glBindTexture\0"));
		storage::glBlendColor.set_ptr(loadfn("glBlendColor\0"));
		storage::glBlendEquation.set_ptr(loadfn("glBlendEquation\0"));
		storage::glBlendEquationSeparate.set_ptr(loadfn("glBlendEquationSeparate\0"));
		storage::glBlendFunc.set_ptr(loadfn("glBlendFunc\0"));
		storage::glBlendFuncSeparate.set_ptr(loadfn("glBlendFuncSeparate\0"));
		storage::glBufferData.set_ptr(loadfn("glBufferData\0"));
		storage::glBufferSubData.set_ptr(loadfn("glBufferSubData\0"));
		storage::glCheckFramebufferStatus.set_ptr(loadfn("glCheckFramebufferStatus\0"));
		storage::glClear.set_ptr(loadfn("glClear\0"));
		storage::glClearColor.set_ptr(loadfn("glClearColor\0"));
		storage::glClearStencil.set_ptr(loadfn("glClearStencil\0"));
		storage::glColorMask.set_ptr(loadfn("glColorMask\0"));
		storage::glCompileShader.set_ptr(loadfn("glCompileShader\0"));
		storage::glCompressedTexImage2D.set_ptr(loadfn("glCompressedTexImage2D\0"));
		storage::glCompressedTexSubImage2D.set_ptr(loadfn("glCompressedTexSubImage2D\0"));
		storage::glCopyTexImage2D.set_ptr(loadfn("glCopyTexImage2D\0"));
		storage::glCopyTexSubImage2D.set_ptr(loadfn("glCopyTexSubImage2D\0"));
		storage::glCreateProgram.set_ptr(loadfn("glCreateProgram\0"));
		storage::glCreateShader.set_ptr(loadfn("glCreateShader\0"));
		storage::glCullFace.set_ptr(loadfn("glCullFace\0"));
		storage::glDeleteBuffers.set_ptr(loadfn("glDeleteBuffers\0"));
		storage::glDeleteFramebuffers.set_ptr(loadfn("glDeleteFramebuffers\0"));
		storage::glDeleteProgram.set_ptr(loadfn("glDeleteProgram\0"));
		storage::glDeleteRenderbuffers.set_ptr(loadfn("glDeleteRenderbuffers\0"));
		storage::glDeleteShader.set_ptr(loadfn("glDeleteShader\0"));
		storage::glDeleteTextures.set_ptr(loadfn("glDeleteTextures\0"));
		storage::glDepthFunc.set_ptr(loadfn("glDepthFunc\0"));
		storage::glDepthMask.set_ptr(loadfn("glDepthMask\0"));
		storage::glDetachShader.set_ptr(loadfn("glDetachShader\0"));
		storage::glDisable.set_ptr(loadfn("glDisable\0"));
		storage::glDisableVertexAttribArray.set_ptr(loadfn("glDisableVertexAttribArray\0"));
		storage::glDrawArrays.set_ptr(loadfn("glDrawArrays\0"));
		storage::glDrawElements.set_ptr(loadfn("glDrawElements\0"));
		storage::glEnable.set_ptr(loadfn("glEnable\0"));
		storage::glEnableVertexAttribArray.set_ptr(loadfn("glEnableVertexAttribArray\0"));
		storage::glFinish.set_ptr(loadfn("glFinish\0"));
		storage::glFlush.set_ptr(loadfn("glFlush\0"));
		storage::glFramebufferRenderbuffer.set_ptr(loadfn("glFramebufferRenderbuffer\0"));
		storage::glFramebufferTexture2D.set_ptr(loadfn("glFramebufferTexture2D\0"));
		storage::glFrontFace.set_ptr(loadfn("glFrontFace\0"));
		storage::glGenBuffers.set_ptr(loadfn("glGenBuffers\0"));
		storage::glGenFramebuffers.set_ptr(loadfn("glGenFramebuffers\0"));
		storage::glGenRenderbuffers.set_ptr(loadfn("glGenRenderbuffers\0"));
		storage::glGenTextures.set_ptr(loadfn("glGenTextures\0"));
		storage::glGenerateMipmap.set_ptr(loadfn("glGenerateMipmap\0"));
		storage::glGetActiveAttrib.set_ptr(loadfn("glGetActiveAttrib\0"));
		storage::glGetActiveUniform.set_ptr(loadfn("glGetActiveUniform\0"));
		storage::glGetAttachedShaders.set_ptr(loadfn("glGetAttachedShaders\0"));
		storage::glGetAttribLocation.set_ptr(loadfn("glGetAttribLocation\0"));
		storage::glGetBooleanv.set_ptr(loadfn("glGetBooleanv\0"));
		storage::glGetBufferParameteriv.set_ptr(loadfn("glGetBufferParameteriv\0"));
		storage::glGetError.set_ptr(loadfn("glGetError\0"));
		storage::glGetFloatv.set_ptr(loadfn("glGetFloatv\0"));
		storage::glGetFramebufferAttachmentParameteriv
			.set_ptr(loadfn("glGetFramebufferAttachmentParameteriv\0"));
		storage::glGetIntegerv.set_ptr(loadfn("glGetIntegerv\0"));
		storage::glGetProgramInfoLog.set_ptr(loadfn("glGetProgramInfoLog\0"));
		storage::glGetProgramiv.set_ptr(loadfn("glGetProgramiv\0"));
		storage::glGetRenderbufferParameteriv.set_ptr(loadfn("glGetRenderbufferParameteriv\0"));
		storage::glGetShaderInfoLog.set_ptr(loadfn("glGetShaderInfoLog\0"));
		storage::glGetShaderSource.set_ptr(loadfn("glGetShaderSource\0"));
		storage::glGetShaderiv.set_ptr(loadfn("glGetShaderiv\0"));
		storage::glGetString.set_ptr(loadfn("glGetString\0"));
		storage::glGetTexParameterfv.set_ptr(loadfn("glGetTexParameterfv\0"));
		storage::glGetTexParameteriv.set_ptr(loadfn("glGetTexParameteriv\0"));
		storage::glGetUniformLocation.set_ptr(loadfn("glGetUniformLocation\0"));
		storage::glGetUniformfv.set_ptr(loadfn("glGetUniformfv\0"));
		storage::glGetUniformiv.set_ptr(loadfn("glGetUniformiv\0"));
		storage::glGetVertexAttribPointerv.set_ptr(loadfn("glGetVertexAttribPointerv\0"));
		storage::glGetVertexAttribfv.set_ptr(loadfn("glGetVertexAttribfv\0"));
		storage::glGetVertexAttribiv.set_ptr(loadfn("glGetVertexAttribiv\0"));
		storage::glHint.set_ptr(loadfn("glHint\0"));
		storage::glIsBuffer.set_ptr(loadfn("glIsBuffer\0"));
		storage::glIsEnabled.set_ptr(loadfn("glIsEnabled\0"));
		storage::glIsFramebuffer.set_ptr(loadfn("glIsFramebuffer\0"));
		storage::glIsProgram.set_ptr(loadfn("glIsProgram\0"));
		storage::glIsRenderbuffer.set_ptr(loadfn("glIsRenderbuffer\0"));
		storage::glIsShader.set_ptr(loadfn("glIsShader\0"));
		storage::glIsTexture.set_ptr(loadfn("glIsTexture\0"));
		storage::glLineWidth.set_ptr(loadfn("glLineWidth\0"));
		storage::glLinkProgram.set_ptr(loadfn("glLinkProgram\0"));
		storage::glPixelStorei.set_ptr(loadfn("glPixelStorei\0"));
		storage::glPolygonOffset.set_ptr(loadfn("glPolygonOffset\0"));
		storage::glReadPixels.set_ptr(loadfn("glReadPixels\0"));
		storage::glRenderbufferStorage.set_ptr(loadfn("glRenderbufferStorage\0"));
		storage::glSampleCoverage.set_ptr(loadfn("glSampleCoverage\0"));
		storage::glScissor.set_ptr(loadfn("glScissor\0"));
		storage::glShaderSource.set_ptr(loadfn("glShaderSource\0"));
		storage::glStencilFunc.set_ptr(loadfn("glStencilFunc\0"));
		storage::glStencilFuncSeparate.set_ptr(loadfn("glStencilFuncSeparate\0"));
		storage::glStencilMask.set_ptr(loadfn("glStencilMask\0"));
		storage::glStencilMaskSeparate.set_ptr(loadfn("glStencilMaskSeparate\0"));
		storage::glStencilOp.set_ptr(loadfn("glStencilOp\0"));
		storage::glStencilOpSeparate.set_ptr(loadfn("glStencilOpSeparate\0"));
		storage::glTexImage2D.set_ptr(loadfn("glTexImage2D\0"));
		storage::glTexParameterf.set_ptr(loadfn("glTexParameterf\0"));
		storage::glTexParameterfv.set_ptr(loadfn("glTexParameterfv\0"));
		storage::glTexParameteri.set_ptr(loadfn("glTexParameteri\0"));
		storage::glTexParameteriv.set_ptr(loadfn("glTexParameteriv\0"));
		storage::glTexSubImage2D.set_ptr(loadfn("glTexSubImage2D\0"));
		storage::glUniform1f.set_ptr(loadfn("glUniform1f\0"));
		storage::glUniform1fv.set_ptr(loadfn("glUniform1fv\0"));
		storage::glUniform1i.set_ptr(loadfn("glUniform1i\0"));
		storage::glUniform1iv.set_ptr(loadfn("glUniform1iv\0"));
		storage::glUniform2f.set_ptr(loadfn("glUniform2f\0"));
		storage::glUniform2fv.set_ptr(loadfn("glUniform2fv\0"));
		storage::glUniform2i.set_ptr(loadfn("glUniform2i\0"));
		storage::glUniform2iv.set_ptr(loadfn("glUniform2iv\0"));
		storage::glUniform3f.set_ptr(loadfn("glUniform3f\0"));
		storage::glUniform3fv.set_ptr(loadfn("glUniform3fv\0"));
		storage::glUniform3i.set_ptr(loadfn("glUniform3i\0"));
		storage::glUniform3iv.set_ptr(loadfn("glUniform3iv\0"));
		storage::glUniform4f.set_ptr(loadfn("glUniform4f\0"));
		storage::glUniform4fv.set_ptr(loadfn("glUniform4fv\0"));
		storage::glUniform4i.set_ptr(loadfn("glUniform4i\0"));
		storage::glUniform4iv.set_ptr(loadfn("glUniform4iv\0"));
		storage::glUniformMatrix2fv.set_ptr(loadfn("glUniformMatrix2fv\0"));
		storage::glUniformMatrix3fv.set_ptr(loadfn("glUniformMatrix3fv\0"));
		storage::glUniformMatrix4fv.set_ptr(loadfn("glUniformMatrix4fv\0"));
		storage::glUseProgram.set_ptr(loadfn("glUseProgram\0"));
		storage::glValidateProgram.set_ptr(loadfn("glValidateProgram\0"));
		storage::glVertexAttrib1f.set_ptr(loadfn("glVertexAttrib1f\0"));
		storage::glVertexAttrib1fv.set_ptr(loadfn("glVertexAttrib1fv\0"));
		storage::glVertexAttrib2f.set_ptr(loadfn("glVertexAttrib2f\0"));
		storage::glVertexAttrib2fv.set_ptr(loadfn("glVertexAttrib2fv\0"));
		storage::glVertexAttrib3f.set_ptr(loadfn("glVertexAttrib3f\0"));
		storage::glVertexAttrib3fv.set_ptr(loadfn("glVertexAttrib3fv\0"));
		storage::glVertexAttrib4f.set_ptr(loadfn("glVertexAttrib4f\0"));
		storage::glVertexAttrib4fv.set_ptr(loadfn("glVertexAttrib4fv\0"));
		storage::glVertexAttribPointer.set_ptr(loadfn("glVertexAttribPointer\0"));
		storage::glViewport.set_ptr(loadfn("glViewport\0"));
	}
}
