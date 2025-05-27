use fake::{Fake, StringFaker};

pub fn get_random_string() -> String {
    StringFaker::with(Vec::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"), 12..32).fake()
}
