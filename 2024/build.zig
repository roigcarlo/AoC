const std = @import("std");

const target_options = struct {
    name: []const u8,
    source: []const u8,
    active: bool,
};

const problems = [_]target_options{
    .{
        .name = "d01",
        .source = "d01/main.zig",
        .active = false,
    },
    .{
        .name = "d02",
        .source = "d02/main.zig",
        .active = true,
    },
};

// @import("irony") The doc from zig is super clear,
// I will be having a good time with this lang
pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Libs
    const utils = b.addModule("utils", .{
        .root_source_file = b.path("utils/utils.zig"),
    });

    const run_step = b.step("run", "Run the app");
    const test_step = b.step("test", "Run unit tests");

    for (problems) |problem| {
        const problem_exe = b.addExecutable(.{
            .name = problem.name,
            .root_source_file = b.path(problem.source),
            .target = target,
            .optimize = optimize,
        });

        problem_exe.root_module.addImport("utils", utils);
        b.installArtifact(problem_exe);

        const run_cmd = b.addRunArtifact(problem_exe);
        run_cmd.step.dependOn(b.getInstallStep());

        if (b.args) |args| {
            run_cmd.addArgs(args);
        }

        run_step.dependOn(&run_cmd.step);

        // Test
        const problem_test = b.addTest(.{
            .root_source_file = b.path(problem.source),
            .target = target,
            .optimize = optimize,
        });

        // Link again because patata.
        problem_test.root_module.addImport("utils", utils);

        const run_exe_unit_tests = b.addRunArtifact(problem_test);
        run_exe_unit_tests.has_side_effects = problem.active;

        test_step.dependOn(&run_exe_unit_tests.step);
    }
}
