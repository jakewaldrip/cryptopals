const std = @import("std");

const allocator = std.heap.page_allocator;
const encoder = std.base64.standard.Encoder;

pub fn main() !void {
    var hex_str_one = "1c0111001f010100061a024b53535009181c".*;
    var hex_str_two = "686974207468652062756c6c277320657965".*;

    const bytes_one = try hex_to_bytes(&hex_str_one);
    defer allocator.free(bytes_one);

    const bytes_two = try hex_to_bytes(&hex_str_two);
    defer allocator.free(bytes_two);

    const res = fixed_xor(bytes_one, bytes_two);
    defer allocator.free(res);

    std.debug.print("Hex: {s}", .{std.fmt.fmtSliceHexLower(res)});
}

fn fixed_xor(input_one: []u8, input_two: []u8) []u8 {
    var result = allocator.alloc(u8, input_one.len) catch unreachable;
    for (input_one, input_two, 0..) |one, two, i| {
        result[i] = one ^ two;
    }
    return result;
}

fn bytes_to_b64(bytes: []u8) ![]const u8 {
    const des_len = std.base64.standard.Encoder.calcSize(bytes.len);
    const buff = try allocator.alloc(u8, des_len);
    const base64_encoded = encoder.encode(buff, bytes);
    return base64_encoded;
}

fn hex_to_bytes(hex_str: []u8) ![]u8 {
    const bytes_buff = try allocator.alloc(u8, hex_str.len * 32);
    const bytes = try std.fmt.hexToBytes(bytes_buff, hex_str);
    return bytes;
}
