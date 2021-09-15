use std::fmt::Debug;
use std::io::{stdin, Read, Stdin};
use std::str::FromStr;

static mut _INPUT_BUF: String = String::new();

fn write_buf<T: Read>(source: &mut T) {
    unsafe {
        source.read_to_string(&mut _INPUT_BUF).unwrap();
    }
}

fn read_buf() -> &'static str {
    unsafe { &_INPUT_BUF }
}

pub struct Input<T: Read> {
    #[allow(dead_code)]
    source: T,
    iter: Box<dyn Iterator<Item = &'static str>>,
}

impl<T: Read> Input<T> {
    pub fn new(mut source: T) -> Self {
        write_buf(&mut source);
        let iter = Box::new(read_buf().split_ascii_whitespace());
        Self { source, iter }
    }

    pub fn read<U>(&mut self) -> U
    where
        U: FromStr,
        U::Err: Debug,
    {
        self.iter.next().unwrap().parse().unwrap()
    }
}

impl Default for Input<Stdin> {
    fn default() -> Self {
        Self::new(stdin())
    }
}
