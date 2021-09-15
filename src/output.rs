use std::fmt::{Display, Formatter};
use std::io::{stdout, Stdout, Write};

#[allow(non_camel_case_types)]
pub enum OutputType {
    iSize(isize),
    Int32(i32),
    Int64(i64),
    uSize(usize),
    uInt32(u32),
    uInt64(u64),
    Bool(bool),
    String(String),
    Vec(Vec<OutputType>),
}

impl From<isize> for OutputType {
    fn from(v: isize) -> Self {
        Self::iSize(v)
    }
}

impl From<i32> for OutputType {
    fn from(v: i32) -> Self {
        Self::Int32(v)
    }
}

impl From<i64> for OutputType {
    fn from(v: i64) -> Self {
        Self::Int64(v)
    }
}

impl From<usize> for OutputType {
    fn from(v: usize) -> Self {
        Self::uSize(v)
    }
}

impl From<u32> for OutputType {
    fn from(v: u32) -> Self {
        Self::uInt32(v)
    }
}

impl From<u64> for OutputType {
    fn from(v: u64) -> Self {
        Self::uInt64(v)
    }
}

impl From<bool> for OutputType {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}

impl From<String> for OutputType {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

impl<T: Into<OutputType>> From<Vec<T>> for OutputType {
    fn from(v: Vec<T>) -> Self {
        Self::Vec(v.into_iter().map(|e| e.into()).collect())
    }
}

impl Display for OutputType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::iSize(value) => write!(f, "{}", *value),
            Self::Int32(value) => write!(f, "{}", *value),
            Self::Int64(value) => write!(f, "{}", *value),
            Self::uSize(value) => write!(f, "{}", *value),
            Self::uInt32(value) => write!(f, "{}", *value),
            Self::uInt64(value) => write!(f, "{}", *value),
            Self::String(value) => write!(f, "{}", *value),
            Self::Bool(value) => {
                if *value {
                    write!(f, "Yes")
                } else {
                    write!(f, "No")
                }
            }
            Self::Vec(v) => {
                write!(
                    f,
                    "{}",
                    v.into_iter()
                        .map(|e| format!("{}", e))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
        }
    }
}

pub struct Output<T: Write>(T);

impl<T: Write> Output<T> {
    pub fn new(destination: T) -> Self {
        Self(destination)
    }

    pub fn write(&mut self, result: OutputType) {
        self.0.write_fmt(format_args!("{}\n", result)).unwrap();
    }
}

impl Default for Output<Stdout> {
    fn default() -> Self {
        Self::new(stdout())
    }
}

#[cfg(test)]
mod test {
    mod output_type {
        use super::super::*;

        #[test]
        fn test_isize() {
            assert_eq!("0", format!("{}", OutputType::from(0isize)));
            assert_eq!("10", format!("{}", OutputType::from(10isize)));
            assert_eq!("-10", format!("{}", OutputType::from(-10isize)));
            assert_eq!("500", format!("{}", OutputType::from(500isize)));
            assert_eq!("-500", format!("{}", OutputType::from(-500isize)));
            assert_eq!(
                format!("{}", isize::MAX),
                format!("{}", OutputType::from(isize::MAX))
            );
            assert_eq!(
                format!("{}", isize::MIN),
                format!("{}", OutputType::from(isize::MIN))
            );
        }

        #[test]
        fn test_i32() {
            assert_eq!("0", format!("{}", OutputType::from(0i32)));
            assert_eq!("10", format!("{}", OutputType::from(10i32)));
            assert_eq!("-10", format!("{}", OutputType::from(-10i32)));
            assert_eq!("500", format!("{}", OutputType::from(500i32)));
            assert_eq!("-500", format!("{}", OutputType::from(-500i32)));
            assert_eq!(
                format!("{}", i32::MAX),
                format!("{}", OutputType::from(i32::MAX))
            );
            assert_eq!(
                format!("{}", i32::MIN),
                format!("{}", OutputType::from(i32::MIN))
            );
        }

        #[test]
        fn test_i64() {
            assert_eq!("0", format!("{}", OutputType::from(0i64)));
            assert_eq!("10", format!("{}", OutputType::from(10i64)));
            assert_eq!("-10", format!("{}", OutputType::from(-10i64)));
            assert_eq!("500", format!("{}", OutputType::from(500i64)));
            assert_eq!("-500", format!("{}", OutputType::from(-500i64)));
            assert_eq!(
                format!("{}", i64::MAX),
                format!("{}", OutputType::from(i64::MAX))
            );
            assert_eq!(
                format!("{}", i64::MIN),
                format!("{}", OutputType::from(i64::MIN))
            );
        }

        #[test]
        fn test_usize() {
            assert_eq!("0", format!("{}", OutputType::from(0usize)));
            assert_eq!("10", format!("{}", OutputType::from(10usize)));
            assert_eq!("500", format!("{}", OutputType::from(500usize)));
            assert_eq!(
                format!("{}", usize::MAX),
                format!("{}", OutputType::from(usize::MAX))
            );
            assert_eq!(
                format!("{}", usize::MIN),
                format!("{}", OutputType::from(usize::MIN))
            );
        }

        #[test]
        fn test_u32() {
            assert_eq!("0", format!("{}", OutputType::from(0u32)));
            assert_eq!("10", format!("{}", OutputType::from(10u32)));
            assert_eq!("500", format!("{}", OutputType::from(500u32)));
            assert_eq!(
                format!("{}", u32::MAX),
                format!("{}", OutputType::from(u32::MAX))
            );
            assert_eq!(
                format!("{}", u32::MIN),
                format!("{}", OutputType::from(u32::MIN))
            );
        }

        #[test]
        fn test_u64() {
            assert_eq!("0", format!("{}", OutputType::from(0u64)));
            assert_eq!("10", format!("{}", OutputType::from(10u64)));
            assert_eq!("500", format!("{}", OutputType::from(500u64)));
            assert_eq!(
                format!("{}", u64::MAX),
                format!("{}", OutputType::from(u64::MAX))
            );
            assert_eq!(
                format!("{}", u64::MIN),
                format!("{}", OutputType::from(u64::MIN))
            );
        }
    }
}
