#![warn(clippy::seek_instead_of_rewind)]

use std::io::{self, SeekFrom};
use std::io::Result;

// Raise lint
fn test1<T: io::Seek>(mut t: T) {
    t.seek(SeekFrom::Start(0));
}

// Do not lint
fn test2<T: io::Seek>(mut t: T) {
    t.seek(SeekFrom::Start(2));
}

// Do not lint
fn test3<T: io::Seek>(mut t: T) {
    t.rewind();
}

struct NonStdSeeker {}

impl NonStdSeeker {
    fn seek(&mut self, _pos: SeekFrom) -> io::Result<u64> {
        Ok(10)
    }
}

fn main() {
    let mut a = NonStdSeeker {};

    // Should not trigger lint
    a.seek(SeekFrom::Start(0));
    a.seek(SeekFrom::Start(1));
}
