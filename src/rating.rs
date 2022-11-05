#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rating(pub u32);

impl Rating {
    pub const MAX: Self = Self(5000);
    pub const MIN: Self = Self(0);

    pub fn new(value: u32) -> Option<Self> {
        if value <= Self::MAX.0 && value >= Self::MIN.0 {
            Some(Self(value))
        } else {
            None
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn invalid_rating_value() {
        assert!(Rating::new(5001).is_none());
    }

    #[test]
    fn cmp_rating() {
        assert!(Rating::MAX > Rating::MIN);
        assert!(Rating::MAX == Rating::MAX);
    }
}
