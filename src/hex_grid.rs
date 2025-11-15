/// A position on a hexagonal grid using axial coordinates. See https://www.redblobgames.com/grids/hexagons/.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos {
    pub r: isize,
    pub q: isize,
}
impl Pos {
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

pub fn pos_in_area(pos: &Pos, area: &Area, target: &Pos) -> bool {
    match area {
        Area::Disk(distance_range) => {
            distance_within_range(pos, target, distance_range)
        }
    }
}

pub fn distance(a: &Pos, b: &Pos) -> usize {
    (isize::abs_diff(a.q, b.q) + isize::abs_diff(a.r, b.r) + isize::abs_diff(a.s(), b.s())) / 2
}

pub fn distance_within_range(a: &Pos, b: &Pos, distance_range: &DistanceRange) -> bool {
    let distance = distance(a, b);
    distance_range.from <= distance && distance < distance_range.to
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_pos_in_area() {
        assert!(pos_in_area(
            &Pos::default(),
            &Area::default(),
            &Pos::default()
        ));
        assert!(pos_in_area(
            &Pos::new(0, 1),
            &Area::Disk(DistanceRange { from: 1, to: 2 }),
            &Pos::new(0, 0)
        ));
    }
}
