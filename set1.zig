const std = @import("std");
const file_input = @embedFile("./file.txt");

const allocator = std.heap.page_allocator;
const encoder = std.base64.standard.Encoder;

pub fn main() !void {
    var file_iter = std.mem.splitSequence(u8, file_input, "\n");
    const line_len = file_iter.first().len;
    file_iter.reset();

    var best_candidate = try allocator.alloc(u8, line_len * 32);
    defer allocator.free(best_candidate);

    var best_key: u8 = undefined;
    var best_score: f32 = -100000.0;

    var idx: i32 = 0;
    while (file_iter.next()) |line| {
        idx = idx + 1;

        const line_slice = if (idx != 327) line[0 .. line.len - 1] else line[0..line.len];
        const line_bytes = try hex_to_bytes(line_slice);
        defer allocator.free(line_bytes);

        for (0..256) |key_attempt| {
            const casted_key_attempt = std.math.cast(u8, key_attempt);
            const candidate = try fixed_xor(line_bytes, casted_key_attempt.?);

            const score = score_english_likeness(candidate);
            if (score > best_score) {
                best_score = score;
                best_candidate = candidate;
                best_key = casted_key_attempt.?;
            }
        }
    }

    std.debug.print("Answer: {s} | Score: {d} | Key: {c}\n", .{ best_candidate, best_score, best_key });
}

fn fixed_xor(input: []const u8, key: u8) ![]u8 {
    var result = try allocator.alloc(u8, input.len);
    for (input, 0..) |one, i| {
        result[i] = one ^ key;
    }
    return result;
}

fn bytes_to_b64(bytes: []u8) ![]const u8 {
    const des_len = std.base64.standard.Encoder.calcSize(bytes.len);
    const buff = try allocator.alloc(u8, des_len);
    const base64_encoded = encoder.encode(buff, bytes);
    return base64_encoded;
}

fn hex_to_bytes(hex_str: []const u8) ![]u8 {
    const bytes_buff = try allocator.alloc(u8, hex_str.len * 32);
    const bytes = try std.fmt.hexToBytes(bytes_buff, hex_str);
    return bytes;
}

fn score_english_likeness(input: []u8) f32 {
    var score: f32 = 0;
    for (input) |c| {
        const char = std.ascii.toLower(c);
        if (char >= 'a' and char <= 'z') {
            score += 1.0;
        } else if (char == ' ' or char == '\n') {
            score += 0.5;
        } else if (char >= 32 and char <= 126) {
            score += 0.1;
        } else {
            score -= 1.0;
        }
    }

    return score;
}
