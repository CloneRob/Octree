// Standard Library
use std::fmt::*;

// External Library
use cgmath::Point3;
use collision::{Aabb3, Frustum};


use OctreeItem;
use octree_node::*;

pub struct Octree<I>
    where I: OctreeItem + Clone + Debug
{
    root: OctreeNode<I>,
    min_side: f32,
}
impl<I> Octree<I> where I: OctreeItem + Clone + Debug
{
    pub fn new(corner: f32, min_side: f32) -> Octree<I> {
        Octree {
            root: OctreeNode::leaf(Aabb3::new(Point3::new(-corner, -corner, -corner),
                                              Point3::new(corner, corner, corner))),
            min_side: min_side,
        }
    }

    pub fn from(vol: &Aabb3<f32>, min_side: f32) -> Octree<I> {
        Octree { root: OctreeNode::leaf(vol.clone()),
                 min_side: min_side
             }
    }
    pub fn insert(&mut self, item: I) -> bool {
        self.root.insert_safe(item, self.min_side)
    }
    pub fn print_volume_by_level(&self) {
        self.root.print_volume_by_level(0)
    }
    pub fn members(&self) -> usize {
        self.root.members()
    }

    pub fn frustum_culling(&self, frustum: &Frustum<f32>) -> Vec<&I> {
        self.root.frustum_culling(frustum)
    }

    pub fn culling_w_capacity(&self, frustum: &Frustum<f32>, capacity: usize) -> Vec<&I> {
        self.root.frustum_culling_cap(frustum, capacity)
    }
}
