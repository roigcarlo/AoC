const std = @import("std");

// Prevent the last char from corrupting the num. Probably much cleaner ways to do this.
pub fn cleanLine(line: []u8) ![]const u8 {
    if (@import("builtin").os.tag == .windows) {
        return std.mem.trimRight(u8, line, "\r");
    } else {
        return line;
    }
}