//! hello_world::ferris module
//!
//! This module uses ferris_say module to show messages
//! on the standard output


use ferris_says::say;
use std::io::{stdout, BufWriter};

/// ferris_show shows some message on the standard output
///
#[allow(dead_code)]
pub fn ferris_show(message: &str) {
    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
