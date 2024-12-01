const std = @import("std");

// @import("irony") The doc from zig is super clear,
// I will be having a good time with this lang
pub fn build(b: *std.Build) void {
    const target    = b.standardTargetOptions(.{});
    const optimize  = b.standardOptimizeOption(.{});

    // Libs
    const utils = b.addModule("utils", .{
        .root_source_file = b.path("utils/utils.zig"),
    });

    // Build
    const d01 = b.addExecutable(.{
        .name = "d01",
        .root_source_file = b.path("d01/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    d01.root_module.addImport("utils", utils);

    b.installArtifact(d01);

    const run_cmd = b.addRunArtifact(d01);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }


    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);

    // Test
    const d01_test = b.addTest(.{
        .root_source_file = b.path("d01/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    // Link again because patata.
    d01_test.root_module.addImport("utils", utils);

    const run_exe_unit_tests = b.addRunArtifact(d01_test);

    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_exe_unit_tests.step);
}
