use serde::*;
use wings::*;

/// Allows for reading from the user's input devices.
#[system_trait(host)]
pub trait Input: 'static {
    /// Gets a handle referencing the given analog action,
    /// which may take on a continuous range of values between `0.0` and `1.0`.
    /// The action is created if it does not exist.
    fn define_analog(&mut self, descriptor: ActionDescriptor) -> AnalogActionId;
    
    /// Gets a handle referencing the given analog action,
    /// which may be either `true` or `false`. The action is created if it does not exist.
    fn define_digital(&mut self, descriptor: ActionDescriptor) -> DigitalActionId;

    /// Gets the current value of the provided analog action.
    fn get_analog(&self, id: AnalogActionId) -> f32;
    
    /// Gets the current value of the provided digital action.
    fn get_digital(&self, id: DigitalActionId) -> DigitalActionResult;
}

/// Identifies an action and describes its default parameters.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActionDescriptor {
    /// A list of default buttons. The first button that exists when
    /// this action is created will automatically be bound to it.
    pub default_bindings: Vec<InputBinding>,
	/// The name of the action.
	pub name: ActionName
}

impl ActionDescriptor {
    /// Creates a new descriptor associated with the given system.
    pub fn new<S: wings::marshal::ExportType>(name: ActionName, default_bindings: &[InputBinding]) -> Self {
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
pub struct DigitalActionResult {
    /// Whether the button is currently pressed.
    pub held: bool,
    /// Whether the button was held last frame but not this frame.
    pub released: bool,
    /// Whether the button was held this frame but not last frame.
    pub pressed: bool
}

/// Identifies an analog action, which can take on a continuous range of values between `0.0` and `1.0`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AnalogActionId(u64);

impl From<u64> for AnalogActionId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<AnalogActionId> for u64 {
    fn from(value: AnalogActionId) -> Self {
        value.0
    }
}

/// Identifies a digital action, which can either be `true` or `false`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DigitalActionId(u64);

impl From<u64> for DigitalActionId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<DigitalActionId> for u64 {
    fn from(value: DigitalActionId) -> Self {
        value.0
    }
}

/// Identifies a source to which an action may be bound.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum InputBinding {
    /// A key on a keyboard.
    Keyboard(KeyInput),
    /// A button or mouse wheel.
    Mouse(MouseInput),
}

/// Denotes a key on a user's keyboard.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum KeyInput {
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

/// Indicates a button on the user's mouse.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MouseInput {
    /// The left mouse button.
    Left,
    /// The center mouse button.
    Middle,
    /// The right mouse button.
    Right,
    /// The analog scrolling wheel.
    Wheel
}