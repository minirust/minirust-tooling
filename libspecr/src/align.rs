use crate::*;

/// `raw` stores the align in bytes.
#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug, Hash)]
pub struct Align { raw: Int }

impl GcCompat for Align {
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.raw.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl Align {
    pub const ONE: Align = Align { raw: Int::ONE };

    /// align is rounded up to the next power of two.
    pub fn from_bytes(align: impl Into<Int>) -> Align {
        let align = align.into();
        let raw = align.next_power_of_two();

        Align { raw }
    }

    pub fn bytes(self) -> Int {
        self.raw
    }

    /// Computes the best alignment possible for the given offset
    /// (the largest power of two that the offset is a multiple of).
    /// For an offset of `0`, it returns None.
    pub fn max_for_offset(offset: Size) -> Option<Align> {
        offset.bytes().trailing_zeros()
            .map(|trailing| {
                let bytes = Int::from(2).pow(trailing);

                Align::from_bytes(bytes)
            })
    }

    /// Lower the alignment, if necessary, such that the given offset
    /// is aligned to it (the offset is a multiple of the alignment).
    pub fn restrict_for_offset(self, offset: Size) -> Align {
        Align::max_for_offset(offset)
            .map(|align| align.min(self))
            .unwrap_or(self)
    }
}
