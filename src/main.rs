use std::process::ExitCode;

use std::io;

fn needle_from_args() -> Result<String, io::Error> {
    std::env::args()
        .nth(1)
        .ok_or_else(|| io::Error::other("no needle specified"))
}

fn sub() -> Result<(), io::Error> {
    let needle: String = needle_from_args()?;
    let s: &[u8] = needle.as_bytes();
    rs_find_prefix_ascii::find::simple::stdin2stdout(s)
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
