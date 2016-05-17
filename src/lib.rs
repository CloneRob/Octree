extern crate cgmath;
extern crate collision;


pub mod octree;
pub mod octree_node;

pub trait OctreeItem {
    fn index(&self) -> cgmath::Point3<f32>;
    fn is_equal(&self, other: &Self) -> bool;
    fn in_frustum(&self, frustum: &collision::Frustum<f32>) -> bool;
}

pub trait Surround {
    fn surrounds(&self, other: &Self) -> bool;
}

pub trait MinVolume {
    fn is_min(&self, val: f32) -> bool;
}
