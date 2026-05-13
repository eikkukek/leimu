use core::{
    fmt::{Display, self},
    hash::{Hash, self},
    ffi::CStr,
};

/// A constant-evaulated pre-hashed name used by [`Attribute`]s, extensions and features.
///
/// Uses 64-bit FNV-1 hash.
#[derive(Clone, Copy, Debug)]
pub struct ConstName {
    name: &'static str,
    hash: u64,
}

impl Display for ConstName {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ConstName {

    /// Creates a new [`ConstName`].
    pub const fn new(name: &'static str) -> Self {
        let mut hash = 0xcbf29ce484222325u64;
        let len = name.len();
        let bytes = name.as_bytes();
        let mut i = 0;
        while i < len {
            hash ^= bytes[i] as u64;
            hash = hash.wrapping_mul(0x00000100000001b3u64);
            i += 1;
        }
        Self {
            name,
            hash,
        }
    }

    /// Creates a new [`ConstName`] from a static [`CStr`] slice.
    pub const fn from_c_str(name: &'static CStr) -> Self {
        match name.to_str() {
            Ok(name) => Self::new(name),
            Err(_) => panic!("failed to convert to str")
        }
    }
}

impl Hash for ConstName {

    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl PartialEq for ConstName {

    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for ConstName {}
