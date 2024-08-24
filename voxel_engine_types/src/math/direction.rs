use crate::math::*;
use std::mem::*;

/// Represents a single octant within an octree.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub enum Octant {
    /// The left-down-back octant.
    #[default]
    Z0Y0X0 = 0x00,
    /// The right-down-back octant.
    Z0Y0X1 = 0x01,
    /// The left-up-back octant.
    Z0Y1X0 = 0x02,
    /// The right-up-back octant.
    Z0Y1X1 = 0x03,
    /// The left-down-front octant.
    Z1Y0X0 = 0x04,
    /// The right-down-front octant.
    Z1Y0X1 = 0x05,
    /// The left-up-front octant.
    Z1Y1X0 = 0x06,
    /// The right-up-front octant.
    Z1Y1X1 = 0x07
}

impl Octant {
    /// Converts the raw bits into a voxel octant.
    /// 
    /// # Safety
    /// 
    /// For this conversion to be defined, the raw bits must be on the range `[0, 7]`.
    #[inline(always)]
    pub const unsafe fn from_raw(bits: u8) -> Self {
        transmute(bits)
    }

    /// Converts the lowest three raw bits into a voxel octant, ignoring any upper bits.
    #[inline(always)]
    pub const fn from_raw_truncate(bits: u8) -> Self {
        unsafe {
            Self::from_raw(bits & 0b111)
        }
    }

    /// Gets the offset of the unit octant corresponding to this value.
    pub const fn as_uvec3(self) -> UVec3 {
        uvec3(self as u32 & 0b1, ((self as u8) >> 1) as u32 & 0b1, ((self as u8) >> 2) as u32)
    }

    /// Inverts the position of this octant along the specified axis.
    pub fn flip(self, axis: Axis) -> Octant {
        unsafe {
            Self::from_raw((self as u8) ^ (1 << (axis as u8)))
        }
    }

    /// An array which lists all eight octants in lexical order.
    #[inline(always)]
    pub fn lexical_order() -> &'static [Octant; 8] {
        &[Octant::Z0Y0X0, Octant::Z0Y0X1, Octant::Z0Y1X0, Octant::Z0Y1X1, Octant::Z1Y0X0, Octant::Z1Y0X1, Octant::Z1Y1X0, Octant::Z1Y1X1]
    }
}

impl From<u8> for Octant {
    #[inline(always)]
    fn from(bits: u8) -> Self {
        assert!(bits < 8);
        unsafe { Self::from_raw(bits) }
    }
}

impl From<Octant> for UVec3 {
    #[inline(always)]
    fn from(x: Octant) -> Self {
        (UVec3::splat(x as u32) >> uvec3(0, 1, 2)) & UVec3::ONE
    }
}


/// Represents a selection of multiple octants within an octree.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Pod, Zeroable)]
#[repr(transparent)]
pub struct OctantFlags(u8);

impl OctantFlags {
    /// No possible octants.
    pub const NONE: Self = Self(0);
    /// The left-down-back octant.
    pub const Z0Y0X0: Self = Self(1 << Octant::Z0Y0X0 as u8);
    /// The right-down-back octant.
    pub const Z0Y0X1: Self = Self(1 << Octant::Z0Y0X1 as u8);
    /// The left-up-back octant.
    pub const Z0Y1X0: Self = Self(1 << Octant::Z0Y1X0 as u8);
    /// The right-up-back octant.
    pub const Z0Y1X1: Self = Self(1 << Octant::Z0Y1X1 as u8);
    /// The left-down-front octant.
    pub const Z1Y0X0: Self = Self(1 << Octant::Z1Y0X0 as u8);
    /// The right-down-front octant.
    pub const Z1Y0X1: Self = Self(1 << Octant::Z1Y0X1 as u8);
    /// The left-up-front octant.
    pub const Z1Y1X0: Self = Self(1 << Octant::Z1Y1X0 as u8);
    /// The right-up-front octant.
    pub const Z1Y1X1: Self = Self(1 << Octant::Z1Y1X1 as u8);
    /// All possible octants.
    pub const ALL: Self = Self(0xff);

    /// Creates a set of flags which contains only the given octant.
    pub const fn from_octant(x: Octant) -> Self {
        Self::from_bits(1 << (x as u8))
    }

    /// Creates a new set of flags from the given underlying bit values.
    pub const fn from_bits(value: u8) -> Self {
        Self(value)
    }

    /// Gets the underlying bit representation of these flags.
    pub const fn bits(self) -> u8 {
        self.0
    }

    /// Whether all of the flags in `other` are also in `self`.
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl BitOr for OctantFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for OctantFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitAnd for OctantFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for OctantFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl Not for OctantFlags {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl From<Octant> for OctantFlags {
    fn from(x: Octant) -> Self {
        Self::from_octant(x)
    }
}

impl IntoIterator for OctantFlags {
    type Item = Octant;

    type IntoIter = std::iter::FilterMap<std::iter::Zip<Range<u8>, std::iter::Repeat<OctantFlags>>, fn((u8, OctantFlags)) -> Option<Octant>>;

    fn into_iter(self) -> Self::IntoIter {
        (0u8..8).zip(std::iter::repeat(self)).filter_map(move |(x, y)| unsafe {
            let octant = Octant::from_raw(x);
            y.contains(octant.into()).then_some(octant)
        })
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct DirectionFlags(u8);

impl DirectionFlags {
    /// No directions.
    pub const NONE: Self = Self(0);
    /// The -x direction.
    pub const LEFT: Self = Self(1 << Direction::LEFT as u8);
    /// The +x direction.
    pub const RIGHT: Self = Self(1 << Direction::RIGHT as u8);
    /// The -y direction.
    pub const DOWN: Self = Self(1 << Direction::DOWN as u8);
    /// The +y direction.
    pub const UP: Self = Self(1 << Direction::UP as u8);
    /// The -z direction.
    pub const BACK: Self = Self(1 << Direction::BACK as u8);
    /// The +z direction.
    pub const FRONT: Self = Self(1 << Direction::FRONT as u8);
    /// All possible directions.
    pub const ALL: Self = Self(DirectionFlags::LEFT.0 | DirectionFlags::RIGHT.0 | DirectionFlags::DOWN.0 | DirectionFlags::UP.0 | DirectionFlags::BACK.0 | DirectionFlags::FRONT.0);

    /// Creates a new set of direction flags that points toward an octant.
    pub const fn from_octant(a: Octant) -> Self {
        unsafe {
            transmute(match a {
                Octant::Z0Y0X0 => DirectionFlags::LEFT.0 | DirectionFlags::DOWN.0 | DirectionFlags::BACK.0,
                Octant::Z0Y0X1 => DirectionFlags::RIGHT.0 | DirectionFlags::DOWN.0 | DirectionFlags::BACK.0,
                Octant::Z0Y1X0 => DirectionFlags::LEFT.0 | DirectionFlags::UP.0 | DirectionFlags::BACK.0,
                Octant::Z0Y1X1 => DirectionFlags::RIGHT.0 | DirectionFlags::UP.0 | DirectionFlags::BACK.0,
                Octant::Z1Y0X0 => DirectionFlags::LEFT.0 | DirectionFlags::DOWN.0 | DirectionFlags::FRONT.0,
                Octant::Z1Y0X1 => DirectionFlags::RIGHT.0 | DirectionFlags::DOWN.0 | DirectionFlags::FRONT.0,
                Octant::Z1Y1X0 => DirectionFlags::LEFT.0 | DirectionFlags::UP.0 | DirectionFlags::FRONT.0,
                Octant::Z1Y1X1 => DirectionFlags::RIGHT.0 | DirectionFlags::UP.0 | DirectionFlags::FRONT.0,
            })
        }
    }

    /// Gets the underlying bit representation of these flags.
    pub const fn bits(self) -> u8 {
        self.0
    }

    /// Whether all of the flags in `other` are also in `self`.
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Constructs a new set of direction flags from the underlying bits,
    /// ignoring any extra bits in the mask.
    pub const fn from_bits_truncate(bits: u8) -> Self {
        Self(bits & Self::ALL.0)
    }

    /// Creates a set of direction flags from two masks. The lowest three bits in the first
    /// mask represent whether the left, down, and back flags should be set. The lowest three
    /// bits in the second mask represent whether the up, right, and front flags should be set.
    pub fn from_negative_positive_masks(negatives: u8, positives: u8) -> DirectionFlags {
        let x = negatives | (positives << 3);
        DirectionFlags::from_bits_truncate((x & 0x21)
            | ((x & 0x02) << 1)
            | ((x & 0x04) << 2)
            | ((x & 0x08) >> 2)
            | ((x & 0x10) >> 1))
    }
}

impl BitOr for DirectionFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for DirectionFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitAnd for DirectionFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for DirectionFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl Not for DirectionFlags {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::from_bits_truncate(!self.0)
    }
}

impl IntoIterator for DirectionFlags {
    type Item = Direction;

    type IntoIter = std::iter::FilterMap<std::iter::Zip<std::array::IntoIter<Direction, 6>, std::iter::Repeat<DirectionFlags>>, fn((Direction, DirectionFlags)) -> Option<Direction>>;

    fn into_iter(self) -> Self::IntoIter {
        [Direction::LEFT, Direction::RIGHT, Direction::DOWN, Direction::UP, Direction::BACK, Direction::FRONT].into_iter().zip(std::iter::repeat(self)).filter_map(move |(x, y)| y.contains(DirectionFlags(1 << (x as u8))).then_some(x))
    }
}

impl From<DirectionFlags> for IVec3 {
    fn from(x: DirectionFlags) -> Self {
        x.into_iter().fold(IVec3::ZERO, |acc, x| acc + IVec3::from(x))
    }
}

/// Represents a direction in 3D Cartesian space.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Direction {
    /// The -x direction.
    LEFT = 0,
    /// The +x direction.
    RIGHT = 1,
    /// The -y direction.
    DOWN = 2,
    /// The +y direction.
    UP = 3,
    /// The -z direction.
    BACK = 4,
    /// The +z direction.
    FRONT = 5
}

impl Direction {
    /// Creates a new direction from a raw byte.
    ///
    /// # Safety
    /// 
    ///  For this function to be sound, bits must be on the range [0, 5].
    pub const unsafe fn from_raw(bits: u8) -> Self {
        transmute(bits)
    }

    /// Returns a unit-length offset in the direction
    /// described by this value.
    pub const fn offset(self) -> IVec3 {
        match self {
            Direction::LEFT => ivec3(-1, 0, 0),
            Direction::RIGHT => ivec3(1, 0, 0),
            Direction::DOWN => ivec3(0, -1, 0),
            Direction::UP => ivec3(0, 1, 0),
            Direction::BACK => ivec3(0, 0, -1),
            Direction::FRONT => ivec3(0, 0, 1)
        }
    }

    /// Returns the opposite of this direction.
    pub const fn reverse(self) -> Self {
        unsafe {
            Self::from_raw((self as u8) ^ 1)
        }
    }

    /// Returns whether this direction points along a positive axis.
    pub fn positive(self) -> bool {
        self as u8 % 2 == 1
    }
}

impl From<Direction> for IVec3 {
    fn from(x: Direction) -> Self {
        x.offset()
    }
}

impl From<Direction> for DirectionFlags {
    fn from(x: Direction) -> Self {
        DirectionFlags(1 << (x as u8))
    }
}

impl From<Octant> for DirectionFlags {
    fn from(a: Octant) -> Self {
        Self::from_octant(a)
    }
}

/// Identifies a cardinal axis in 3D Cartesian space.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Axis {
    /// The x-axis.
    X = 0,
    /// The y-axis.
    Y = 1,
    /// The z-axis.
    Z = 2
}

impl Axis {
    /// Gets the underlying bit representation of this axis.
    pub const fn as_u8(self) -> u8 {
        unsafe {
            transmute(self)
        }
    }

    /// Creates a new axis from raw bits.
    /// 
    /// # Safety
    /// 
    /// For this conversion to be sound, the input must be on the range `[0, 2]`.
    pub const unsafe fn from_raw(bits: u8) -> Self {
        transmute(bits)
    }

    pub const fn from_direction(value: Direction) -> Self {
        unsafe {
            transmute((value as u8) >> 1)
        }
    }

    /// Obtains the direction that points toward the negative along this axis.
    pub const fn as_direction_negative(self) -> Direction {
        unsafe {
            Direction::from_raw((self as u8) << 1)
        }
    }

    /// Obtains the direction that points toward the positive along this axis.
    pub const fn as_direction_positive(self) -> Direction {
        unsafe {
            Direction::from_raw(((self as u8) << 1) | 1)
        }
    }
}

impl From<Direction> for Axis {
    fn from(value: Direction) -> Self {
        Self::from_direction(value)
    }
}

impl From<Axis> for Vec3A {
    fn from(value: Axis) -> Self {
        let mut result = Vec3A::ZERO;
        result[value.as_u8() as usize] = 1.0;
        result
    }
}

impl Index<Axis> for IVec3 {
    type Output = i32;

    fn index(&self, index: Axis) -> &Self::Output {
        &self[index.as_u8() as usize]
    }
}

impl IndexMut<Axis> for IVec3 {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        &mut self[index.as_u8() as usize]
    }
}

impl Index<Axis> for UVec3 {
    type Output = u32;

    fn index(&self, index: Axis) -> &Self::Output {
        &self[index.as_u8() as usize]
    }
}

impl IndexMut<Axis> for UVec3 {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        &mut self[index.as_u8() as usize]
    }
}

impl Index<Axis> for Vec3 {
    type Output = f32;

    fn index(&self, index: Axis) -> &Self::Output {
        &self[index.as_u8() as usize]
    }
}

impl IndexMut<Axis> for Vec3 {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        &mut self[index.as_u8() as usize]
    }
}

impl Index<Axis> for Vec3A {
    type Output = f32;

    fn index(&self, index: Axis) -> &Self::Output {
        &self[index.as_u8() as usize]
    }
}

impl IndexMut<Axis> for Vec3A {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        &mut self[index.as_u8() as usize]
    }
}

/// Provides a way to map between Cartesian cardinal directions and
/// another type.
#[derive(Copy, Clone, Debug, Default)]
pub struct DirectionMap<T>([T; 6]);

impl<T> DirectionMap<T> {
    /// Provides a reference to an array of `T`, in standard order.
    pub const fn as_array(&self) -> &[T; 6] {
        &self.0
    }

    /// Provides a mutable reference to an array of `T`, in standard order.
    pub fn as_array_mut(&mut self) -> &mut [T; 6] {
        &mut self.0
    }

    /// Creates a new mapping that contains the specified item for every direction.
    pub fn splat(item: T) -> Self where T: Clone {
        Self([item.clone(), item.clone(), item.clone(), item.clone(), item.clone(), item])
    }

    /// Converts an array of `T`, in standard order, into a direction map.
    pub const fn from_array(directions: [T; 6]) -> Self {
        Self(directions)
    }

    /// Gets an immutable reference to the item associated with the specified direction.
    pub fn get(&self, direction: Direction) -> &T {
        unsafe {
            self.0.get_unchecked(direction as usize)
        }
    }

    /// Gets a mutable reference to the item associated with the specified direction.
    pub fn get_mut(&mut self, direction: Direction) -> &mut T {
        unsafe {
            self.0.get_unchecked_mut(direction as usize)
        }
    }

    /// Obtains an iterator that loops over all of the items,
    /// along with their directions, in this direction map.
    pub fn iter(&self) -> impl Iterator<Item = (Direction, &T)> {
        self.into_iter()
    }

    /// Transforms the items in the map into a new map.
    pub fn map<U, F: FnMut(Direction, T) -> U>(self, mut f: F) -> DirectionMap<U> {
        let [i0, i1, i2, i3, i4, i5] = self.0;
        DirectionMap([
            f(Direction::LEFT, i0),
            f(Direction::RIGHT, i1),
            f(Direction::DOWN, i2),
            f(Direction::UP, i3),
            f(Direction::BACK, i4),
            f(Direction::FRONT, i5),
        ])
    }

    /// Transforms the items in the map, by reference, into a new map.
    pub fn map_ref<'a, 'b: 'a, U: 'b, F: FnMut(Direction, &'a T) -> U>(&'a self, mut f: F) -> DirectionMap<U> {
        unsafe {
            DirectionMap([
                f(Direction::LEFT, self.0.get_unchecked(0)),
                f(Direction::RIGHT, self.0.get_unchecked(1)),
                f(Direction::DOWN, self.0.get_unchecked(2)),
                f(Direction::UP, self.0.get_unchecked(3)),
                f(Direction::BACK, self.0.get_unchecked(4)),
                f(Direction::FRONT, self.0.get_unchecked(5)),
            ])
        }
    }
}

impl<T: Clone> DirectionMap<&T> {
    /// Clones the elements in the map.
    pub fn cloned(&self) -> DirectionMap<T> {
        self.map(|_, x| x.clone())
    }
}

impl<'a, T> IntoIterator for &'a DirectionMap<T> {
    type Item = (Direction, &'a T);

    type IntoIter = std::iter::Map<std::iter::Enumerate<std::slice::Iter<'a, T>>, fn((usize, &'a T)) -> (Direction, &'a T)>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe { self.0.iter().enumerate().map(|(d, reader)| (Direction::from_raw(d as u8), reader)) }
    }
}

impl<T> Index<Direction> for DirectionMap<T> {
    type Output = T;

    fn index(&self, direction: Direction) -> &Self::Output {
        self.get(direction)
    }
}

impl<T> IndexMut<Direction> for DirectionMap<T> {
    fn index_mut(&mut self, direction: Direction) -> &mut Self::Output {
        self.get_mut(direction)
    }
}

/// Converts from a direction to the four suboctants of an octree which lie on that side of the octree.
pub const DIRECTION_OCTANT_MAP: DirectionMap<[Octant; 4]> = DirectionMap::from_array([
    [Octant::Z0Y0X0, Octant::Z0Y1X0, Octant::Z1Y0X0, Octant::Z1Y1X0],
    [Octant::Z0Y0X1, Octant::Z0Y1X1, Octant::Z1Y0X1, Octant::Z1Y1X1],
    [Octant::Z0Y0X0, Octant::Z0Y0X1, Octant::Z1Y0X0, Octant::Z1Y0X1],
    [Octant::Z0Y1X0, Octant::Z0Y1X1, Octant::Z1Y1X0, Octant::Z1Y1X1],
    [Octant::Z0Y0X0, Octant::Z0Y0X1, Octant::Z0Y1X0, Octant::Z0Y1X1],
    [Octant::Z1Y0X0, Octant::Z1Y0X1, Octant::Z1Y1X0, Octant::Z1Y1X1]
]);


/// Provides a way to map between Cartesian octants and another type.
#[derive(Copy, Clone, Debug, Default)]
pub struct OctantMap<T>([T; 8]);

impl<T> OctantMap<T> {
    /// Provides a reference to an array of `T`, in standard order.
    pub const fn as_array(&self) -> &[T; 8] {
        &self.0
    }

    /// Provides a mutable reference to an array of `T`, in standard order.
    pub fn as_array_mut(&mut self) -> &mut [T; 8] {
        &mut self.0
    }

    /// Creates a new mapping that contains the specified item for every octant.
    pub fn splat(item: T) -> Self where T: Clone {
        Self([item.clone(), item.clone(), item.clone(), item.clone(), item.clone(), item.clone(), item.clone(), item])
    }

    /// Converts an array of `T`, in standard order, into an octant map.
    pub const fn from_array(octants: [T; 8]) -> Self {
        Self(octants)
    }

    /// Gets an immutable reference to the item associated with the specified octant.
    pub fn get(&self, octant: Octant) -> &T {
        unsafe {
            self.0.get_unchecked(octant as usize)
        }
    }

    /// Gets a mutable reference to the item associated with the specified octant.
    pub fn get_mut(&mut self, octant: Octant) -> &mut T {
        unsafe {
            self.0.get_unchecked_mut(octant as usize)
        }
    }

    /// Obtains an iterator that loops over all of the items,
    /// along with their octants, in this octant map.
    pub fn iter(&self) -> impl Iterator<Item = (Octant, &T)> {
        self.into_iter()
    }

    /// Transforms the items in the map into a new map.
    pub fn map<U, F: FnMut(Octant, T) -> U>(self, mut f: F) -> OctantMap<U> {
        let [i0, i1, i2, i3, i4, i5, i6, i7] = self.0;
        OctantMap([
            f(Octant::Z0Y0X0, i0),
            f(Octant::Z0Y0X1, i1),
            f(Octant::Z0Y1X0, i2),
            f(Octant::Z0Y1X1, i3),
            f(Octant::Z1Y0X0, i4),
            f(Octant::Z1Y0X1, i5),
            f(Octant::Z1Y1X0, i6),
            f(Octant::Z1Y1X1, i7),
        ])
    }

    /// Transforms the items in the map, by reference, into a new map.
    pub fn map_ref<'a, 'b: 'a, U: 'b, F: FnMut(Octant, &'a T) -> U>(&'a self, mut f: F) -> OctantMap<U> {
        unsafe {
            OctantMap([
                f(Octant::Z0Y0X0, self.0.get_unchecked(0)),
                f(Octant::Z0Y0X1, self.0.get_unchecked(1)),
                f(Octant::Z0Y1X0, self.0.get_unchecked(2)),
                f(Octant::Z0Y1X1, self.0.get_unchecked(3)),
                f(Octant::Z1Y0X0, self.0.get_unchecked(4)),
                f(Octant::Z1Y0X1, self.0.get_unchecked(5)),
                f(Octant::Z1Y1X0, self.0.get_unchecked(6)),
                f(Octant::Z1Y1X1, self.0.get_unchecked(7)),
            ])
        }
    }
}

impl<T: Clone> OctantMap<&T> {
    /// Clones the elements in the map.
    pub fn cloned(&self) -> OctantMap<T> {
        self.map(|_, x| x.clone())
    }
}

impl<'a, T> IntoIterator for &'a OctantMap<T> {
    type Item = (Octant, &'a T);

    type IntoIter = std::iter::Map<std::iter::Enumerate<std::slice::Iter<'a, T>>, fn((usize, &'a T)) -> (Octant, &'a T)>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe { self.0.iter().enumerate().map(|(d, reader)| (Octant::from_raw(d as u8), reader)) }
    }
}

impl<T> Index<Octant> for OctantMap<T> {
    type Output = T;

    fn index(&self, octant: Octant) -> &Self::Output {
        self.get(octant)
    }
}

impl<T> IndexMut<Octant> for OctantMap<T> {
    fn index_mut(&mut self, octant: Octant) -> &mut Self::Output {
        self.get_mut(octant)
    }
}