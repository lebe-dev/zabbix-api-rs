use fake::{Fake, StringFaker};

pub fn get_random_string() -> String {
    StringFaker::with(
        Vec::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"),
        12..32,
    )
    .fake()
}

/// Generates a random 32-character hexadecimal string (16 bytes)
pub fn get_random_hex_string() -> String {
    StringFaker::with(Vec::from("0123456789abcdef"), 32).fake()
}
