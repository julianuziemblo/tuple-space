use std::ops::RangeBounds;

pub struct SliceU8<'a>(pub &'a [u8]);

impl<'a> From<&'a [u8]> for SliceU8<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self(value)
    }
}

pub struct VecU8(pub Vec<u8>);

impl From<Vec<u8>> for VecU8 {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl<'a> std::fmt::Binary for SliceU8<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !f.alternate() {
            write!(f, "\n  ")?;
            for i in 0..f.width().unwrap_or(4) {
                write!(f, "{i}        ")?;
            }
            return self.0.iter().enumerate().try_for_each(|(i, &e)| {
                if i % f.width().unwrap_or(4) == 0 {
                    write!(f, "\n{} ", i / 4)?;
                }
                write!(f, "{e:08b} ")
            });
        }
        self.0.iter().try_for_each(|&e| write!(f, "{e:08b} "))
    }
}

impl std::fmt::Binary for VecU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        SliceU8(&self.0).fmt(f)
    }
}

pub trait Serializable: Sized {
    type Error;
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Error>;
}

pub fn take_first_n_const<'a, T, const N: usize>(
    collection: &'a [T],
) -> Result<[T; N], TakeIndexError>
where
    [T; N]: std::convert::TryFrom<&'a [T]>,
{
    if collection.len() >= N {
        collection[..N].try_into().map_err(|_| TakeIndexError(N))
    } else {
        Err(TakeIndexError(N))
    }
}

pub fn take_range<T, R>(collection: &[T], range: R) -> Result<&[T], TakeIndexError>
where
    R: RangeBounds<usize> + std::slice::SliceIndex<[T], Output = [T]>,
{
    use std::ops::Bound as B;
    let hi = match range.end_bound() {
        B::Included(&high) => {
            if high >= collection.len() {
                return Err(TakeIndexError(high));
            }
            high
        }
        B::Excluded(&high) => {
            if high > collection.len() {
                return Err(TakeIndexError(high));
            }
            high
        }
        B::Unbounded => collection.len(),
    };
    let lo = match range.start_bound() {
        B::Included(&low) => low,
        B::Excluded(&low) => low,
        B::Unbounded => 0,
    };

    if hi < lo {
        return Err(TakeIndexError(lo));
    }

    Ok(&collection[range])
}

#[derive(Clone, Copy, Debug)]
pub struct TakeIndexError(pub usize);

impl std::fmt::Display for TakeIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TakeIndexError: The provided index {} is outside of the bounds of the provided collection.",
            self.0
        )
    }
}

impl std::error::Error for TakeIndexError {}
