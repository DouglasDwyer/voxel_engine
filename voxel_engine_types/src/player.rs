use crate::math::*;
use serde::*;
use wings::*;

/// Allows for setting various properties of the current player.
/// This trait mostly contains debug functionality for testing;
/// its API will change in the future.
/// Only available on the [`Client`](crate::Client).
#[system_trait(host)]
pub trait Player: 'static {
    /// Deletes some voxels at the given world position.
    /// The edit occurs at the end of the current frame.
    fn delete_voxels_at(&self, position: IVec3);

    /// Sets the entity and target that the player is currently dragging.
    fn drag_physics_object(&self, operation: Option<DragEntity>);

    /// Gets the player's current transform.
    fn get_transform(&self) -> Transform;
    
    /// Places some voxels at the given world position. The voxel shape
    /// changes based upon index.
    /// The edit occurs at the end of the current frame.
    fn place_voxels_at(&self, position: IVec3, shape_index: u32);

    /// Sets the player's current transform.
    fn set_transform(&mut self, transform: Transform);

    /// Spawns a physics object for testing at the end of the current frame.
    fn spawn_physics_object(&self, position: WorldVec, kind_index: u32);
}

/// Describes a drag operation on an entity.
#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DragEntity {
    /// The body-local point where the object was grabbed.
    pub contact_point: Vec3A,
    /// The ID of the object.
    pub id: u64,
    /// The position to which the object should be dragged.
    pub target_position: WorldVec
}