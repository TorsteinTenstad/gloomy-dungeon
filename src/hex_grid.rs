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
pub struct Disk {
    pub inner_radius: usize,
    pub outer_radius: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Area {
    Disk(Disk),
}

impl Default for Area {
    fn default() -> Self {
        Area::Disk(Disk {
            inner_radius: 0,
            outer_radius: 1,
        })
    }
}

pub fn pos_in_area(pos: &Pos, area: &Area, target: &Pos) -> bool {
    match area {
        Area::Disk(disk) => {
            let distance = distance(pos, target);
            disk.inner_radius <= distance && distance < disk.outer_radius
        }
    }
}

pub fn distance(a: &Pos, b: &Pos) -> usize {
    (isize::abs_diff(a.q, b.q) + isize::abs_diff(a.r, b.r) + isize::abs_diff(a.s(), b.s())) / 2
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
            &Area::Disk(Disk {
                inner_radius: 1,
                outer_radius: 2
            }),
            &Pos::new(0, 0)
        ));
    }
}
