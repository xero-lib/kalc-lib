use super::Type;
use crate::macros::impls::{impl_int_ops, impl_self_ops};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Integer {
    Rug(rug::Integer),
    Fastnum(fastnum::I512),
    F64(i128),
    F32(i128),
}

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rug(a) => a.fmt(f),
            Self::Fastnum(a) => a.fmt(f),
            Self::F64(a) => a.fmt(f),
            Self::F32(a) => a.fmt(f),
        }
    }
}

impl Integer {
    pub fn is_probably_prime(&self, reps: u32) -> bool {
        match self {
            Self::Rug(a) => a.is_probably_prime(reps) != rug::integer::IsPrime::No,
            Self::Fastnum(_) => false,
            Self::F64(_) => false,
            Self::F32(_) => false,
        }
    }
    pub fn next_prime(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.next_prime()),
            Self::Fastnum(a) => Self::Fastnum(a),
            Self::F64(a) => Self::F64(a),
            Self::F32(a) => Self::F32(a),
        }
    }
    pub fn from(obj: Type, val: u32) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Integer::from(val)),
            Type::Fastnum => Self::Fastnum(fastnum::I512::from(val)),
            Type::F64 => Self::F64(val as i128),
            Type::F32 => Self::F32(val as i128),
        }
    }
    pub fn new(obj: Type) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Integer::new()),
            Type::Fastnum => Self::Fastnum(fastnum::I512::from(0)),
            Type::F64 => Self::F64(0),
            Type::F32 => Self::F32(0),
        }
    }
    pub fn to_i128(self) -> i128 {
        match self {
            Self::Rug(a) => a.to_i128().unwrap_or_default(),
            Self::Fastnum(a) => a.to_string().parse().unwrap_or_default(),
            Self::F64(a) => a,
            Self::F32(a) => a,
        }
    }
    pub fn to_string_radix(self, base: i32) -> String {
        match self {
            Self::Rug(a) => a.to_string_radix(base),
            Self::Fastnum(a) => a.to_str_radix(base as u32),
            Self::F64(a) => a.to_string(),
            Self::F32(a) => a.to_string(),
        }
    }
}

impl_self_ops!(Integer, Rug, Fastnum, F64, F32);
impl_int_ops!(Integer, Integer, i32);
