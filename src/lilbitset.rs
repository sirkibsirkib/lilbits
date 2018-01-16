// use serde::{Deserialize, Serialize};
// use std::fmt;
use std::fmt::{Debug,Formatter};
use std::collections::{HashSet,BTreeSet};



#[derive(Copy,Clone,Serialize,Deserialize,Eq,PartialEq)]
pub struct LilBitSet {
    bits : u64,
}

impl LilBitSet {
    const VALUE_LIMIT : u8 = 63;

    #[inline]
    fn mask() -> u64 { ::std::u64::MAX }
    #[inline]
    pub fn new() -> LilBitSet { LilBitSet {bits : 0} }
    #[inline]
    pub fn largest_allowed() -> u8 { Self::VALUE_LIMIT-1 } 
    #[inline]
    pub fn new_from_raw(raw: u64) -> Self { LilBitSet {bits: raw} }
    #[inline]
    pub fn into_raw(self) -> u64 { self.bits }

    pub fn get(&self, element: u8) -> Option<u8> {
        if self.contains(element) {
            Some(element)
        } else {
            None
        }
    }

    #[inline]
    pub fn contains(&self, element : u8) -> bool {
        Self::check_element_is_ok(element);
        ! self.is_empty() //checking this is lightning fast
        && ((1 << element) & self.bits) > 0
    }

    #[inline]
    pub fn is_empty(&self) -> bool { self.bits == 0 }

    #[inline]
    fn check_element_is_ok(element: u8) {
        if element > Self::VALUE_LIMIT {
            panic!("LilBitSet incapable of handling a u8 of that size!");
        }
    }

    pub fn insert(&mut self, element: u8) -> bool {
        Self::check_element_is_ok(element);
        let had = self.contains(element);
        self.bits |= 1 << element;
        !had
    }

    pub fn try_insert(&mut self, element: u8) -> bool {
        if element > Self::VALUE_LIMIT {
            return false;
        }
        let had = self.contains(element);
        self.bits |= 1 << element;
        !had
    }

    pub fn remove(&mut self, element: u8) -> bool {
        Self::check_element_is_ok(element);
        let had = self.contains(element);
        self.bits &= Self::mask() - (1 << element);
        had
    }

    pub fn union(&self, other: Self) -> Self {
        LilBitSet { bits: self.bits | other.bits }
    }

    pub fn itersection(&self, other: Self) -> Self {
        LilBitSet { bits: self.bits & other.bits }
    }

    pub fn len(&self) -> usize {
        self.into_iter().count()
    }

    pub fn symmetric_difference(&self, other: Self) -> Self {
        LilBitSet { bits: self.bits ^ other.bits }
    }

    pub fn complement(&self) -> LilBitSet {
        LilBitSet { bits: Self::mask() - self.bits }
    }

    pub fn universe(&self) -> LilBitSet {
        LilBitSet { bits: Self::mask() }
    }
}

impl Into<HashSet<u8>> for LilBitSet {
    fn into(self) -> HashSet<u8> { self.into_iter().collect() }
}
impl Into<BTreeSet<u8>> for LilBitSet {
    fn into(self) -> BTreeSet<u8> { self.into_iter().collect() }
}

impl Into<LilBitSet> for HashSet<u8> {
    fn into(self) -> LilBitSet { self.into_iter().collect() }
}
impl Into<LilBitSet> for BTreeSet<u8> {
    fn into(self) -> LilBitSet { self.into_iter().collect() }
}


impl Debug for LilBitSet {
    fn fmt(&self, f: &mut Formatter) -> Result<(), ::std::fmt::Error> {
        let mut first = true;
        write!(f, "{{")?;
        for value in 0..64 {
            if self.contains(value) {
                if !first {
                    write!(f, ",")?;
                } else {
                    first = false;
                }
                write!(f, "{:?}", value)?;
            }
        }
        write!(f, "}}")
    }
}

impl IntoIterator for LilBitSet {
    type Item = u8;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { bit_set: self, index: 0 }
    }
}


pub struct IntoIter {
    bit_set : LilBitSet,
    index : u8,
}

impl Iterator for IntoIter {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        while self.index <= LilBitSet::VALUE_LIMIT {
            self.index += 1;
            if self.bit_set.contains(self.index-1) {
                return Some(self.index-1)
            }
        }
        None
    }
}

impl ::std::iter::FromIterator<u8> for LilBitSet {
    fn from_iter<I: IntoIterator<Item=u8>>(iter: I) -> Self {
        let mut lilbitset = LilBitSet::new();
        for i in iter {
            lilbitset.insert(i);
        }
        lilbitset
    }
}

impl<'a> ::std::iter::FromIterator<&'a u8> for LilBitSet {
    fn from_iter<I: IntoIterator<Item=&'a u8>>(iter: I) -> Self {
        let mut lilbitset = LilBitSet::new();
        for i in iter {
            lilbitset.insert(*i);
        }
        lilbitset
    }
}

#[macro_export]
/// This code is adapted from github.com/bluss/maplit (MIT license)
macro_rules! lilbits {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(lilbits!(@single $rest)),*]));

    ($($key:expr,)+) => { lilbits!($($key),+) };
    ($($key:expr),*) => {
        {
            let _cap = lilbits!(@count $($key),*);
            let mut _set = LilBitSet::new();
            $(
                let _ = _set.insert($key);
            )*
            _set
        }
    };
}