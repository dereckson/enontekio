use std::cmp::PartialOrd;
use std::ops::*;
use num_integer::Integer;

/// Description of how two ranges intersect
#[derive(Debug, PartialEq)]
pub enum IntersectionDescription {
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

impl IntersectionDescription {
    /// Tests if there is any intersection
    pub fn is_any(&self) -> bool {
        match self {
            IntersectionDescription::Below => false,
            IntersectionDescription::Above => false,
            _ => true,
        }
    }

    /// Tests if the range is fully within the other
    pub fn is_within(&self) -> bool {
        match self {
            IntersectionDescription::Within | IntersectionDescription::Same => true,
            _ => false,
        }
    }

    /// Tests if the range is fully over the other
    pub fn is_over(&self) -> bool {
        match self {
            IntersectionDescription::Over | IntersectionDescription::Same => true,
            _ => false,
        }
    }
}

pub trait Intersect<T: PartialOrd, U: RangeBounds<T>>: RangeBounds<T> {
    /// Describes the intersection between two ranges.
    fn describe_intersection(&self, other: &U) -> IntersectionDescription;
}

impl<T: PartialOrd> Intersect<T, Range<T>> for Range<T> {
    fn describe_intersection(&self, other: &Range<T>) -> IntersectionDescription {
        if self.end == other.end {
            if self.start < other.start {
                IntersectionDescription::Over
            } else if self.start > other.start {
                IntersectionDescription::Within
            } else {
                IntersectionDescription::Same
            }
        } else if self.end < other.end {
            if self.end <= other.start {
                IntersectionDescription::Below
            } else if self.start < other.start {
                IntersectionDescription::BelowOverlap
            } else {
                IntersectionDescription::Within
            }
        } else if self.start < other.end {
            if self.start <= other.start {
                IntersectionDescription::Over
            } else {
                IntersectionDescription::AboveOverlap
            }
        } else {
            IntersectionDescription::Above
        }
    }
}

impl<T: PartialOrd> Intersect<T, RangeFrom<T>> for Range<T> {
    fn describe_intersection(&self, other: &RangeFrom<T>) -> IntersectionDescription {
        if self.end <= other.start {
            IntersectionDescription::Below
        } else if self.start < other.start {
            IntersectionDescription::BelowOverlap
        } else {
            IntersectionDescription::Within
        }
    }
}

impl<T: PartialOrd> Intersect<T, RangeFull> for Range<T> {
    fn describe_intersection(&self, _: &RangeFull) -> IntersectionDescription {
        IntersectionDescription::Within
    }
}

impl<T: PartialOrd> Intersect<T, RangeTo<T>> for Range<T> {
    fn describe_intersection(&self, other: &RangeTo<T>) -> IntersectionDescription {
        if self.start >= other.end {
            IntersectionDescription::Above
        } else if self.end > other.end {
            IntersectionDescription::AboveOverlap
        } else {
            IntersectionDescription::Within
        }
    }
}

impl<T: PartialOrd + Integer + Copy> Intersect<T, Range<T>> for RangeInclusive<T> {
    fn describe_intersection(&self, other: &Range<T>) -> IntersectionDescription {
        let end = *self.end() + T::one();
        (*self.start()..end).describe_intersection(other)
    }
}

impl<T: PartialOrd + Integer + Copy> Intersect<T, RangeFrom<T>> for RangeInclusive<T> {
    fn describe_intersection(&self, other: &RangeFrom<T>) -> IntersectionDescription {
        let end = *self.end() + T::one();
        (*self.start()..end).describe_intersection(other)
    }
}

impl<T: PartialOrd> Intersect<T, RangeFull> for RangeInclusive<T> {
    fn describe_intersection(&self, _: &RangeFull) -> IntersectionDescription {
        IntersectionDescription::Within
    }
}

impl<T: PartialOrd + Integer + Copy> Intersect<T, RangeTo<T>> for RangeInclusive<T> {
    fn describe_intersection(&self, other: &RangeTo<T>) -> IntersectionDescription {
        let end = *self.end() + T::one();
        (*self.start()..end).describe_intersection(other)
    }
}

impl<T: PartialOrd + Copy + Integer> Intersect<T, RangeInclusive<T>> for Range<T> {
    fn describe_intersection(&self, other: &RangeInclusive<T>) -> IntersectionDescription {
       let other_end = *other.end() + T::one();
        self.describe_intersection(&(*other.start()..other_end))
    }
}

impl<T: PartialOrd + Integer + Copy> Intersect<T, RangeInclusive<T>> for RangeInclusive<T> {
    fn describe_intersection(&self, other: &RangeInclusive<T>) -> IntersectionDescription {
        let end = *self.end() + T::one();
        (*self.start()..end).describe_intersection(other)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_range_intersect() {
        assert_eq!((3..10).describe_intersection(&(11..11)), IntersectionDescription::Below);
        assert_eq!((3..10).describe_intersection(&(10..11)), IntersectionDescription::Below);
        assert_eq!((3..10).describe_intersection(&(9..11)), IntersectionDescription::BelowOverlap);
        assert_eq!((3..10).describe_intersection(&(9..10)), IntersectionDescription::Over);
        assert_eq!((3..10).describe_intersection(&(3..10)), IntersectionDescription::Same);
        assert_eq!((3..10).describe_intersection(&(5..9)), IntersectionDescription::Over);
        assert_eq!((3..10).describe_intersection(&(3..9)), IntersectionDescription::Over);
        assert_eq!((3..10).describe_intersection(&(2..11)), IntersectionDescription::Within);
        assert_eq!((3..10).describe_intersection(&(3..11)), IntersectionDescription::Within);
        assert_eq!((3..10).describe_intersection(&(2..9)), IntersectionDescription::AboveOverlap);
        assert_eq!((3..10).describe_intersection(&(2..3)), IntersectionDescription::Above);
        assert_eq!((3..10).describe_intersection(&(1..2)), IntersectionDescription::Above);

        assert_eq!((3..10).describe_intersection(&(11..)), IntersectionDescription::Below);
        assert_eq!((3..10).describe_intersection(&(10..)), IntersectionDescription::Below);
        assert_eq!((3..10).describe_intersection(&(9..)), IntersectionDescription::BelowOverlap);
        assert_eq!((3..10).describe_intersection(&(3..)), IntersectionDescription::Within);
        assert_eq!((3..10).describe_intersection(&(2..)), IntersectionDescription::Within);

        assert_eq!((3..10).describe_intersection(&(..)), IntersectionDescription::Within);

        assert_eq!((3..10).describe_intersection(&(..11)), IntersectionDescription::Within);
        assert_eq!((3..10).describe_intersection(&(..10)), IntersectionDescription::Within);
        assert_eq!((3..10).describe_intersection(&(..9)), IntersectionDescription::AboveOverlap);
        assert_eq!((3..10).describe_intersection(&(..3)), IntersectionDescription::Above);
        assert_eq!((3..10).describe_intersection(&(..2)), IntersectionDescription::Above);
    }

    #[test]
    pub fn test_inclusive_range_intersect() {
        assert_eq!((3..=9).describe_intersection(&(11..11)), IntersectionDescription::Below);
        assert_eq!((3..=9).describe_intersection(&(10..11)), IntersectionDescription::Below);
        assert_eq!((3..=9).describe_intersection(&(9..11)), IntersectionDescription::BelowOverlap);
        assert_eq!((3..=9).describe_intersection(&(9..10)), IntersectionDescription::Over);
        assert_eq!((3..=9).describe_intersection(&(3..10)), IntersectionDescription::Same);
        assert_eq!((3..=9).describe_intersection(&(5..9)), IntersectionDescription::Over);
        assert_eq!((3..=9).describe_intersection(&(3..9)), IntersectionDescription::Over);
        assert_eq!((3..=9).describe_intersection(&(2..11)), IntersectionDescription::Within);
        assert_eq!((3..=9).describe_intersection(&(3..11)), IntersectionDescription::Within);
        assert_eq!((3..=9).describe_intersection(&(2..9)), IntersectionDescription::AboveOverlap);
        assert_eq!((3..=9).describe_intersection(&(2..3)), IntersectionDescription::Above);
        assert_eq!((3..=9).describe_intersection(&(1..2)), IntersectionDescription::Above);

        assert_eq!((3..=9).describe_intersection(&(11..)), IntersectionDescription::Below);
        assert_eq!((3..=9).describe_intersection(&(10..)), IntersectionDescription::Below);
        assert_eq!((3..=9).describe_intersection(&(9..)), IntersectionDescription::BelowOverlap);
        assert_eq!((3..=9).describe_intersection(&(3..)), IntersectionDescription::Within);
        assert_eq!((3..=9).describe_intersection(&(2..)), IntersectionDescription::Within);

        assert_eq!((3..=9).describe_intersection(&(..)), IntersectionDescription::Within);

        assert_eq!((3..=9).describe_intersection(&(..11)), IntersectionDescription::Within);
        assert_eq!((3..=9).describe_intersection(&(..10)), IntersectionDescription::Within);
        assert_eq!((3..=9).describe_intersection(&(..9)), IntersectionDescription::AboveOverlap);
        assert_eq!((3..=9).describe_intersection(&(..3)), IntersectionDescription::Above);
        assert_eq!((3..=9).describe_intersection(&(..2)), IntersectionDescription::Above);
    }

    #[test]
    pub fn test_range_intersect_with_inclusive_range() {
        assert_eq!((3..10).describe_intersection(&(11..=10)), IntersectionDescription::Below);
        assert_eq!((3..10).describe_intersection(&(10..=10)), IntersectionDescription::Below);
        assert_eq!((3..10).describe_intersection(&(9..=10)), IntersectionDescription::BelowOverlap);
        assert_eq!((3..10).describe_intersection(&(9..=9)), IntersectionDescription::Over);
        assert_eq!((3..10).describe_intersection(&(3..=9)), IntersectionDescription::Same);
        assert_eq!((3..10).describe_intersection(&(5..=8)), IntersectionDescription::Over);
        assert_eq!((3..10).describe_intersection(&(3..=8)), IntersectionDescription::Over);
        assert_eq!((3..10).describe_intersection(&(2..=10)), IntersectionDescription::Within);
        assert_eq!((3..10).describe_intersection(&(3..=10)), IntersectionDescription::Within);
        assert_eq!((3..10).describe_intersection(&(2..=8)), IntersectionDescription::AboveOverlap);
        assert_eq!((3..10).describe_intersection(&(2..=2)), IntersectionDescription::Above);
        assert_eq!((3..10).describe_intersection(&(1..=1)), IntersectionDescription::Above);
    }

    #[test]
    pub fn test_inclusive_range_intersect_with_inclusive_range() {
        assert_eq!((3..=9).describe_intersection(&(11..=10)), IntersectionDescription::Below);
        assert_eq!((3..=9).describe_intersection(&(10..=10)), IntersectionDescription::Below);
        assert_eq!((3..=9).describe_intersection(&(9..=10)), IntersectionDescription::BelowOverlap);
        assert_eq!((3..=9).describe_intersection(&(9..=9)), IntersectionDescription::Over);
        assert_eq!((3..=9).describe_intersection(&(3..=9)), IntersectionDescription::Same);
        assert_eq!((3..=9).describe_intersection(&(5..=8)), IntersectionDescription::Over);
        assert_eq!((3..=9).describe_intersection(&(3..=8)), IntersectionDescription::Over);
        assert_eq!((3..=9).describe_intersection(&(2..=10)), IntersectionDescription::Within);
        assert_eq!((3..=9).describe_intersection(&(3..=10)), IntersectionDescription::Within);
        assert_eq!((3..=9).describe_intersection(&(2..=8)), IntersectionDescription::AboveOverlap);
        assert_eq!((3..=9).describe_intersection(&(2..=2)), IntersectionDescription::Above);
        assert_eq!((3..=9).describe_intersection(&(1..=1)), IntersectionDescription::Above);
    }
}
