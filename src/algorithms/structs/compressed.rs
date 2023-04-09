use super::{point::Point, rect::Rect};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct CompressedIndex {
    c_index: Vec<i32>,
}

impl CompressedIndex {
    fn new(v: Vec<i32>) -> Self {
        Self { c_index: v }
    }
    fn get_index_of(&self, val: &i32) -> usize {
        self.c_index.iter().find_position(|i| i > &val).unwrap().0 - 1
    }
    fn len(&self) -> usize {
        self.c_index.len()
    }
    fn capacity(&self) -> usize {
        self.c_index.capacity()
    }
    pub fn from_rects(r: &Vec<Rect>) -> (Self, Self) {
        let mut c_x = Vec::new();
        let mut c_y = Vec::new();
        r.iter().for_each(|r| {
            c_x.push(r.lower_l.x);
            c_y.push(r.lower_l.y);
            c_x.push(r.upper_r.x);
            c_y.push(r.upper_r.y);
        });
        (Self::new(c_x), Self::new(c_y))
    }

    pub fn compress(&mut self) {
        self.c_index.sort();
        self.c_index = self.c_index.clone().into_iter().unique().collect();
    }
}

#[derive(Debug)]
pub struct CompressedMap {
    c_map: Vec<Vec<u32>>,
    c_idx: CompressedIndex,
    c_idy: CompressedIndex,
}

impl CompressedMap {
    pub fn fill_with(&mut self, rects: &Vec<Rect>) {
        for r in rects {
            let ll = &r.lower_l;
            let ur = &r.upper_r;
            let ll = Point::new(
                self.c_idx.get_index_of(&ll.x) as i32,
                self.c_idy.get_index_of(&ll.y) as i32,
            );
            let ur = Point::new(
                self.c_idx.get_index_of(&ur.x) as i32,
                self.c_idy.get_index_of(&ur.y) as i32,
            );
            for i in ll.x..=ur.x {
                for j in ll.y..=ur.y {
                    self.c_map[i as usize][j as usize] += 1;
                }
            }
        }
    }

    pub fn get_value(&self, p: &Point) -> u32 {
        let id_x = self.c_idx.get_index_of(&p.x);
        let id_y = self.c_idx.get_index_of(&p.y);
        self.c_map[id_x][id_y]
    }
}

impl From<(&CompressedIndex, &CompressedIndex)> for CompressedMap {
    fn from(value: (&CompressedIndex, &CompressedIndex)) -> Self {
        let (c_x, c_y) = value;
        let mut c_map: Vec<Vec<u32>> = Vec::with_capacity(c_x.capacity());
        for _i in 0..c_x.len() {
            c_map.push(Vec::with_capacity(c_y.capacity()));
        }
        Self {
            c_map: (c_map),
            c_idx: c_x.clone(),
            c_idy: c_y.clone(),
        }
    }
}