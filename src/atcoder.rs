use crate::input::Input;
use crate::output::{Output, OutputType};
use std::io::{Read, Stdin, Stdout, Write};

pub trait Solver: Sized {
    fn solve<R: Read, O: Into<OutputType>>(&mut self, input: &mut Input<R>) -> O;
}

pub struct Atcoder<R: Read, W: Write> {
    input: Input<R>,
    output: Output<W>,
}

impl<R: Read, W: Write> Atcoder<R, W> {
    pub fn with_io(input: Input<R>, output: Output<W>) -> Self {
        Self { input, output }
    }

    pub fn run<T: Solver, O: Into<OutputType>>(&mut self, solver: T) {
        let mut solver = solver;
        let result = solver.solve::<R, O>(&mut self.input).into();
        self.output.write(result);
    }
}

impl Default for Atcoder<Stdin, Stdout> {
    fn default() -> Self {
        Self {
            input: Input::<Stdin>::default(),
            output: Output::<Stdout>::default(),
        }
    }
}
