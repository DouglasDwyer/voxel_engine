use crate::math::*;
use wings::*;

/// Allows for setting various properties of the current player.
#[system_trait(host)]
pub trait Player: 'static {
    /// Gets the player's current transform.
    fn get_transform(&self) -> Transform;

    /// Sets the player's current transform.
    fn set_transform(&mut self, transform: Transform);
}