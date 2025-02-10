use std::io;

use std::io::BufWriter;
use std::io::Write;

use std::io::BufRead;

pub fn find_prefix(line: &[u8], needle: &[u8]) -> bool {
    if line.len() < needle.len() {
        return false;
    }

    let partial: &[u8] = &line[..needle.len()];
    partial == needle
}

pub fn lines2matched2writer<I, W>(lines: I, mut wtr: W, needle: &[u8]) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<Vec<u8>, io::Error>>,
    W: Write,
{
    for rline in lines {
        let line: Vec<u8> = rline?;
        let found: bool = find_prefix(&line, needle);
        if found {
            wtr.write_all(&line)?;
            wtr.write_all(b"\n")?;
        }
    }
    wtr.flush()
}

pub fn stdin2stdout(needle: &[u8]) -> Result<(), io::Error> {
    let i = io::stdin();
    let il = i.lock();
    let lines = il.split(b'\n');

    let o = io::stdout();
    let mut ol = o.lock();

    let bw = BufWriter::new(&mut ol);
    lines2matched2writer(lines, bw, needle)?;

    ol.flush()
}
