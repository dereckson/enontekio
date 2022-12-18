use std::cmp::PartialOrd;
use std::ops::*;
use num_integer::Integer;

/// Description of how two ranges intersect
#[derive(Debug, PartialEq)]
pub enum Intersection {
    /// The self is below the other
    Below,

    /// The self is below but overlapping
    BelowOverlap,

    /// The self is within the other
    Within,

    /// The self is same as the other
    Same,

    /// The self is over the other, the other is within the self
    Over,

    /// The self is above but overlapping
    AboveOverlap,

    /// The self is above the other
    Above,
}

impl Intersection {
    /// Test if there is any intersection
    pub fn is_any(&self) -> bool {
        match self {
            Intersection::Below => false,
            Intersection::Above => false,
            _ => true,
        }
    }

    /// Test if the range is fully within the other
    pub fn is_within(&self) -> bool {
        match self {
            Intersection::Within | Intersection::Same => true,
            _ => false,
        }
    }

    /// Test if the range is fully over the other
    pub fn is_over(&self) -> bool {
        match self {
            Intersection::Over | Intersection::Same => true,
            _ => false,
        }
    }
}

pub trait Intersect<T: PartialOrd, U: RangeBounds<T>>: RangeBounds<T> {
    /// Describes the intersection between two ranges.
    fn intersect(&self, other: &U) -> Intersection;
}

impl<T: PartialOrd> Intersect<T, Range<T>> for Range<T> {
    fn intersect(&self, other: &Range<T>) -> Intersection {
        if self.end == other.end {
            if self.start < other.start {
                Intersection::Over
            } else if self.start > other.start {
                Intersection::Within
            } else {
                Intersection::Same
            }
        } else if self.end < other.end {
            if self.end <= other.start {
                Intersection::Below
            } else if self.start < other.start {
                Intersection::BelowOverlap
            } else {
                Intersection::Within
            }
        } else if self.start < other.end {
            if self.start <= other.start {
                Intersection::Over
            } else {
                Intersection::AboveOverlap
            }
        } else {
            Intersection::Above
        }
    }
}

impl<T: PartialOrd> Intersect<T, RangeFrom<T>> for Range<T> {
    fn intersect(&self, other: &RangeFrom<T>) -> Intersection {
        if self.end <= other.start {
            Intersection::Below
        } else if self.start < other.start {
            Intersection::BelowOverlap
        } else {
            Intersection::Within
        }
    }
}

impl<T: PartialOrd> Intersect<T, RangeFull> for Range<T> {
    fn intersect(&self, _: &RangeFull) -> Intersection {
        Intersection::Within
    }
}

impl<T: PartialOrd> Intersect<T, RangeTo<T>> for Range<T> {
    fn intersect(&self, other: &RangeTo<T>) -> Intersection {
        if self.start >= other.end {
            Intersection::Above
        } else if self.end > other.end {
            Intersection::AboveOverlap
        } else {
            Intersection::Within
        }
    }
}

impl<T: PartialOrd + Integer + Copy> Intersect<T, Range<T>> for RangeInclusive<T> {
    fn intersect(&self, other: &Range<T>) -> Intersection {
        let end = *self.end() + T::one();
        (*self.start()..end).intersect(other)
    }
}

impl<T: PartialOrd + Integer + Copy> Intersect<T, RangeFrom<T>> for RangeInclusive<T> {
    fn intersect(&self, other: &RangeFrom<T>) -> Intersection {
        let end = *self.end() + T::one();
        (*self.start()..end).intersect(other)
    }
}

impl<T: PartialOrd> Intersect<T, RangeFull> for RangeInclusive<T> {
    fn intersect(&self, _: &RangeFull) -> Intersection {
        Intersection::Within
    }
}

impl<T: PartialOrd + Integer + Copy> Intersect<T, RangeTo<T>> for RangeInclusive<T> {
    fn intersect(&self, other: &RangeTo<T>) -> Intersection {
        let end = *self.end() + T::one();
        (*self.start()..end).intersect(other)
    }
}

impl<T: PartialOrd + Copy + Integer> Intersect<T, RangeInclusive<T>> for Range<T> {
    fn intersect(&self, other: &RangeInclusive<T>) -> Intersection {
       let other_end = *other.end() + T::one();
        self.intersect(&(*other.start()..other_end))
    }
}

impl<T: PartialOrd + Integer + Copy> Intersect<T, RangeInclusive<T>> for RangeInclusive<T> {
    fn intersect(&self, other: &RangeInclusive<T>) -> Intersection {
        let end = *self.end() + T::one();
        (*self.start()..end).intersect(other)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_range_intersect() {
        assert_eq!((3..10).intersect(&(11..11)), Intersection::Below);
        assert_eq!((3..10).intersect(&(10..11)), Intersection::Below);
        assert_eq!((3..10).intersect(&(9..11)), Intersection::BelowOverlap);
        assert_eq!((3..10).intersect(&(9..10)), Intersection::Over);
        assert_eq!((3..10).intersect(&(3..10)), Intersection::Same);
        assert_eq!((3..10).intersect(&(5..9)), Intersection::Over);
        assert_eq!((3..10).intersect(&(3..9)), Intersection::Over);
        assert_eq!((3..10).intersect(&(2..11)), Intersection::Within);
        assert_eq!((3..10).intersect(&(3..11)), Intersection::Within);
        assert_eq!((3..10).intersect(&(2..9)), Intersection::AboveOverlap);
        assert_eq!((3..10).intersect(&(2..3)), Intersection::Above);
        assert_eq!((3..10).intersect(&(1..2)), Intersection::Above);

        assert_eq!((3..10).intersect(&(11..)), Intersection::Below);
        assert_eq!((3..10).intersect(&(10..)), Intersection::Below);
        assert_eq!((3..10).intersect(&(9..)), Intersection::BelowOverlap);
        assert_eq!((3..10).intersect(&(3..)), Intersection::Within);
        assert_eq!((3..10).intersect(&(2..)), Intersection::Within);

        assert_eq!((3..10).intersect(&(..)), Intersection::Within);

        assert_eq!((3..10).intersect(&(..11)), Intersection::Within);
        assert_eq!((3..10).intersect(&(..10)), Intersection::Within);
        assert_eq!((3..10).intersect(&(..9)), Intersection::AboveOverlap);
        assert_eq!((3..10).intersect(&(..3)), Intersection::Above);
        assert_eq!((3..10).intersect(&(..2)), Intersection::Above);
    }

    #[test]
    pub fn test_inclusive_range_intersect() {
        assert_eq!((3..=9).intersect(&(11..11)), Intersection::Below);
        assert_eq!((3..=9).intersect(&(10..11)), Intersection::Below);
        assert_eq!((3..=9).intersect(&(9..11)), Intersection::BelowOverlap);
        assert_eq!((3..=9).intersect(&(9..10)), Intersection::Over);
        assert_eq!((3..=9).intersect(&(3..10)), Intersection::Same);
        assert_eq!((3..=9).intersect(&(5..9)), Intersection::Over);
        assert_eq!((3..=9).intersect(&(3..9)), Intersection::Over);
        assert_eq!((3..=9).intersect(&(2..11)), Intersection::Within);
        assert_eq!((3..=9).intersect(&(3..11)), Intersection::Within);
        assert_eq!((3..=9).intersect(&(2..9)), Intersection::AboveOverlap);
        assert_eq!((3..=9).intersect(&(2..3)), Intersection::Above);
        assert_eq!((3..=9).intersect(&(1..2)), Intersection::Above);

        assert_eq!((3..=9).intersect(&(11..)), Intersection::Below);
        assert_eq!((3..=9).intersect(&(10..)), Intersection::Below);
        assert_eq!((3..=9).intersect(&(9..)), Intersection::BelowOverlap);
        assert_eq!((3..=9).intersect(&(3..)), Intersection::Within);
        assert_eq!((3..=9).intersect(&(2..)), Intersection::Within);

        assert_eq!((3..=9).intersect(&(..)), Intersection::Within);

        assert_eq!((3..=9).intersect(&(..11)), Intersection::Within);
        assert_eq!((3..=9).intersect(&(..10)), Intersection::Within);
        assert_eq!((3..=9).intersect(&(..9)), Intersection::AboveOverlap);
        assert_eq!((3..=9).intersect(&(..3)), Intersection::Above);
        assert_eq!((3..=9).intersect(&(..2)), Intersection::Above);
    }

    #[test]
    pub fn test_range_intersect_with_inclusive_range() {
        assert_eq!((3..10).intersect(&(11..=10)), Intersection::Below);
        assert_eq!((3..10).intersect(&(10..=10)), Intersection::Below);
        assert_eq!((3..10).intersect(&(9..=10)), Intersection::BelowOverlap);
        assert_eq!((3..10).intersect(&(9..=9)), Intersection::Over);
        assert_eq!((3..10).intersect(&(3..=9)), Intersection::Same);
        assert_eq!((3..10).intersect(&(5..=8)), Intersection::Over);
        assert_eq!((3..10).intersect(&(3..=8)), Intersection::Over);
        assert_eq!((3..10).intersect(&(2..=10)), Intersection::Within);
        assert_eq!((3..10).intersect(&(3..=10)), Intersection::Within);
        assert_eq!((3..10).intersect(&(2..=8)), Intersection::AboveOverlap);
        assert_eq!((3..10).intersect(&(2..=2)), Intersection::Above);
        assert_eq!((3..10).intersect(&(1..=1)), Intersection::Above);
    }

    #[test]
    pub fn test_inclusive_range_intersect_with_inclusive_range() {
        assert_eq!((3..=9).intersect(&(11..=10)), Intersection::Below);
        assert_eq!((3..=9).intersect(&(10..=10)), Intersection::Below);
        assert_eq!((3..=9).intersect(&(9..=10)), Intersection::BelowOverlap);
        assert_eq!((3..=9).intersect(&(9..=9)), Intersection::Over);
        assert_eq!((3..=9).intersect(&(3..=9)), Intersection::Same);
        assert_eq!((3..=9).intersect(&(5..=8)), Intersection::Over);
        assert_eq!((3..=9).intersect(&(3..=8)), Intersection::Over);
        assert_eq!((3..=9).intersect(&(2..=10)), Intersection::Within);
        assert_eq!((3..=9).intersect(&(3..=10)), Intersection::Within);
        assert_eq!((3..=9).intersect(&(2..=8)), Intersection::AboveOverlap);
        assert_eq!((3..=9).intersect(&(2..=2)), Intersection::Above);
        assert_eq!((3..=9).intersect(&(1..=1)), Intersection::Above);
    }
}
