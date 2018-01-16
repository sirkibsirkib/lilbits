use std::fmt::{Debug,Formatter};
use std::collections::{HashSet,BTreeSet};

/// Set-like data structure for storing u8 elements. Cannot contains values >= 64.
#[derive(Copy,Clone,Serialize,Deserialize,Eq,PartialEq)]
pub struct LilBitSet {
    bits : u64,
}

impl LilBitSet {
    const VALUE_LIMIT : u8 = 63;

    #[inline]
    fn mask() -> u64 { ::std::u64::MAX }
    #[inline]

    /// Creates a new, empty `LilBitSet`.
    pub fn new() -> LilBitSet { LilBitSet {bits : 0} }
    #[inline]
    /// Returns the largest u8 values that IS permitted to be inserted.
    pub fn largest_allowed() -> u8 { Self::VALUE_LIMIT } 
    #[inline]
    /// Constructs a LilBitSet from a `u64`. The indices of positive
    /// bits refers to the presence of a u8 in the set. ie '...1010'
    /// binary for 10 will represent the set {1,3}.
    pub fn new_from_raw(raw: u64) -> Self { LilBitSet {bits: raw} }
    #[inline]
    /// Inverse of `new_from_raw`
    pub fn into_raw(self) -> u64 { self.bits }

    /// Included only as it is a common function for the HashSet. Here it simply
    /// wraps `contains`.
    /// # Panics
    /// panics when element > Self::largest_allowed().
    pub fn get(&self, element: u8) -> Option<u8> {
        if self.contains(element) {
            Some(element)
        } else {
            None
        }
    }

    #[inline]
    /// Returns true IFF the given `u8` is in the set.
    /// # Panics
    /// panics when element > Self::largest_allowed().
    pub fn contains(&self, element : u8) -> bool {
        Self::check_element_is_ok(element);
        ! self.is_empty() //checking this is lightning fast
        && ((1 << element) & self.bits) > 0
    }

    #[inline]
    /// Returns true IFF the set is empty (contains no values).
    pub fn is_empty(&self) -> bool { self.bits == 0 }

    /// Of course this set is not unqiue in its finite value-space for set elements.
    /// However, as there is the additional restriction of x <= Self::largest_allowed()
    /// for any element x, the UNIVERSE of missing elements can be useful
    pub fn universe() -> LilBitSet {
        LilBitSet { bits: Self::mask() }
    }

    #[inline]
    fn check_element_is_ok(element: u8) {
        if element > Self::VALUE_LIMIT {
            panic!("LilBitSet incapable of handling a u8 of that size!");
        }
    }

    /// Attempts to insert the given `u8` into the set. Returns `true` IFF successful.
    /// # Panics
    /// panics when element > Self::largest_allowed().
    pub fn insert(&mut self, element: u8) -> bool {
        Self::check_element_is_ok(element);
        let had = self.contains(element);
        self.bits |= 1 << element;
        !had
    }

    /// Attempts to remove a given `u8` from the set. Returns `true` IFF successful.
    /// # Panics
    /// panics when element >= Self::largest_allowed().
    pub fn remove(&mut self, element: u8) -> bool {
        Self::check_element_is_ok(element);
        let had = self.contains(element);
        self.bits &= Self::mask() - (1 << element);
        had
    }

    /// Returns the set union of this, and the given `LilBitSet`.
    pub fn union(&self, other: &Self) -> Self {
        LilBitSet { bits: self.bits | other.bits }
    }

    /// Returns the set itersection of this, and the given `LilBitSet`.
    pub fn itersection(&self, other: &Self) -> Self {
        LilBitSet { bits: self.bits & other.bits }
    }

    /// Returns the set cardinality (number of elements of the set).
    pub fn len(&self) -> usize {
        self.into_iter().count()
    }

    /// Returns the set symmetric difference of this, and the given `LilBitSet`.
    pub fn symmetric_difference(&self, other: &Self) -> Self {
        LilBitSet { bits: self.bits ^ other.bits }
    }

    /// Relying on `Self::universe()`, this function can be useful in reasoning
    /// over _missing_ elements in a more convenient way
    pub fn complement(&self) -> LilBitSet {
        LilBitSet { bits: Self::mask() - self.bits }
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


/// Iterator over elements inside a `LilBitSet`.
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