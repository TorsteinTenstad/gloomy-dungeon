/// A position on a hexagonal grid using axial coordinates. See https://www.redblobgames.com/grids/hexagons/.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PosAxial {
    pub r: isize,
    pub q: isize,
}

#[derive(Default, Debug, Hash, PartialEq, Eq)]
pub struct PosOddQHex {
    pub q: isize,
    pub r: isize,
}

impl PosOddQHex {
    pub fn new(r: isize, q: isize) -> Self {
        Self { r, q }
    }
    pub fn to_axial(&self) -> PosAxial {
        let parity = self.q & 1;
        let q = self.q;
        let r = self.r - (self.q - parity) / 2;
        return PosAxial { r, q };
    }
}

impl PosAxial {
    pub fn new(r: isize, q: isize) -> Self {
        Self { r, q }
    }
    pub fn s(&self) -> isize {
        -self.q - self.r
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PosDelta {
    pub r: isize,
    pub q: isize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistanceRange {
    pub from: usize, // inclusive
    pub to: usize,   // not inclusive
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Area {
    Disk(DistanceRange),
}

impl Default for Area {
    fn default() -> Self {
        Area::Disk(DistanceRange { from: 0, to: 1 })
    }
}

pub fn pos_in_area(pos: &PosAxial, area: &Area, target: &PosAxial) -> bool {
    match area {
        Area::Disk(distance_range) => distance_within_range(pos, target, distance_range),
    }
}

pub fn distance(a: &PosAxial, b: &PosAxial) -> usize {
    (isize::abs_diff(a.q, b.q) + isize::abs_diff(a.r, b.r) + isize::abs_diff(a.s(), b.s())) / 2
}

pub fn distance_within_range(a: &PosAxial, b: &PosAxial, distance_range: &DistanceRange) -> bool {
    let distance = distance(a, b);
    distance_range.from <= distance && distance < distance_range.to
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_pos_in_area() {
        assert!(pos_in_area(
            &PosAxial::default(),
            &Area::default(),
            &PosAxial::default()
        ));
        assert!(pos_in_area(
            &PosAxial::new(0, 1),
            &Area::Disk(DistanceRange { from: 1, to: 2 }),
            &PosAxial::new(0, 0)
        ));
    }
}
