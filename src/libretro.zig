fn @".?"(optional: type) type {
    return @typeInfo(optional).optional.child;
}
// fn arrayOfStructs(
//     comptime Struct: type,
//     comptime structFieldNames: *const [fields(Struct).len][]const u8,
//     comptime tuples: []const FieldNamesRemovedTuple(Struct, structFieldNames),
// ) [tuples.len]Struct {
//     var structs: [tuples.len]Struct = undefined;
//     for (&structs, tuples) |*@"struct", tuple| {
//         inline for (structFieldNames, fields(@TypeOf(tuple))) |structFieldName, tupleField|
//             @field(@"struct", structFieldName) = @field(tuple, tupleField.name);
//     }
//     return structs;
// }
const assert = debug.assert;
const c = @cImport({
    @cInclude("libretro-common/libretro.h");
    @cDefine("_NO_CRT_STDIO_INLINE", "1");
    @cInclude("stdio.h");
});
const debug = std.debug;
// const fields = meta.fields;
// fn FieldNamesRemovedTuple(comptime Struct: type, comptime names: *const [fields(Struct).len][]const u8) type {
//     var fieldTypes: [fields(Struct).len]type = undefined;
//     for (&fieldTypes, names) |*fieldType, name|
//         fieldType.* = @TypeOf(@field(@as(Struct, undefined), name));
//     return meta.Tuple(&fieldTypes);
// }
const gl = @import("gl");
const heap = std.heap;
const mem = std.mem;
const std = @import("std");
const _ = @import("zigimg");

//////////////////////////////////////////////////////////////////////////////
//                              Core:                                       //
//////////////////////////////////////////////////////////////////////////////
var pkg = .{ .name = "flaretro", .version = "0.1.19" };
const video_width = 624;
const video_height = 336;
const fps = 50;
const sample_rate = 22050;

comptime {
    assert(sample_rate % fps == 0);
    for (@typeInfo(@This()).@"struct".decls) |decl| {
        const field = @field(@This(), decl.name);
        if (@TypeOf(field) != @TypeOf(@field(c, decl.name))) {
            @compileError("pub decl type mismatch: " ++ decl.name);
        }
        @export(&field, .{ .name = decl.name, .linkage = .strong });
    }
}

var allocator: mem.Allocator = undefined;
var audio_batch_cb: @".?"(c.retro_audio_sample_batch_t) = undefined;
var audio_cb: @".?"(c.retro_audio_sample_t) = undefined;
var env_cb: @".?"(c.retro_environment_t) = undefined;
var gpa: heap.GeneralPurposeAllocator(.{}) = undefined;
var input_poll_cb: @".?"(c.retro_input_poll_t) = undefined;
var input_state_cb: @".?"(c.retro_input_state_t) = undefined;
var log_cb: @".?"(c.retro_log_printf_t) = undefined;
var logging: c.retro_log_callback = undefined;
var video_cb: @".?"(c.retro_video_refresh_t) = undefined;
var hw_render: c.retro_hw_render_callback = .{
    .context_type = c.RETRO_HW_CONTEXT_OPENGL,
    .version_major = 2,
    .version_minor = 1,
    .depth = false,
    .stencil = false,
    .bottom_left_origin = true,
    .cache_context = false,
    .debug_context = false,
    .get_current_framebuffer = null,
    .get_proc_address = null,
    .context_reset = &contextReset,
    .context_destroy = &contextDestroy,
};

fn contextReset() callconv(.C) void {
    const gl_procs = &(struct {
        var gl_procs: gl.ProcTable = undefined;
    }.gl_procs);
    assert(gl_procs.init(hw_render.get_proc_address.?));
    gl.makeProcTableCurrent(gl_procs);
    log_cb(c.RETRO_LOG_INFO, "GL_VERSION = %s\n", gl.GetString(gl.VERSION));
}

fn contextDestroy() callconv(.C) void {
    gl.makeProcTableCurrent(null);
}

fn fallbackLog(level: c.retro_log_level, fmt: [*c]const u8, ...) callconv(.C) void {
    _ = level;
    var va = @cVaStart();
    defer @cVaEnd(&va);
    _ = c.vfprintf(c.stderr, fmt, @ptrCast(&va));
}

pub fn retro_init() callconv(.C) void {
    gpa = .{};
    allocator = gpa.allocator();
}

pub fn retro_deinit() callconv(.C) void {
    allocator = undefined;
    _ = gpa.deinit();
    gpa = undefined;
}

pub fn retro_api_version() callconv(.C) c_uint {
    return c.RETRO_API_VERSION;
}

pub fn retro_get_system_info(info: [*c]c.retro_system_info) callconv(.C) void {
    info.* = .{
        .library_name = pkg.name,
        .library_version = pkg.version,
        .valid_extensions = "",
        .need_fullpath = true,
        .block_extract = true,
    };
}

pub fn retro_get_system_av_info(info: [*c]c.retro_system_av_info) callconv(.C) void {
    info.* = .{
        .timing = .{
            .fps = fps,
            .sample_rate = sample_rate,
        },
        .geometry = .{
            .base_width = video_width,
            .base_height = video_height,
            .max_width = video_width,
            .max_height = video_height,
            .aspect_ratio = 0.0,
        },
    };
}

pub fn retro_set_environment(cb: c.retro_environment_t) callconv(.C) void {
    env_cb = cb.?;
    log_cb = if (env_cb(c.RETRO_ENVIRONMENT_GET_LOG_INTERFACE, &logging))
        logging.log.?
    else
        fallbackLog;
    {
        var support_no_game = true;
        _ = env_cb(c.RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME, &support_no_game);
    }
}

pub fn retro_set_video_refresh(cb: c.retro_video_refresh_t) callconv(.C) void {
    video_cb = cb.?;
}

pub fn retro_set_audio_sample(cb: c.retro_audio_sample_t) callconv(.C) void {
    audio_cb = cb.?;
}

pub fn retro_set_audio_sample_batch(cb: c.retro_audio_sample_batch_t) callconv(.C) void {
    audio_batch_cb = cb.?;
}

pub fn retro_set_input_poll(cb: c.retro_input_poll_t) callconv(.C) void {
    input_poll_cb = cb.?;
}

pub fn retro_set_input_state(cb: c.retro_input_state_t) callconv(.C) void {
    input_state_cb = cb.?;
}

pub fn retro_set_controller_port_device(port: c_uint, device: c_uint) callconv(.C) void {
    _, _ = .{ port, device };
}

pub fn retro_reset() callconv(.C) void {}

pub fn retro_run() callconv(.C) void {
    input_poll_cb();

    // glBindFramebuffer(GL_FRAMEBUFFER, (HW_RENDER.get_current_framebuffer.unwrap_unchecked())() as _);
    // static mut FRAME_COUNT: Wrapping<usize> = Wrapping(0);
    // let f: f32 = if FRAME_COUNT.0 % 64 <= 31 { 0.33 } else { 0.67 };
    // FRAME_COUNT += 1;
    // glClearColor(f, f, f, f);
    // glViewport(0, 0, VIDEO_WIDTH as _, VIDEO_HEIGHT as _);
    // glClear(GL_COLOR_BUFFER_BIT);
    // glUseProgram(SHAD_PROG);
    // glEnableVertexAttribArray(ATTR_COORD2D);
    // glBindBuffer(GL_ARRAY_BUFFER, VBO_TRIANGLE);
    // glVertexAttribPointer(ATTR_COORD2D, 2, GL_FLOAT, GL_FALSE, 0, null());
    // glDrawArrays(GL_TRIANGLES, 0, 3);
    // glBindBuffer(GL_ARRAY_BUFFER, 0);
    // glDisableVertexAttribArray(ATTR_COORD2D);
    // glUseProgram(0);
    // {
    //     VIDEO_CB(RETRO_HW_FRAME_BUFFER_VALID, VIDEO_WIDTH as _, VIDEO_HEIGHT as _, IRRELEVANT);
    //     const IRRELEVANT: size_t = size_t::MAX;
    // }
    // {
    //     const NUM_AUDIO_FRAMES: usize = SAMPLE_RATE / FPS;
    //     const NUM_AUDIO_CHANNELS: usize = 2;
    //     const SILENCE_SAMPLES: &[i16] = &[0; NUM_AUDIO_FRAMES * NUM_AUDIO_CHANNELS];
    //     AUDIO_BATCH_CB(ptr!(SILENCE_SAMPLES), NUM_AUDIO_FRAMES as _);
    // }
}

pub fn retro_serialize_size() callconv(.C) usize {
    return 0;
}

pub fn retro_serialize(data: ?*anyopaque, size: usize) callconv(.C) bool {
    _ = .{ data, size };
    return false;
}

pub fn retro_unserialize(data: ?*const anyopaque, size: usize) callconv(.C) bool {
    _ = .{ data, size };
    return false;
}

pub fn retro_cheat_reset() callconv(.C) void {}

pub fn retro_cheat_set(index: c_uint, enabled: bool, code: [*c]const u8) callconv(.C) void {
    _, _, _ = .{ index, enabled, code };
}

pub fn retro_load_game(info: [*c]const c.retro_game_info) callconv(.C) bool {
    if (info != null) {
        const path = info.*.path;
        if (path != null) {
            // zig fmt: off
            log_cb(
                c.RETRO_LOG_ERROR,
                    \\
                    \\  This core doesn't support specifying content files / paths explicitly.
                    \\  Please remove the "%s" argument.
                    \\
                ,
                path,
            );
            // zig fmt: on
            return false;
        }
    }
    {
        const pixel_format_name = "RETRO_PIXEL_FORMAT_XRGB8888";
        var pixel_format = @field(c, pixel_format_name);
        if (!env_cb(c.RETRO_ENVIRONMENT_SET_PIXEL_FORMAT, &pixel_format)) {
            log_cb(c.RETRO_LOG_ERROR, pixel_format_name ++ " is not supported.\n");
            return false;
        }
    }
    if (!env_cb(c.RETRO_ENVIRONMENT_SET_HW_RENDER, &hw_render)) {
        log_cb(c.RETRO_LOG_ERROR, "HW Context could not be initialized.\n");
        return false;
    }
    return true;
}

pub fn retro_load_game_special(
    game_type: c_uint,
    info: [*c]const c.struct_retro_game_info,
    num_info: usize,
) callconv(.C) bool {
    _, _, _ = .{ game_type, info, num_info };
    return false;
}

pub fn retro_unload_game() callconv(.C) void {}

pub fn retro_get_region() callconv(.C) c_uint {
    return c.RETRO_REGION_PAL;
}

pub fn retro_get_memory_data(id: c_uint) callconv(.C) ?*anyopaque {
    _ = id;
    return null;
}

pub fn retro_get_memory_size(id: c_uint) callconv(.C) usize {
    _ = id;
    return 0;
}
