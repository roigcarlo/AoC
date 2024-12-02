const std = @import("std");
const cleanLine = @import("utils").cleanLine;

const print = std.debug.print;

test "Day 01 part 01" {
    // Read a file
    var file = try std.fs.cwd().openFile("d01/data", .{});
    defer file.close();

    var buffered = std.io.bufferedReader(file.reader());
    const buffredr = buffered.reader();

    var buff: [1024]u8 = undefined;

    var list_a = std.ArrayList(i32).init(std.testing.allocator);
    var list_b = std.ArrayList(i32).init(std.testing.allocator);

    defer list_a.deinit();
    defer list_b.deinit();

    while (try buffredr.readUntilDelimiterOrEof(buff[0..], '\n')) |line| {
        var it = std.mem.tokenizeSequence(u8, try cleanLine(line), " ");

        try list_a.append(try std.fmt.parseInt(i32, it.next().?, 10));
        try list_b.append(try std.fmt.parseInt(i32, it.next().?, 10));
    }

    // Actually solving the problem
    std.mem.sort(i32, list_a.items, {}, std.sort.asc(i32));
    std.mem.sort(i32, list_b.items, {}, std.sort.asc(i32));

    // Iterating over both lists using the iterator pattern
    var sum: u32 = 0;
    for (list_a.items, list_b.items) |x, y| {
        sum += @abs(x - y);
    }

    // Print Solution
    print("Solution P1: {}\n", .{sum});

    // Check
    try std.testing.expectEqual(sum, @as(i32, 765748));
}

test "Day 01 part 02" {
    // Read a file
    var file = try std.fs.cwd().openFile("d01/data", .{});
    defer file.close();

    var buffered = std.io.bufferedReader(file.reader());
    const buffredr = buffered.reader();

    var buff: [1024]u8 = undefined;

    var list_a = std.ArrayList(i32).init(std.testing.allocator);
    var countr = std.AutoHashMap(i32, i32).init(std.testing.allocator);

    defer list_a.deinit();
    defer countr.deinit();

    while (try buffredr.readUntilDelimiterOrEof(buff[0..], '\n')) |line| {
        var it = std.mem.tokenizeSequence(u8, try cleanLine(line), " ");

        const idx: i32 = try std.fmt.parseInt(i32, it.next().?, 10);
        const val: i32 = try std.fmt.parseInt(i32, it.next().?, 10);

        try list_a.append(idx);
        try countr.put(val, @as(i32, 1) + (countr.get(val) orelse 0));
    }

    // Iterating over both lists using the iterator pattern
    var sum: i32 = 0;
    for (list_a.items) |idx| {
        sum += idx * (countr.get(idx) orelse 0);
    }

    // Print Solution
    print("Solution P2: {}\n", .{sum});

    // Check
    try std.testing.expectEqual(sum, @as(i32, 27732508));
}
