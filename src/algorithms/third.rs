use super::structs::compressed::index::CompressedIndex;
use super::structs::tree::PersistentTree;
use super::structs::{point::Point, rect::Rect};
use super::traits::lab::LabSolution;

#[derive(Debug)]
pub struct AlgorithmOnPersistenTree;

impl LabSolution for AlgorithmOnPersistenTree {
    fn count_rect_for_point(points: &Vec<Point>, rects: &Vec<Rect>) {
        if rects.is_empty() {
            print!("Here is no rectangles");
        } else {
            let (mut c_idx, mut c_idy): (CompressedIndex, CompressedIndex) =
                CompressedIndex::from_rects(&rects);
            c_idx.compress();
            c_idy.compress();

            let (seg_tree, c_idr) = PersistentTree::build_with(&c_idx, &c_idy, rects);

            for p in points {
                print!(
                    "{} ",
                    PersistentTree::query(&seg_tree, p, &c_idr, &c_idx, &c_idy)
                );
            }
        }
        println!();
    }
}
