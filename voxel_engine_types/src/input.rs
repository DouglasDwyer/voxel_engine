use crate::math::*;
use private::*;
use serde::de::*;
use serde::*;
use std::hash::*;
use std::marker::*;
use wings::*;

/// Allows for reading from the user's input devices. Only available on the [`Client`](crate::Client).
#[system_trait(host)]
pub trait Input: 'static {
    /// Gets the value of a raw input, without considering whether any actions
    /// are registered with it.
    fn get_raw(&self, raw_input: RawInput) -> f32;

    /// Gets the mouse cursor's movement for this frame. This value
    /// is given in device units, adjusted by mouse sensitivity.
    fn pointer_delta(&self) -> Vec2;

    /// Gets the normalized direction that the user's mouse is pointing
    /// in 3D space. When the pointer is locked, this is always relative
    /// to the screen center. When the pointer is over a UI element (or
    /// something other than the game) this returns `None`.
    fn pointer_direction(&self) -> Option<Vec3A>;

    /// Returns whether the pointer is currently locked to the center of the screen.
    fn pointer_locked(&self) -> bool;

    /// Sets whether the mouse cursor will be invisible and locked to the center of the screen.
    /// The user's mouse movements can then be read with [`Self::pointer_delta`].
    fn set_pointer_locked(&mut self, locked: bool);

    /// Gets the number of ticks that the mouse wheel has scrolled
    /// this frame. This value is given in device units.
    fn scroll_delta(&self) -> IVec2;

    /// Gets a handle referencing the given analog action,
    /// which may take on a continuous range of values.
    /// The action is created if it does not exist.
    #[doc(hidden)]
    fn define_analog(&mut self, descriptor: ActionDescriptor<Analog>) -> ActionId<Analog>;

    /// Gets a handle referencing the given digital action,
    /// which may be either `true` or `false`. The action is created if it does not exist.
    #[doc(hidden)]
    fn define_digital(&mut self, descriptor: ActionDescriptor<Digital>) -> ActionId<Digital>;

    /// Gets the current value of the provided analog action.
    #[doc(hidden)]
    fn get_analog(&self, id: ActionId<Analog>) -> f32;

    /// Gets the current value of the provided digital action.
    #[doc(hidden)]
    fn get_digital(&self, id: ActionId<Digital>) -> DigitalResult;
}

impl dyn Input {
    /// Gets a handle referencing the given action, registering the action if it did not exist.
    pub fn define<I: InputKind>(&mut self, descriptor: ActionDescriptor<I>) -> ActionId<I> {
        I::define(self, descriptor)
    }

    /// Gets the current value of the provided action.
    pub fn get<I: InputKind>(&self, id: ActionId<I>) -> I::Result {
        I::get(self, id)
    }
}

/// Inputs that return a continuous range of values.
/// The neutral/default value returned is 0.0.
#[derive(Copy, Clone, Debug, Hash, Default, PartialEq, Eq)]
pub struct Analog;

/// Inputs that return either `true` or `false`.
/// The neutral/default value returned is `false`.
#[derive(Copy, Clone, Debug, Hash, Default, PartialEq, Eq)]
pub struct Digital;

/// Identifies a certain kind of input.
pub trait InputKind: Sealed + Sized {
    /// The type that identifies buttons or joysticks of this kind on user input devices.
    type Binding: Copy + std::fmt::Debug + PartialEq + Serialize + DeserializeOwned;

    /// The type of value returned when querying this input.
    type Result: Copy + std::fmt::Debug + PartialEq + Serialize + DeserializeOwned;

    /// Defines a new action of this type.
    fn define(input: &mut dyn Input, descriptor: ActionDescriptor<Self>) -> ActionId<Self>;

    /// Gets the state of the given action.
    fn get(input: &dyn Input, id: ActionId<Self>) -> Self::Result;
}

impl InputKind for Analog {
    type Binding = AnalogBinding;

    type Result = f32;

    fn define(input: &mut dyn Input, descriptor: ActionDescriptor<Self>) -> ActionId<Self> {
        input.define_analog(descriptor)
    }

    fn get(input: &dyn Input, id: ActionId<Self>) -> Self::Result {
        input.get_analog(id)
    }
}

impl InputKind for Digital {
    type Binding = DigitalBinding;

    type Result = DigitalResult;

    fn define(input: &mut dyn Input, descriptor: ActionDescriptor<Self>) -> ActionId<Self> {
        input.define_digital(descriptor)
    }

    fn get(input: &dyn Input, id: ActionId<Self>) -> Self::Result {
        input.get_digital(id)
    }
}

/// Identifies an action that has been bound for user input.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ActionId<I: InputKind>(u64, PhantomData<fn(I)>);

impl<I: InputKind> From<u64> for ActionId<I> {
    fn from(value: u64) -> Self {
        Self(value, PhantomData)
    }
}

impl<I: InputKind> From<ActionId<I>> for u64 {
    fn from(value: ActionId<I>) -> Self {
        value.0
    }
}

/// Identifies an action and describes its default parameters.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActionDescriptor<I: InputKind> {
    /// A description of the action, to be displayed to the user.
    pub description: String,
    /// A list of default buttons. The first button that exists when
    /// this action is created will automatically be bound to it.
    pub default_bindings: Vec<I::Binding>,
    /// The name of the action.
    pub name: ActionName,
}

impl<I: InputKind> ActionDescriptor<I> {
    /// Creates a new descriptor associated with the given system.
    pub fn new(
        name: ActionName,
        description: impl Into<String>,
        default_bindings: &[I::Binding],
    ) -> Self {
        Self {
            default_bindings: default_bindings.to_vec(),
            description: description.into(),
            name,
        }
    }
}

/// Identifies an action by its name and source system.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActionName {
    /// The name associated with the action.
    pub name: String,
    /// The system defining this action.
    pub system: ExportedType,
}

impl ActionName {
    /// Creates a new action name associated with the given system.
    pub fn new<S: wings::marshal::ExportType>(name: &str) -> Self {
        Self {
            name: name.to_string(),
            system: S::TYPE.into(),
        }
    }
}

/// Describes the current state of a digital action.
#[derive(
    Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct DigitalResult {
    /// Whether the button is currently pressed.
    pub held: bool,
    /// Whether the button was held last frame but not this frame.
    pub released: bool,
    /// Whether the button was held this frame but not last frame.
    pub pressed: bool,
}

/// Determines how a raw user input will affect an analog action.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AnalogBinding {
    /// Whether the input should be multiplied by `-1.0` before being returned.
    pub invert: bool,
    /// The raw input to read.
    pub raw_input: RawInput,
}

/// Determines how a raw user input will affect an analog action.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DigitalBinding {
    /// The signed value beyond which this input will be considered active. If negative,
    /// then the raw input must return a lower value than `threshold` to activate.
    /// If positive, then the raw input must return a higher value.
    pub threshold: f32,
    /// The raw input to read.
    pub raw_input: RawInput,
}

/// Identifies a source to which an action may be bound.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RawInput {
    /// The input refers to an axis on a gamepad.
    GamepadAxis(GamepadAxis),
    /// The input refers to a button on a gamepad.
    GamepadButton(GamepadButton),
    /// The input refers to a button on a keyboard.
    Key(Key),
    /// The input refers to a button a mouse.
    MouseButton(MouseButton),
}

/// Identifies a continuous axis on a gamepad, returning a value on the range `[-1.0, 1.0]`.
/// Follows the [gilrs standard layout](https://docs.rs/gilrs/latest/gilrs/).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum GamepadAxis {
    /// The horizontal axis of the left stick.
    LeftStickX,
    /// The vertical axis of the left stick.
    LeftStickY,
    /// The left Z-stick.
    LeftZ,
    /// The horizontal axis of the right stick.
    RightStickX,
    /// The vertical axis of the right stick.
    RightStickY,
    /// The right Z-stick.
    RightZ,
    /// The horizontal axis of the D-pad.
    DPadX,
    /// The vertical axis of the D-pad.
    DPadY,
}

impl GamepadAxis {
    /// A list of all possible gamepad axis values.
    pub const ALL: [Self; 8] = [
        Self::LeftStickX,
        Self::LeftStickY,
        Self::LeftZ,
        Self::RightStickX,
        Self::RightStickY,
        Self::RightZ,
        Self::DPadX,
        Self::DPadY,
    ];
}

/// Denotes a key on a user's keyboard.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Key {
    /// The `A` key.
    A,
    /// The `B` key.
    B,
    /// The `C` key.
    C,
    /// The `D` key.
    D,
    /// The `E` key.
    E,
    /// The `F` key.
    F,
    /// The `G` key.
    G,
    /// The `H` key.
    H,
    /// The `I` key.
    I,
    /// The `J` key.
    J,
    /// The `K` key.
    K,
    /// The `L` key.
    L,
    /// The `M` key.
    M,
    /// The `N` key.
    N,
    /// The `O` key.
    O,
    /// The `P` key.
    P,
    /// The `Q` key.
    Q,
    /// The `R` key.
    R,
    /// The `S` key.
    S,
    /// The `T` key.
    T,
    /// The `U` key.
    U,
    /// The `V` key.
    V,
    /// The `W` key.
    W,
    /// The `X` key.
    X,
    /// The `Y` key.
    Y,
    /// The `Z` key.
    Z,
    /// The `esc` key.
    Escape,
    /// The `F1` key.
    F1,
    /// The `F2` key.
    F2,
    /// The `F3` key.
    F3,
    /// The `F4` key.
    F4,
    /// The `F5` key.
    F5,
    /// The `F6` key.
    F6,
    /// The `F7` key.
    F7,
    /// The `F8` key.
    F8,
    /// The `F9` key.
    F9,
    /// The `F10` key.
    F10,
    /// The `F11` key.
    F11,
    /// The `F12` key.
    F12,
    /// The `F13` key.
    F13,
    /// The `F14` key.
    F14,
    /// The `F15` key.
    F15,
    /// The `F16` key.
    F16,
    /// The `F17` key.
    F17,
    /// The `F18` key.
    F18,
    /// The `F19` key.
    F19,
    /// The `F20` key.
    F20,
    /// The `F21` key.
    F21,
    /// The `F22` key.
    F22,
    /// The `F23` key.
    F23,
    /// The `F24` key.
    F24,
    /// The `1` key.
    Key1,
    /// The `2` key.
    Key2,
    /// The `3` key.
    Key3,
    /// The `4` key.
    Key4,
    /// The `5` key.
    Key5,
    /// The `6` key.
    Key6,
    /// The `7` key.
    Key7,
    /// The `8` key.
    Key8,
    /// The `9` key.
    Key9,
    /// The `0` key.
    Key0,
    /// The `numlock` key.
    Numlock,
    /// The `0` key on the numpad.
    Numpad0,
    /// The `1` key on the numpad.
    Numpad1,
    /// The `2` key on the numpad.
    Numpad2,
    /// The `3` key on the numpad.
    Numpad3,
    /// The `4` key on the numpad.
    Numpad4,
    /// The `5` key on the numpad.
    Numpad5,
    /// The `6` key on the numpad.
    Numpad6,
    /// The `7` key on the numpad.
    Numpad7,
    /// The `8` key on the numpad.
    Numpad8,
    /// The `9` key on the numpad.
    Numpad9,
    /// The `+` key on the numpad.
    NumpadPlus,
    /// The `-` key on the numpad.
    NumpadMinus,
    /// The `*` key on the numpad.
    NumpadAsterisk,
    /// The `/` key on the numpad.
    NumpadSlash,
    /// The `.` key on the numpad.
    NumpadDecimal,
    /// The `enter` key on the numpad.
    NumpadEnter,
    /// The `snapshot` key.
    Snapshot,
    /// The `scroll lock` key.
    ScrollLock,
    /// The `pause` key.
    Pause,
    /// The `insert` key.
    Insert,
    /// The `home` key.
    Home,
    /// The `delete` key.
    Delete,
    /// The `end` key.
    End,
    /// The `page up` key.
    PageUp,
    /// The `page down` key.
    PageDown,
    /// The `left` arrow key.
    Left,
    /// The `right` arrow key.
    Right,
    /// The `up` arrow key.
    Up,
    /// The `down` arrow key.
    Down,
    /// The `grave` key.
    Grave,
    /// The `back` key.
    Back,
    /// The `tab` key.
    Tab,
    /// The `caps` key.
    CapitalLock,
    /// The `return` key.
    Return,
    /// The `space` key.
    Space,
    /// The left `alt` key.
    LAlt,
    /// The right `alt` key.
    RAlt,
    /// The left `shift` key.
    LShift,
    /// The right `shift` key.
    RShift,
    /// The left `ctrl` key.
    LControl,
    /// The right `ctrl` key.
    RControl,
    /// The left `windows` or `command` key.
    LWin,
    /// The right `windows` or `command` key.
    RWin,
    /// The `-` key.
    Minus,
    /// The `=` key.
    Equals,
    /// The `[` key.
    LBracket,
    /// The `]` key.
    RBracket,
    /// The `\` key.
    Backslash,
    /// The `;` key.
    Semicolon,
    /// The `'` key.
    Apostrophe,
    /// The `,` key.
    Comma,
    /// The `.` key.
    Period,
    /// The `/` key.
    Slash,
}

impl Key {
    /// A list of all possible key values.
    pub const ALL: [Self; 115] = [
        Self::A,
        Self::B,
        Self::C,
        Self::D,
        Self::E,
        Self::F,
        Self::G,
        Self::H,
        Self::I,
        Self::J,
        Self::K,
        Self::L,
        Self::M,
        Self::N,
        Self::O,
        Self::P,
        Self::Q,
        Self::R,
        Self::S,
        Self::T,
        Self::U,
        Self::V,
        Self::W,
        Self::X,
        Self::Y,
        Self::Z,
        Self::Escape,
        Self::F1,
        Self::F2,
        Self::F3,
        Self::F4,
        Self::F5,
        Self::F6,
        Self::F7,
        Self::F8,
        Self::F9,
        Self::F10,
        Self::F11,
        Self::F12,
        Self::F13,
        Self::F14,
        Self::F15,
        Self::F16,
        Self::F17,
        Self::F18,
        Self::F19,
        Self::F20,
        Self::F21,
        Self::F22,
        Self::F23,
        Self::F24,
        Self::Key1,
        Self::Key2,
        Self::Key3,
        Self::Key4,
        Self::Key5,
        Self::Key6,
        Self::Key7,
        Self::Key8,
        Self::Key9,
        Self::Key0,
        Self::Numlock,
        Self::Numpad0,
        Self::Numpad1,
        Self::Numpad2,
        Self::Numpad3,
        Self::Numpad4,
        Self::Numpad5,
        Self::Numpad6,
        Self::Numpad7,
        Self::Numpad8,
        Self::Numpad9,
        Self::NumpadPlus,
        Self::NumpadMinus,
        Self::NumpadAsterisk,
        Self::NumpadSlash,
        Self::NumpadDecimal,
        Self::NumpadEnter,
        Self::Snapshot,
        Self::ScrollLock,
        Self::Pause,
        Self::Insert,
        Self::Home,
        Self::Delete,
        Self::End,
        Self::PageUp,
        Self::PageDown,
        Self::Left,
        Self::Right,
        Self::Up,
        Self::Down,
        Self::Grave,
        Self::Back,
        Self::Tab,
        Self::CapitalLock,
        Self::Return,
        Self::Space,
        Self::LAlt,
        Self::RAlt,
        Self::LShift,
        Self::RShift,
        Self::LControl,
        Self::RControl,
        Self::LWin,
        Self::RWin,
        Self::Minus,
        Self::Equals,
        Self::LBracket,
        Self::RBracket,
        Self::Backslash,
        Self::Semicolon,
        Self::Apostrophe,
        Self::Comma,
        Self::Period,
        Self::Slash,
    ];
}

/// Identifies a button on a controller, following the [gilrs standard layout](https://docs.rs/gilrs/latest/gilrs/).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum GamepadButton {
    /// The bottom button on the action pad.
    South,
    /// The right button on the action pad.
    East,
    /// The north button on the action pad.
    North,
    /// The west button on the action pad.
    West,
    /// The C-button.
    C,
    /// The Z-button.
    Z,
    /// The first left trigger.
    LeftTrigger,
    /// The second left trigger.
    LeftTrigger2,
    /// The first right trigger.
    RightTrigger,
    /// The second right trigger.
    RightTrigger2,
    /// The select button.
    Select,
    /// The start button.
    Start,
    /// The mode button.
    Mode,
    /// The left thumb button.
    LeftThumb,
    /// The right thumb button.
    RightThumb,
    /// The D-pad up button.
    DPadUp,
    /// The D-pad down button.
    DPadDown,
    /// The D-pad left button.
    DPadLeft,
    /// The D-pad right button.
    DPadRight,
}

impl GamepadButton {
    /// A list of all possible gamepad button values.
    pub const ALL: [Self; 19] = [
        Self::South,
        Self::East,
        Self::North,
        Self::West,
        Self::C,
        Self::Z,
        Self::LeftTrigger,
        Self::LeftTrigger2,
        Self::RightTrigger,
        Self::RightTrigger2,
        Self::Select,
        Self::Start,
        Self::Mode,
        Self::LeftThumb,
        Self::RightThumb,
        Self::DPadUp,
        Self::DPadDown,
        Self::DPadLeft,
        Self::DPadRight,
    ];
}

/// Indicates a button on the user's mouse.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum MouseButton {
    /// The left mouse button.
    Left,
    /// The center mouse button.
    Middle,
    /// The right mouse button.
    Right,
}

impl MouseButton {
    /// A list of all possible mouse button values.
    pub const ALL: [Self; 3] = [Self::Left, Self::Middle, Self::Right];
}

/// Hides internal implementation details.
mod private {
    use super::*;

    /// Prevents third-party crates from implementing a trait.
    pub trait Sealed {}

    impl Sealed for Analog {}
    impl Sealed for Digital {}
}
