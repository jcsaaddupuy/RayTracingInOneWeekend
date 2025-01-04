pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        };
        if x > self.max {
            return self.max;
        };
        x
    }
}

const EMPTY: Interval = Interval::new(f64::INFINITY, -f64::INFINITY);
const UNIVERSE: Interval = Interval::new(-f64::INFINITY, f64::INFINITY);

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_const_intervals() {
        assert_eq!(EMPTY.contains(1.0), false);
        assert_eq!(EMPTY.contains(f64::INFINITY), false);
        assert_eq!(EMPTY.surrounds(f64::INFINITY), false);

        assert_eq!(UNIVERSE.contains(1.0), true);
        assert_eq!(UNIVERSE.surrounds(1.0), true);
    }

    #[test]
    fn test_intervals() {
        let interval = Interval::new(10.0, 20.0);
        assert_eq!(interval.contains(1.0), false);

        assert_eq!(interval.contains(11.0), true);
        assert_eq!(interval.surrounds(11.0), true);

        assert_eq!(interval.contains(9.0), false);
        assert_eq!(interval.surrounds(9.0), false);

        assert_eq!(interval.contains(10.0), true);
        assert_eq!(interval.surrounds(10.0), false);

        assert_eq!(interval.contains(21.0), false);
        assert_eq!(interval.surrounds(21.0), false);

        assert_eq!(interval.contains(20.0), true);
        assert_eq!(interval.surrounds(20.0), false);
    }

    #[test]
    fn test_clamp() {
        let interval = Interval::new(10.0, 20.0);
        assert_eq!(interval.clamp(1.0), 10.0);
        assert_eq!(interval.clamp(21.0), 20.0);
    }
}
