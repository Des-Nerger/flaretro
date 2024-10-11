const std = @import("std");

// Although this function looks imperative, note that its job is to
// declaratively construct a build graph that will be executed by an external
// runner.
pub fn build(b: *std.Build) void {
    // Standard target options allows the person running `zig build` to choose
    // what target to build for. Here we do not override the defaults, which
    // means any target is allowed, and the default is native. Other options
    // for restricting supported target set are available.
    const target = b.standardTargetOptions(.{});

    // Standard optimization options allow the person running `zig build` to select
    // between Debug, ReleaseSafe, ReleaseFast, and ReleaseSmall.
    // Here we also set a preferred release mode.
    const optimize = b.standardOptimizeOption(.{ .preferred_optimize_mode = .ReleaseSafe });

    const lib = b.addSharedLibrary(.{
        .name = "flaretro",
        // In this case the main source file is merely a path, however, in more
        // complicated build scripts, this could be a generated file.
        .root_source_file = b.path("src/libretro.zig"),
        .target = target,
        .optimize = optimize,
    });
    lib.linkLibC();

    const zigimg = b.dependency("zigimg", .{
        .target = target,
        .optimize = optimize,
    });
    lib.root_module.addImport("zigimg", zigimg.module("zigimg"));

    // Choose the OpenGL API, version, profile and extensions you want to generate bindings for.
    const gl_bindings = @import("zigglgen").generateBindingsModule(b, .{
        .api = .gl,
        .version = .@"2.1",
        .extensions = &.{.ARB_framebuffer_object},
    });

    // Import the generated module.
    lib.root_module.addImport("gl", gl_bindings);

    // This declares intent for the library to be installed when the user
    // invokes the "install" step (the default step when running `zig build`).
    const install_artifact = b.addInstallArtifact(lib, .{
        .dest_dir = .{ .override = .prefix },
    });
    b.getInstallStep().dependOn(&install_artifact.step);

    // const exe = b.addExecutable(.{
    //     .name = "flaretro",
    //     .root_source_file = b.path("src/main.zig"),
    //     .target = target,
    //     .optimize = optimize,
    // });

    // // This declares intent for the executable to be installed into the
    // // standard location when the user invokes the "install" step (the default
    // // step when running `zig build`).
    // b.installArtifact(exe);

    // // This *creates* a Run step in the build graph, to be executed when another
    // // step is evaluated that depends on it. The next line below will establish
    // // such a dependency.
    // const run_cmd = b.addRunArtifact(exe);

    // // By making the run step depend on the install step, it will be run from the
    // // installation directory rather than directly from within the cache directory.
    // // This is not necessary, however, if the application depends on other installed
    // // files, this ensures they will be present and in the expected location.
    // run_cmd.step.dependOn(b.getInstallStep());

    // // This allows the user to pass arguments to the application in the build
    // // command itself, like this: `zig build run -- arg1 arg2 etc`
    // if (b.args) |args| {
    //     run_cmd.addArgs(args);
    // }

    // // This creates a build step. It will be visible in the `zig build --help` menu,
    // // and can be selected like this: `zig build run`
    // // This will evaluate the `run` step rather than the default, which is "install".
    // const run_step = b.step("run", "Run the app");
    // run_step.dependOn(&run_cmd.step);
}
