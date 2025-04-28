const std = @import("std");

const allocator = std.heap.page_allocator;
const encoder = std.base64.standard.Encoder;

pub fn main() !void {
    var hex_str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".*;

    const bytes_one = try hex_to_bytes(&hex_str);
    defer allocator.free(bytes_one);

    var best_score: f32 = -100000.0;

    var best_candidate = try allocator.alloc(u8, hex_str.len * 32);
    var best_key: u8 = undefined;
    defer allocator.free(best_candidate);

    for (0..256) |key_attempt| {
        const casted_key_attempt = std.math.cast(u8, key_attempt);
        const candidate = try fixed_xor(bytes_one, casted_key_attempt.?);

        const score = score_english_likeness(candidate);
        if (score > best_score) {
            best_score = score;
            best_candidate = candidate;
            best_key = casted_key_attempt.?;
        }
    }

    std.debug.print("Answer: {s} | Score: {d} | Key: {c}\n", .{ best_candidate, best_score, best_key });
}

fn fixed_xor(input: []u8, key: u8) ![]u8 {
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

fn hex_to_bytes(hex_str: []u8) ![]u8 {
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
