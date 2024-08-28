use crate::math::*;
use serde::*;
use wings::*;

/// Describes a ray that should be cast into the world.
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Ray {
    /// The ray's direction of travel.
    pub direction: Vec3A,
    /// The starting point of the ray in world space.
    pub position: WorldVec,
    /// The maximum distance that the ray may travel.
    pub max_distance: f32,
}

/// Indicates that a ray intersected with voxel geometry.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct RaycastHit {
    /// The distance from the starting point that the ray traveled.
    pub distance: f32,
    /// The normal of the voxel face that was hit.
    pub face: Direction,
    /// The voxel object that was hit.
    pub object: RaycastObject,
    /// The local coordinate of the voxel that was hit. If the object was
    /// [`RaycastObject::World`], then this coordinate corresponds to a world-space position.
    /// If the object was a [`RaycastObject::Entity`], then this coordinate corresponds to
    /// that voxel index on the entity.
    pub voxel: IVec3,
}

/// An object that was hit during a ray query.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RaycastObject {
    /// An entity was hit.
    Entity {
        /// The opaque ID of the entity.
        id: u64,
    },
    /// The main voxel grid was hit.
    World {},
}

/// Determines the intersection between rays in the world and voxel objects.
#[system_trait(host)]
pub trait Raycaster: 'static {
    /// Casts a ray that can hit both entities and the main voxel grid.
    fn cast(&self, ray: &Ray) -> Option<RaycastHit>;

    /// Casts a ray that can hit entities but ignores the main voxel grid.
    fn cast_entities(&self, ray: &Ray) -> Option<RaycastHit>;

    /// Casts a ray that can hit the main voxel grid but ignores entities.
    fn cast_world(&self, ray: &Ray) -> Option<RaycastHit>;
}
