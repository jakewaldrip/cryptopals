const std = @import("std");

pub fn main() !void {
    var hex_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".*;

    const allocator = std.heap.page_allocator;
    const bytes_buff = try allocator.alloc(u8, 3072);
    defer allocator.free(bytes_buff);

    const bytes = try hex_to_bytes(&hex_str, bytes_buff);

    const des_len = std.base64.standard.Encoder.calcSize(bytes.len);
    const memory = try allocator.alloc(u8, des_len);
    defer allocator.free(memory);

    const base64_encoded = try bytes_to_b64(bytes, memory);
    std.debug.print("{s}", .{base64_encoded});
    // const res = fixed_xor(base64_encoded);
}

// fn fixed_xor(input: []u8) []u8 {}

fn bytes_to_b64(bytes: []u8, buff: []u8) ![]const u8 {
    const base64_encoded = std.base64.standard.Encoder.encode(buff, bytes);
    return base64_encoded;
}

fn hex_to_bytes(hex_str: []u8, buff: []u8) ![]u8 {
    const bytes = try std.fmt.hexToBytes(buff, hex_str);
    return bytes;
}
