use private::*;
use serde::*;
use serde::de::*;
use std::hash::*;
use std::marker::*;
use wings::*;

/// Allows for reading from the user's input devices.
#[system_trait(host)]
pub trait Input: 'static {
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
	type Binding = AnalogInput;

	type Result = f32;
	
	fn define(input: &mut dyn Input, descriptor: ActionDescriptor<Self>) -> ActionId<Self> {
		input.define_analog(descriptor)
	}
	
	fn get(input: &dyn Input, id: ActionId<Self>) -> Self::Result {
		input.get_analog(id)
	}
}

impl InputKind for Digital {
	type Binding = DigitalInput;

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
    /// A list of default buttons. The first button that exists when
    /// this action is created will automatically be bound to it.
    pub default_bindings: Vec<I::Binding>,
	/// The name of the action.
	pub name: ActionName
}

impl<I: InputKind> ActionDescriptor<I> {
    /// Creates a new descriptor associated with the given system.
    pub fn new(name: ActionName, default_bindings: &[I::Binding]) -> Self {
        Self {
            default_bindings: default_bindings.to_vec(),
            name
        }
    }
}

/// Identifies an action by its name and source system.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActionName {
    /// The name associated with the action.
    pub name: String,
    /// The system defining this action.
    pub system: ExportedType
}

impl ActionName {
    /// Creates a new action name associated with the given system.
    pub fn new<S: wings::marshal::ExportType>(name: &str) -> Self {
        Self {
            name: name.to_string(),
            system: S::TYPE.into()
        }
    }
}

/// Describes the current state of a digital action.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DigitalResult {
    /// Whether the button is currently pressed.
    pub held: bool,
    /// Whether the button was held last frame but not this frame.
    pub released: bool,
    /// Whether the button was held this frame but not last frame.
    pub pressed: bool
}

/// Identifies a source to which an analog action may be bound.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AnalogInput {
	/// An axis on a gamepad.
	Gamepad(GamepadAxis),
	/// An axis on a mouse.
	Mouse(MouseAxis)
}

/// Identifies a continuous axis on a gamepad, returning a value on the range `[-1.0, 1.0]`.
/// Follows the [gilrs standard layout](https://docs.rs/gilrs/ev/enum.Button.html).
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
    DPadY
}

/// Identifies a continuous axis on a mouse.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MouseAxis {
	/// The horizontal wheel axis. Returns a real number corresponding to the number
	/// of wheel "clicks" that occurred on the current frame.
	WheelX,
	/// The vertical wheel axis. Returns a real number corresponding to the number
	/// of wheel "clicks" that occurred on the current frame.
	WheelY
}

/// Identifies a source to which a digital action may be bound.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DigitalInput {
	/// A button on a controller.
	Gamepad(GamepadButton),
    /// A key on a keyboard.
    Keyboard(Key),
    /// A mouse button.
    Mouse(MouseButton),
}

/// Denotes a key on a user's keyboard.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Key {
	// Alphabet
	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
	I,
	J,
	K,
	L,
	M,
	N,
	O,
	P,
	Q,
	R,
	S,
	T,
	U,
	V,
	W,
	X,
	Y,
	Z,
	// Function Keys
	Escape,
	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	F11,
	F12,
	F13,
	F14,
	F15,
	F16,
	F17,
	F18,
	F19,
	F20,
	F21,
	F22,
	F23,
	F24,
	// Number Keys (Not Numpad)
	Key1,
	Key2,
	Key3,
	Key4,
	Key5,
	Key6,
	Key7,
	Key8,
	Key9,
	Key0,
	// Numpad Keys
	Numlock,
	Numpad0,
	Numpad1,
	Numpad2,
	Numpad3,
	Numpad4,
	Numpad5,
	Numpad6,
	Numpad7,
	Numpad8,
	Numpad9,
	NumpadPlus,
	NumpadMinus,
	NumpadAsterisk,
	NumpadSlash,
	NumpadDecimal,
	NumpadEnter,
	// Control Keys
	Snapshot,
	ScrollLock,
	Pause,
	// Home Keys
	Insert,
	Home,
	Delete,
	End,
	PageUp,
	PageDown,
	// Arrow Keys
	Left,
	Right,
	Up,
	Down,
	// Keyboard Controls
	Grave,
	Back,
	Tab,
	CapitalLock,
	Return,
	Space,
	// Modifiers
	LAlt,
	RAlt,
	LShift,
	RShift,
	LControl,
	RControl,
	LWin,
	RWin,
	// Alpha-adjacent
	Minus,
	Equals,
	LBracket,
	RBracket,
	Backslash,
	Semicolon,
	Apostrophe,
	Comma,
	Period,
	Slash,
}

/// Identifies a button on a controller, following the [gilrs standard layout](https://docs.rs/gilrs/ev/enum.Button.html).
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
    DPadRight
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
    Right
}

/// Hides internal implementation details.
mod private {
	use super::*;

	/// Prevents third-party crates from implementing a trait.
	pub trait Sealed {}

	impl Sealed for Analog {}
	impl Sealed for Digital {}
}