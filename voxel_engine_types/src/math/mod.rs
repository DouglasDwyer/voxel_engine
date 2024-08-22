use bytemuck::*;
pub use glam::*;
pub use crate::math::direction::*;
use serde::*;
use std::ops::*;

/// Types for distinguishing between various cardinal directions;
mod direction;

/// Describes a location and orientation in 3D space.
#[derive(Copy, Clone, Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Transform {
    /// The position of the object in the world.
    pub position: WorldVec,
    /// A quaternion which converts from the object's rotation space to world rotation space.
    pub rotation: Quat
}

impl Transform {
    /// Creates a new transform with the specified position and rotation.
    pub fn new(position: WorldVec, rotation: Quat) -> Self {
        Self { position, rotation }
    }

    /// Smoothly interpolates between the two given transforms. When `t = 0`,
    /// `a` is returned, and when `t = 1`, `b` is returned. `t` may be any finite
    /// floating point number.
    pub fn interpolate(a: &Transform, b: &Transform, t: f32) -> Self {
        Self::new(a.position.lerp(b.position, t), a.rotation.slerp(b.rotation, t))
    }

    /// Creates a matrix which converts from points in the model coordinate
    /// system to points in this transform's coordinate space.
    pub fn view_model_matrix(&self, model: &Self) -> Mat4 {
        Mat4::from_rotation_translation(self.rotation.inverse(), Vec3::ZERO)
            * Mat4::from_rotation_translation(model.rotation, model.position.displacement(self.position).into())
    }

    /// Returns the front-facing direction of this transform in the parent
    /// coordinate space.
    pub fn look_direction(&self) -> Vec3A {
        self.rotation * Vec3A::Z
    }
}

/// Represents a position in world space.
#[repr(C)]
#[derive(Copy, Clone, Default, Serialize, Deserialize, Hash, PartialEq, Eq, Pod, Zeroable)]
pub struct WorldVec {
    /// The x-coordinate.
    pub x: WorldCoord,
    /// The y-coordinate.
    pub y: WorldCoord,
    /// The z-coordinate.
    pub z: WorldCoord,
}

impl WorldVec {
    /// The world vector that corresponds to the origin.
    pub const ZERO: Self = Self { x: WorldCoord::ZERO, y: WorldCoord::ZERO, z: WorldCoord::ZERO };

    /// Retrieves a representation of this value as an integer vector, in world units.
    pub fn bits(self) -> IVec3 {
        cast(self)
    }

    /// Obtains the displacement between this world position and another one
    /// in floating-point voxel units.
    pub fn displacement(self, other: Self) -> Vec3A {
        let disp = cast::<_, IVec3>(self) - cast::<_, IVec3>(other);
        
        let vox = (disp >> WorldCoord::LOG2_UNITS_PER_VOXEL).as_vec3a();
        let frac = (disp & (WorldCoord::UNITS_PER_VOXEL as i32 - 1)).as_vec3a() / 256.0;

        vox + frac
    }

    /// Converts an integer vector, in world units, to a world vector.
    pub fn from_bits(v: IVec3) -> Self {
        cast(v)
    }

    /// Creates a new world vector positioned on the minimum corner
    /// of the given voxel position.
    pub fn from_voxel(voxel: IVec3) -> Self {
        cast(voxel << WorldCoord::LOG2_UNITS_PER_VOXEL)
    }

    /// Linearly interpolates between two world vectors,
    /// based upon the provided parameter.
    pub fn lerp(self, other: Self, t: f32) -> Self {
        self + Self::from(t * other.displacement(self))
    }

    /// Determines the voxel in which this world position resides.
    pub fn voxel(self) -> IVec3 {
        cast::<_, IVec3>(self) >> WorldCoord::LOG2_UNITS_PER_VOXEL
    }
}

impl Add for WorldVec {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        cast(cast::<_, IVec3>(self) + cast::<_, IVec3>(rhs))
    }
}

impl Sub for WorldVec {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        cast(cast::<_, IVec3>(self) - cast::<_, IVec3>(rhs))
    }
}

impl AddAssign for WorldVec {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for WorldVec {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl From<Vec3A> for WorldVec {
    fn from(x: Vec3A) -> Self {
        let x_floor = x.floor();
        let x_fract = x - x_floor;
        let vox = x_floor.as_ivec3() << WorldCoord::LOG2_UNITS_PER_VOXEL;
        let rem = ((WorldCoord::UNITS_PER_VOXEL as f32) * x_fract).as_ivec3();
        cast(vox + rem)
    }
}

impl std::fmt::Debug for WorldVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.displacement(WorldVec::default())))
    }
}

/// A single coordinate within the world, representing a position or displacement.
#[repr(transparent)]
#[derive(Copy, Clone, Default, Serialize, Deserialize, Hash, PartialEq, Eq, Pod, Zeroable)]
pub struct WorldCoord(i32);

impl WorldCoord {
    /// The world coordinate that corresponds to the origin.
    pub const ZERO: Self = Self(0);

    /// The granularity of a world coordinate, or the number of units per voxel.
    pub const UNITS_PER_VOXEL: u32 = 1 << Self::LOG2_UNITS_PER_VOXEL;
    
    /// The base-2 logarithm of the number of units per voxel.
    pub const LOG2_UNITS_PER_VOXEL: u32 = 8;

    /// Retrieves a representation of this value as an integer in world units.
    pub fn bits(self) -> i32 {
        self.0
    }

    /// Obtains the displacement between this world position and another one
    /// in floating-point voxel units.
    pub fn displacement(self, other: Self) -> f32 {
        let disp = self.0 - other.0;
        
        let vox = (disp >> Self::LOG2_UNITS_PER_VOXEL) as f32;
        let frac = (disp & (Self::UNITS_PER_VOXEL as i32 - 1)) as f32 / Self::UNITS_PER_VOXEL as f32;

        vox + frac
    }

    /// Converts an integer coordinate, in world units, to a world coordinate.
    pub fn from_bits(v: i32) -> Self {
        Self(v)
    }

    /// Creates a new world coordinate positioned on the minimum corner
    /// of the given voxel position.
    pub fn from_voxel(voxel: i32) -> Self {
        Self(voxel << Self::LOG2_UNITS_PER_VOXEL)
    }

    /// Linearly interpolates between two world coordinates,
    /// based upon the provided parameter.
    pub fn lerp(self, other: Self, t: f32) -> Self {
        self + Self::from(t * other.displacement(self))
    }

    /// Determines the voxel in which this world position resides.
    pub fn voxel(self) -> i32 {
        self.0 >> Self::LOG2_UNITS_PER_VOXEL
    }
}

impl Add for WorldCoord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for WorldCoord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl AddAssign for WorldCoord {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for WorldCoord {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl From<f32> for WorldCoord {
    fn from(x: f32) -> Self {
        let x_floor = x.floor();
        let x_fract = x - x_floor;
        let vox = (x_floor as i32) << Self::LOG2_UNITS_PER_VOXEL;
        let rem = ((Self::UNITS_PER_VOXEL as f32) * x_fract) as i32;
        Self(vox + rem)
    }
}

impl std::fmt::Debug for WorldCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}