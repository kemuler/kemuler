pub use enigo::{
    Enigo, Key as EnigoKeyboard, KeyboardControllable, MouseButton as EnigoMouseButton,
    MouseControllable,
};

use crate::{
    combinator::Combine,
    emulatable::{EmulateAbsoluteValue, EmulateRelativeValue},
};

macro_rules! crate_impls {
    ($thing:ident) => {
        impl Combine for $thing {}

        impl<T> std::ops::Add<T> for $thing {
            type Output = crate::combinator::And<Self, T>;

            fn add(self, rhs: T) -> Self::Output {
                self.and(rhs)
            }
        }
    };
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyboard {
    #[cfg(target_os = "windows")]
    Num0,
    #[cfg(target_os = "windows")]
    Num1,
    #[cfg(target_os = "windows")]
    Num2,
    #[cfg(target_os = "windows")]
    Num3,
    #[cfg(target_os = "windows")]
    Num4,
    #[cfg(target_os = "windows")]
    Num5,
    #[cfg(target_os = "windows")]
    Num6,
    #[cfg(target_os = "windows")]
    Num7,
    #[cfg(target_os = "windows")]
    Num8,
    #[cfg(target_os = "windows")]
    Num9,
    #[cfg(target_os = "windows")]
    A,
    #[cfg(target_os = "windows")]
    B,
    #[cfg(target_os = "windows")]
    C,
    #[cfg(target_os = "windows")]
    D,
    #[cfg(target_os = "windows")]
    E,
    #[cfg(target_os = "windows")]
    F,
    #[cfg(target_os = "windows")]
    G,
    #[cfg(target_os = "windows")]
    H,
    #[cfg(target_os = "windows")]
    I,
    #[cfg(target_os = "windows")]
    J,
    #[cfg(target_os = "windows")]
    K,
    #[cfg(target_os = "windows")]
    L,
    #[cfg(target_os = "windows")]
    M,
    #[cfg(target_os = "windows")]
    N,
    #[cfg(target_os = "windows")]
    O,
    #[cfg(target_os = "windows")]
    P,
    #[cfg(target_os = "windows")]
    Q,
    #[cfg(target_os = "windows")]
    R,
    #[cfg(target_os = "windows")]
    S,
    #[cfg(target_os = "windows")]
    T,
    #[cfg(target_os = "windows")]
    U,
    #[cfg(target_os = "windows")]
    V,
    #[cfg(target_os = "windows")]
    W,
    #[cfg(target_os = "windows")]
    X,
    #[cfg(target_os = "windows")]
    Y,
    #[cfg(target_os = "windows")]
    Z,
    #[cfg(target_os = "windows")]
    AbntC1,
    #[cfg(target_os = "windows")]
    AbntC2,
    #[cfg(target_os = "windows")]
    Accept,
    #[cfg(target_os = "windows")]
    Add,
    /// alt key on Linux and Windows (option key on macOS)
    Alt,
    #[cfg(target_os = "windows")]
    Apps,
    #[cfg(target_os = "windows")]
    Attn,
    /// backspace key
    Backspace,
    #[cfg(target_os = "linux")]
    Begin,
    #[cfg(target_os = "linux")]
    Break,
    #[cfg(target_os = "windows")]
    BrowserBack,
    #[cfg(target_os = "windows")]
    BrowserFavorites,
    #[cfg(target_os = "windows")]
    BrowserForward,
    #[cfg(target_os = "windows")]
    BrowserHome,
    #[cfg(target_os = "windows")]
    BrowserRefresh,
    #[cfg(target_os = "windows")]
    BrowserSearch,
    #[cfg(target_os = "windows")]
    BrowserStop,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Cancel,
    /// caps lock key
    CapsLock,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Clear,
    /// control key
    Control,
    #[cfg(target_os = "windows")]
    Convert,
    #[cfg(target_os = "windows")]
    Crsel,
    #[cfg(target_os = "windows")]
    DBEAlphanumeric,
    #[cfg(target_os = "windows")]
    DBECodeinput,
    #[cfg(target_os = "windows")]
    DBEDetermineString,
    #[cfg(target_os = "windows")]
    DBEEnterDLGConversionMode,
    #[cfg(target_os = "windows")]
    DBEEnterIMEConfigMode,
    #[cfg(target_os = "windows")]
    DBEEnterWordRegisterMode,
    #[cfg(target_os = "windows")]
    DBEFlushString,
    #[cfg(target_os = "windows")]
    DBEHiragana,
    #[cfg(target_os = "windows")]
    DBEKatakana,
    #[cfg(target_os = "windows")]
    DBENoCodepoint,
    #[cfg(target_os = "windows")]
    DBENoRoman,
    #[cfg(target_os = "windows")]
    DBERoman,
    #[cfg(target_os = "windows")]
    DBESBCSChar,
    #[cfg(target_os = "windows")]
    DBESChar,
    #[cfg(target_os = "windows")]
    Decimal,
    /// delete key
    Delete,
    #[cfg(target_os = "windows")]
    Divide,
    /// down arrow key
    DownArrow,
    /// end key
    End,
    #[cfg(target_os = "windows")]
    Ereof,
    /// escape key (esc)
    Escape,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Execute,
    #[cfg(target_os = "windows")]
    Exsel,
    /// F1 key
    F1,
    /// F2 key
    F2,
    /// F3 key
    F3,
    /// F4 key
    F4,
    /// F5 key
    F5,
    /// F6 key
    F6,
    /// F7 key
    F7,
    /// F8 key
    F8,
    /// F9 key
    F9,
    /// F10 key
    F10,
    /// F11 key
    F11,
    /// F12 key
    F12,
    /// F13 key
    F13,
    /// F14 key
    F14,
    /// F15 key
    F15,
    /// F16 key
    F16,
    /// F17 key
    F17,
    /// F18 key
    F18,
    /// F19 key
    F19,
    /// F20 key
    F20,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    /// F21 key
    F21,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    /// F22 key
    F22,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    /// F23 key
    F23,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    /// F24 key
    F24,
    #[cfg(target_os = "linux")]
    F25,
    #[cfg(target_os = "linux")]
    F26,
    #[cfg(target_os = "linux")]
    F27,
    #[cfg(target_os = "linux")]
    F28,
    #[cfg(target_os = "linux")]
    F29,
    #[cfg(target_os = "linux")]
    F30,
    #[cfg(target_os = "linux")]
    F31,
    #[cfg(target_os = "linux")]
    F32,
    #[cfg(target_os = "linux")]
    F33,
    #[cfg(target_os = "linux")]
    F34,
    #[cfg(target_os = "linux")]
    F35,
    #[cfg(target_os = "macos")]
    Function,
    #[cfg(target_os = "windows")]
    Final,
    #[cfg(target_os = "linux")]
    Find,
    #[cfg(target_os = "windows")]
    GamepadA,
    #[cfg(target_os = "windows")]
    GamepadB,
    #[cfg(target_os = "windows")]
    GamepadDPadDown,
    #[cfg(target_os = "windows")]
    GamepadDPadLeft,
    #[cfg(target_os = "windows")]
    GamepadDPadRight,
    #[cfg(target_os = "windows")]
    GamepadDPadUp,
    #[cfg(target_os = "windows")]
    GamepadLeftShoulder,
    #[cfg(target_os = "windows")]
    GamepadLeftThumbstickButton,
    #[cfg(target_os = "windows")]
    GamepadLeftThumbstickDown,
    #[cfg(target_os = "windows")]
    GamepadLeftThumbstickLeft,
    #[cfg(target_os = "windows")]
    GamepadLeftThumbstickRight,
    #[cfg(target_os = "windows")]
    GamepadLeftThumbstickUp,
    #[cfg(target_os = "windows")]
    GamepadLeftTrigger,
    #[cfg(target_os = "windows")]
    GamepadMenu,
    #[cfg(target_os = "windows")]
    GamepadRightShoulder,
    #[cfg(target_os = "windows")]
    GamepadRightThumbstickButton,
    #[cfg(target_os = "windows")]
    GamepadRightThumbstickDown,
    #[cfg(target_os = "windows")]
    GamepadRightThumbstickLeft,
    #[cfg(target_os = "windows")]
    GamepadRightThumbstickRight,
    #[cfg(target_os = "windows")]
    GamepadRightThumbstickUp,
    #[cfg(target_os = "windows")]
    GamepadRightTrigger,
    #[cfg(target_os = "windows")]
    GamepadView,
    #[cfg(target_os = "windows")]
    GamepadX,
    #[cfg(target_os = "windows")]
    GamepadY,
    #[cfg(target_os = "windows")]
    Hangeul,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Hangul,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Hanja,
    Help,
    /// home key
    Home,
    #[cfg(target_os = "windows")]
    Ico00,
    #[cfg(target_os = "windows")]
    IcoClear,
    #[cfg(target_os = "windows")]
    IcoHelp,
    #[cfg(target_os = "windows")]
    IMEOff,
    #[cfg(target_os = "windows")]
    IMEOn,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Insert,
    #[cfg(target_os = "windows")]
    Junja,
    #[cfg(target_os = "windows")]
    Kana,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Kanji,
    #[cfg(target_os = "windows")]
    LaunchApp1,
    #[cfg(target_os = "windows")]
    LaunchApp2,
    #[cfg(target_os = "windows")]
    LaunchMail,
    #[cfg(target_os = "windows")]
    LaunchMediaSelect,
    #[cfg(target_os = "macos")]
    /// Opens launchpad
    Launchpad,
    #[cfg(target_os = "windows")]
    LButton,
    LControl,
    /// left arrow key
    LeftArrow,
    #[cfg(target_os = "linux")]
    Linefeed,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    LMenu,
    LShift,
    #[cfg(target_os = "windows")]
    LWin,
    #[cfg(target_os = "windows")]
    MButton,
    #[cfg(target_os = "windows")]
    MediaNextTrack,
    #[cfg(target_os = "windows")]
    MediaPlayPause,
    #[cfg(target_os = "windows")]
    MediaPrevTrack,
    #[cfg(target_os = "windows")]
    MediaStop,
    /// meta key (also known as "windows", "super", and "command")
    Meta,
    #[cfg(target_os = "macos")]
    /// Opens mission control
    MissionControl,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    ModeChange,
    #[cfg(target_os = "windows")]
    Multiply,
    #[cfg(target_os = "windows")]
    NavigationAccept,
    #[cfg(target_os = "windows")]
    NavigationCancel,
    #[cfg(target_os = "windows")]
    NavigationDown,
    #[cfg(target_os = "windows")]
    NavigationLeft,
    #[cfg(target_os = "windows")]
    NavigationMenu,
    #[cfg(target_os = "windows")]
    NavigationRight,
    #[cfg(target_os = "windows")]
    NavigationUp,
    #[cfg(target_os = "windows")]
    NavigationView,
    #[cfg(target_os = "windows")]
    NoName,
    #[cfg(target_os = "windows")]
    NonConvert,
    #[cfg(target_os = "windows")]
    None,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Numlock,
    #[cfg(target_os = "windows")]
    Numpad0,
    #[cfg(target_os = "windows")]
    Numpad1,
    #[cfg(target_os = "windows")]
    Numpad2,
    #[cfg(target_os = "windows")]
    Numpad3,
    #[cfg(target_os = "windows")]
    Numpad4,
    #[cfg(target_os = "windows")]
    Numpad5,
    #[cfg(target_os = "windows")]
    Numpad6,
    #[cfg(target_os = "windows")]
    Numpad7,
    #[cfg(target_os = "windows")]
    Numpad8,
    #[cfg(target_os = "windows")]
    Numpad9,
    #[cfg(target_os = "windows")]
    OEM1,
    #[cfg(target_os = "windows")]
    OEM102,
    #[cfg(target_os = "windows")]
    OEM2,
    #[cfg(target_os = "windows")]
    OEM3,
    #[cfg(target_os = "windows")]
    OEM4,
    #[cfg(target_os = "windows")]
    OEM5,
    #[cfg(target_os = "windows")]
    OEM6,
    #[cfg(target_os = "windows")]
    OEM7,
    #[cfg(target_os = "windows")]
    OEM8,
    #[cfg(target_os = "windows")]
    OEMAttn,
    #[cfg(target_os = "windows")]
    OEMAuto,
    #[cfg(target_os = "windows")]
    OEMAx,
    #[cfg(target_os = "windows")]
    OEMBacktab,
    #[cfg(target_os = "windows")]
    OEMClear,
    #[cfg(target_os = "windows")]
    OEMComma,
    #[cfg(target_os = "windows")]
    OEMCopy,
    #[cfg(target_os = "windows")]
    OEMCusel,
    #[cfg(target_os = "windows")]
    OEMEnlw,
    #[cfg(target_os = "windows")]
    OEMFinish,
    #[cfg(target_os = "windows")]
    OEMFJJisho,
    #[cfg(target_os = "windows")]
    OEMFJLoya,
    #[cfg(target_os = "windows")]
    OEMFJMasshou,
    #[cfg(target_os = "windows")]
    OEMFJRoya,
    #[cfg(target_os = "windows")]
    OEMFJTouroku,
    #[cfg(target_os = "windows")]
    OEMJump,
    #[cfg(target_os = "windows")]
    OEMMinus,
    #[cfg(target_os = "windows")]
    OEMNECEqual,
    #[cfg(target_os = "windows")]
    OEMPA1,
    #[cfg(target_os = "windows")]
    OEMPA2,
    #[cfg(target_os = "windows")]
    OEMPA3,
    #[cfg(target_os = "windows")]
    OEMPeriod,
    #[cfg(target_os = "windows")]
    OEMPlus,
    #[cfg(target_os = "windows")]
    OEMReset,
    #[cfg(target_os = "windows")]
    OEMWsctrl,
    /// option key on macOS (alt key on Linux and Windows)
    Option,
    #[cfg(target_os = "windows")]
    PA1,
    #[cfg(target_os = "windows")]
    Packet,
    /// page down key
    PageDown,
    /// page up key
    PageUp,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Pause,
    #[cfg(target_os = "windows")]
    Play,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Print,
    #[cfg(target_os = "windows")]
    Processkey,
    #[cfg(target_os = "windows")]
    RButton,
    #[cfg(target_os = "macos")]
    RCommand,
    RControl,
    #[cfg(target_os = "linux")]
    Redo,
    /// return key
    Return,
    /// right arrow key
    RightArrow,
    #[cfg(target_os = "windows")]
    RMenu,
    #[cfg(target_os = "macos")]
    ROption,
    RShift,
    #[cfg(target_os = "windows")]
    RWin,
    #[cfg(target_os = "windows")]
    Scroll,
    #[cfg(target_os = "linux")]
    ScrollLock,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Select,
    #[cfg(target_os = "linux")]
    ScriptSwitch,
    #[cfg(target_os = "windows")]
    Separator,
    /// shift key
    Shift,
    #[cfg(target_os = "linux")]
    /// Lock shift key
    ShiftLock,
    #[cfg(target_os = "windows")]
    Sleep,
    #[cfg(target_os = "windows")]
    Snapshot,
    /// space key
    Space,
    #[cfg(target_os = "windows")]
    Subtract,
    #[cfg(target_os = "linux")]
    SysReq,
    /// tab key (tabulator)
    Tab,
    #[cfg(target_os = "linux")]
    Undo,
    /// up arrow key
    UpArrow,
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    VolumeDown,
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    VolumeMute,
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    VolumeUp,
    #[cfg(target_os = "windows")]
    XButton1,
    #[cfg(target_os = "windows")]
    XButton2,
    #[cfg(target_os = "windows")]
    Zoom,
    /// keyboard layout dependent key
    Layout(char),
    /// raw keycode eg 0x38
    Raw(u16),
}

impl Keyboard {
    fn as_enigo(&self) -> EnigoKeyboard {
        match self {
            Keyboard::Num0 => EnigoKeyboard::Num0,
            Keyboard::Num1 => EnigoKeyboard::Num1,
            Keyboard::Num2 => EnigoKeyboard::Num2,
            Keyboard::Num3 => EnigoKeyboard::Num3,
            Keyboard::Num4 => EnigoKeyboard::Num4,
            Keyboard::Num5 => EnigoKeyboard::Num5,
            Keyboard::Num6 => EnigoKeyboard::Num6,
            Keyboard::Num7 => EnigoKeyboard::Num7,
            Keyboard::Num8 => EnigoKeyboard::Num8,
            Keyboard::Num9 => EnigoKeyboard::Num9,
            Keyboard::A => EnigoKeyboard::A,
            Keyboard::B => EnigoKeyboard::B,
            Keyboard::C => EnigoKeyboard::C,
            Keyboard::D => EnigoKeyboard::D,
            Keyboard::E => EnigoKeyboard::E,
            Keyboard::F => EnigoKeyboard::F,
            Keyboard::G => EnigoKeyboard::G,
            Keyboard::H => EnigoKeyboard::H,
            Keyboard::I => EnigoKeyboard::I,
            Keyboard::J => EnigoKeyboard::J,
            Keyboard::K => EnigoKeyboard::K,
            Keyboard::L => EnigoKeyboard::L,
            Keyboard::M => EnigoKeyboard::M,
            Keyboard::N => EnigoKeyboard::N,
            Keyboard::O => EnigoKeyboard::O,
            Keyboard::P => EnigoKeyboard::P,
            Keyboard::Q => EnigoKeyboard::Q,
            Keyboard::R => EnigoKeyboard::R,
            Keyboard::S => EnigoKeyboard::S,
            Keyboard::T => EnigoKeyboard::T,
            Keyboard::U => EnigoKeyboard::U,
            Keyboard::V => EnigoKeyboard::V,
            Keyboard::W => EnigoKeyboard::W,
            Keyboard::X => EnigoKeyboard::X,
            Keyboard::Y => EnigoKeyboard::Y,
            Keyboard::Z => EnigoKeyboard::Z,
            Keyboard::AbntC1 => EnigoKeyboard::AbntC1,
            Keyboard::AbntC2 => EnigoKeyboard::AbntC2,
            Keyboard::Accept => EnigoKeyboard::Accept,
            Keyboard::Add => EnigoKeyboard::Add,
            Keyboard::Alt => EnigoKeyboard::Alt,
            Keyboard::Apps => EnigoKeyboard::Apps,
            Keyboard::Attn => EnigoKeyboard::Attn,
            Keyboard::Backspace => EnigoKeyboard::Backspace,
            Keyboard::BrowserBack => EnigoKeyboard::BrowserBack,
            Keyboard::BrowserFavorites => EnigoKeyboard::BrowserFavorites,
            Keyboard::BrowserForward => EnigoKeyboard::BrowserForward,
            Keyboard::BrowserHome => EnigoKeyboard::BrowserHome,
            Keyboard::BrowserRefresh => EnigoKeyboard::BrowserRefresh,
            Keyboard::BrowserSearch => EnigoKeyboard::BrowserSearch,
            Keyboard::BrowserStop => EnigoKeyboard::BrowserStop,
            Keyboard::Cancel => EnigoKeyboard::Cancel,
            Keyboard::CapsLock => EnigoKeyboard::CapsLock,
            Keyboard::Clear => EnigoKeyboard::Clear,
            Keyboard::Control => EnigoKeyboard::Control,
            Keyboard::Convert => EnigoKeyboard::Convert,
            Keyboard::Crsel => EnigoKeyboard::Crsel,
            Keyboard::DBEAlphanumeric => EnigoKeyboard::DBEAlphanumeric,
            Keyboard::DBECodeinput => EnigoKeyboard::DBECodeinput,
            Keyboard::DBEDetermineString => EnigoKeyboard::DBEDetermineString,
            Keyboard::DBEEnterDLGConversionMode => EnigoKeyboard::DBEEnterDLGConversionMode,
            Keyboard::DBEEnterIMEConfigMode => EnigoKeyboard::DBEEnterIMEConfigMode,
            Keyboard::DBEEnterWordRegisterMode => EnigoKeyboard::DBEEnterWordRegisterMode,
            Keyboard::DBEFlushString => EnigoKeyboard::DBEFlushString,
            Keyboard::DBEHiragana => EnigoKeyboard::DBEHiragana,
            Keyboard::DBEKatakana => EnigoKeyboard::DBEKatakana,
            Keyboard::DBENoCodepoint => EnigoKeyboard::DBENoCodepoint,
            Keyboard::DBENoRoman => EnigoKeyboard::DBENoRoman,
            Keyboard::DBERoman => EnigoKeyboard::DBERoman,
            Keyboard::DBESBCSChar => EnigoKeyboard::DBESBCSChar,
            Keyboard::DBESChar => EnigoKeyboard::DBESChar,
            Keyboard::Decimal => EnigoKeyboard::Decimal,
            Keyboard::Delete => EnigoKeyboard::Delete,
            Keyboard::Divide => EnigoKeyboard::Divide,
            Keyboard::DownArrow => EnigoKeyboard::DownArrow,
            Keyboard::End => EnigoKeyboard::End,
            Keyboard::Ereof => EnigoKeyboard::Ereof,
            Keyboard::Escape => EnigoKeyboard::Escape,
            Keyboard::Execute => EnigoKeyboard::Execute,
            Keyboard::Exsel => EnigoKeyboard::Exsel,
            Keyboard::F1 => EnigoKeyboard::F1,
            Keyboard::F2 => EnigoKeyboard::F2,
            Keyboard::F3 => EnigoKeyboard::F3,
            Keyboard::F4 => EnigoKeyboard::F4,
            Keyboard::F5 => EnigoKeyboard::F5,
            Keyboard::F6 => EnigoKeyboard::F6,
            Keyboard::F7 => EnigoKeyboard::F7,
            Keyboard::F8 => EnigoKeyboard::F8,
            Keyboard::F9 => EnigoKeyboard::F9,
            Keyboard::F10 => EnigoKeyboard::F10,
            Keyboard::F11 => EnigoKeyboard::F11,
            Keyboard::F12 => EnigoKeyboard::F12,
            Keyboard::F13 => EnigoKeyboard::F13,
            Keyboard::F14 => EnigoKeyboard::F14,
            Keyboard::F15 => EnigoKeyboard::F15,
            Keyboard::F16 => EnigoKeyboard::F16,
            Keyboard::F17 => EnigoKeyboard::F17,
            Keyboard::F18 => EnigoKeyboard::F18,
            Keyboard::F19 => EnigoKeyboard::F19,
            Keyboard::F20 => EnigoKeyboard::F20,
            Keyboard::F21 => EnigoKeyboard::F21,
            Keyboard::F22 => EnigoKeyboard::F22,
            Keyboard::F23 => EnigoKeyboard::F23,
            Keyboard::F24 => EnigoKeyboard::F24,
            Keyboard::Final => EnigoKeyboard::Final,
            Keyboard::GamepadA => EnigoKeyboard::GamepadA,
            Keyboard::GamepadB => EnigoKeyboard::GamepadB,
            Keyboard::GamepadDPadDown => EnigoKeyboard::GamepadDPadDown,
            Keyboard::GamepadDPadLeft => EnigoKeyboard::GamepadDPadLeft,
            Keyboard::GamepadDPadRight => EnigoKeyboard::GamepadDPadRight,
            Keyboard::GamepadDPadUp => EnigoKeyboard::GamepadDPadUp,
            Keyboard::GamepadLeftShoulder => EnigoKeyboard::GamepadLeftShoulder,
            Keyboard::GamepadLeftThumbstickButton => EnigoKeyboard::GamepadLeftThumbstickButton,
            Keyboard::GamepadLeftThumbstickDown => EnigoKeyboard::GamepadLeftThumbstickDown,
            Keyboard::GamepadLeftThumbstickLeft => EnigoKeyboard::GamepadLeftThumbstickLeft,
            Keyboard::GamepadLeftThumbstickRight => EnigoKeyboard::GamepadLeftThumbstickRight,
            Keyboard::GamepadLeftThumbstickUp => EnigoKeyboard::GamepadLeftThumbstickUp,
            Keyboard::GamepadLeftTrigger => EnigoKeyboard::GamepadLeftTrigger,
            Keyboard::GamepadMenu => EnigoKeyboard::GamepadMenu,
            Keyboard::GamepadRightShoulder => EnigoKeyboard::GamepadRightShoulder,
            Keyboard::GamepadRightThumbstickButton => EnigoKeyboard::GamepadRightThumbstickButton,
            Keyboard::GamepadRightThumbstickDown => EnigoKeyboard::GamepadRightThumbstickDown,
            Keyboard::GamepadRightThumbstickLeft => EnigoKeyboard::GamepadRightThumbstickLeft,
            Keyboard::GamepadRightThumbstickRight => EnigoKeyboard::GamepadRightThumbstickRight,
            Keyboard::GamepadRightThumbstickUp => EnigoKeyboard::GamepadRightThumbstickUp,
            Keyboard::GamepadRightTrigger => EnigoKeyboard::GamepadRightTrigger,
            Keyboard::GamepadView => EnigoKeyboard::GamepadView,
            Keyboard::GamepadX => EnigoKeyboard::GamepadX,
            Keyboard::GamepadY => EnigoKeyboard::GamepadY,
            Keyboard::Hangeul => EnigoKeyboard::Hangeul,
            Keyboard::Hangul => EnigoKeyboard::Hangul,
            Keyboard::Hanja => EnigoKeyboard::Hanja,
            Keyboard::Help => EnigoKeyboard::Help,
            Keyboard::Home => EnigoKeyboard::Home,
            Keyboard::Ico00 => EnigoKeyboard::Ico00,
            Keyboard::IcoClear => EnigoKeyboard::IcoClear,
            Keyboard::IcoHelp => EnigoKeyboard::IcoHelp,
            Keyboard::IMEOff => EnigoKeyboard::IMEOff,
            Keyboard::IMEOn => EnigoKeyboard::IMEOn,
            Keyboard::Insert => EnigoKeyboard::Insert,
            Keyboard::Junja => EnigoKeyboard::Junja,
            Keyboard::Kana => EnigoKeyboard::Kana,
            Keyboard::Kanji => EnigoKeyboard::Kanji,
            Keyboard::LaunchApp1 => EnigoKeyboard::LaunchApp1,
            Keyboard::LaunchApp2 => EnigoKeyboard::LaunchApp2,
            Keyboard::LaunchMail => EnigoKeyboard::LaunchMail,
            Keyboard::LaunchMediaSelect => EnigoKeyboard::LaunchMediaSelect,
            Keyboard::LButton => EnigoKeyboard::LButton,
            Keyboard::LControl => EnigoKeyboard::LControl,
            Keyboard::LeftArrow => EnigoKeyboard::LeftArrow,
            Keyboard::LMenu => EnigoKeyboard::LMenu,
            Keyboard::LShift => EnigoKeyboard::LShift,
            Keyboard::LWin => EnigoKeyboard::LWin,
            Keyboard::MButton => EnigoKeyboard::MButton,
            Keyboard::MediaNextTrack => EnigoKeyboard::MediaNextTrack,
            Keyboard::MediaPlayPause => EnigoKeyboard::MediaPlayPause,
            Keyboard::MediaPrevTrack => EnigoKeyboard::MediaPrevTrack,
            Keyboard::MediaStop => EnigoKeyboard::MediaStop,
            Keyboard::Meta => EnigoKeyboard::Meta,
            Keyboard::ModeChange => EnigoKeyboard::ModeChange,
            Keyboard::Multiply => EnigoKeyboard::Multiply,
            Keyboard::NavigationAccept => EnigoKeyboard::NavigationAccept,
            Keyboard::NavigationCancel => EnigoKeyboard::NavigationCancel,
            Keyboard::NavigationDown => EnigoKeyboard::NavigationDown,
            Keyboard::NavigationLeft => EnigoKeyboard::NavigationLeft,
            Keyboard::NavigationMenu => EnigoKeyboard::NavigationMenu,
            Keyboard::NavigationRight => EnigoKeyboard::NavigationRight,
            Keyboard::NavigationUp => EnigoKeyboard::NavigationUp,
            Keyboard::NavigationView => EnigoKeyboard::NavigationView,
            Keyboard::NoName => EnigoKeyboard::NoName,
            Keyboard::NonConvert => EnigoKeyboard::NonConvert,
            Keyboard::None => EnigoKeyboard::None,
            Keyboard::Numlock => EnigoKeyboard::Numlock,
            Keyboard::Numpad0 => EnigoKeyboard::Numpad0,
            Keyboard::Numpad1 => EnigoKeyboard::Numpad1,
            Keyboard::Numpad2 => EnigoKeyboard::Numpad2,
            Keyboard::Numpad3 => EnigoKeyboard::Numpad3,
            Keyboard::Numpad4 => EnigoKeyboard::Numpad4,
            Keyboard::Numpad5 => EnigoKeyboard::Numpad5,
            Keyboard::Numpad6 => EnigoKeyboard::Numpad6,
            Keyboard::Numpad7 => EnigoKeyboard::Numpad7,
            Keyboard::Numpad8 => EnigoKeyboard::Numpad8,
            Keyboard::Numpad9 => EnigoKeyboard::Numpad9,
            Keyboard::OEM1 => EnigoKeyboard::OEM1,
            Keyboard::OEM102 => EnigoKeyboard::OEM102,
            Keyboard::OEM2 => EnigoKeyboard::OEM2,
            Keyboard::OEM3 => EnigoKeyboard::OEM3,
            Keyboard::OEM4 => EnigoKeyboard::OEM4,
            Keyboard::OEM5 => EnigoKeyboard::OEM5,
            Keyboard::OEM6 => EnigoKeyboard::OEM6,
            Keyboard::OEM7 => EnigoKeyboard::OEM7,
            Keyboard::OEM8 => EnigoKeyboard::OEM8,
            Keyboard::OEMAttn => EnigoKeyboard::OEMAttn,
            Keyboard::OEMAuto => EnigoKeyboard::OEMAuto,
            Keyboard::OEMAx => EnigoKeyboard::OEMAx,
            Keyboard::OEMBacktab => EnigoKeyboard::OEMBacktab,
            Keyboard::OEMClear => EnigoKeyboard::OEMClear,
            Keyboard::OEMComma => EnigoKeyboard::OEMComma,
            Keyboard::OEMCopy => EnigoKeyboard::OEMCopy,
            Keyboard::OEMCusel => EnigoKeyboard::OEMCusel,
            Keyboard::OEMEnlw => EnigoKeyboard::OEMEnlw,
            Keyboard::OEMFinish => EnigoKeyboard::OEMFinish,
            Keyboard::OEMFJJisho => EnigoKeyboard::OEMFJJisho,
            Keyboard::OEMFJLoya => EnigoKeyboard::OEMFJLoya,
            Keyboard::OEMFJMasshou => EnigoKeyboard::OEMFJMasshou,
            Keyboard::OEMFJRoya => EnigoKeyboard::OEMFJRoya,
            Keyboard::OEMFJTouroku => EnigoKeyboard::OEMFJTouroku,
            Keyboard::OEMJump => EnigoKeyboard::OEMJump,
            Keyboard::OEMMinus => EnigoKeyboard::OEMMinus,
            Keyboard::OEMNECEqual => EnigoKeyboard::OEMNECEqual,
            Keyboard::OEMPA1 => EnigoKeyboard::OEMPA1,
            Keyboard::OEMPA2 => EnigoKeyboard::OEMPA2,
            Keyboard::OEMPA3 => EnigoKeyboard::OEMPA3,
            Keyboard::OEMPeriod => EnigoKeyboard::OEMPeriod,
            Keyboard::OEMPlus => EnigoKeyboard::OEMPlus,
            Keyboard::OEMReset => EnigoKeyboard::OEMReset,
            Keyboard::OEMWsctrl => EnigoKeyboard::OEMWsctrl,
            Keyboard::Option => EnigoKeyboard::Option,
            Keyboard::PA1 => EnigoKeyboard::PA1,
            Keyboard::Packet => EnigoKeyboard::Packet,
            Keyboard::PageDown => EnigoKeyboard::PageDown,
            Keyboard::PageUp => EnigoKeyboard::PageUp,
            Keyboard::Pause => EnigoKeyboard::Pause,
            Keyboard::Play => EnigoKeyboard::Play,
            Keyboard::Print => EnigoKeyboard::Print,
            Keyboard::Processkey => EnigoKeyboard::Processkey,
            Keyboard::RButton => EnigoKeyboard::RButton,
            Keyboard::RControl => EnigoKeyboard::RControl,
            Keyboard::Return => EnigoKeyboard::Return,
            Keyboard::RightArrow => EnigoKeyboard::RightArrow,
            Keyboard::RMenu => EnigoKeyboard::RMenu,
            Keyboard::RShift => EnigoKeyboard::RShift,
            Keyboard::RWin => EnigoKeyboard::RWin,
            Keyboard::Scroll => EnigoKeyboard::Scroll,
            Keyboard::Select => EnigoKeyboard::Select,
            Keyboard::Separator => EnigoKeyboard::Separator,
            Keyboard::Shift => EnigoKeyboard::Shift,
            Keyboard::Sleep => EnigoKeyboard::Sleep,
            Keyboard::Snapshot => EnigoKeyboard::Snapshot,
            Keyboard::Space => EnigoKeyboard::Space,
            Keyboard::Subtract => EnigoKeyboard::Subtract,
            Keyboard::Tab => EnigoKeyboard::Tab,
            Keyboard::UpArrow => EnigoKeyboard::UpArrow,
            Keyboard::VolumeDown => EnigoKeyboard::VolumeDown,
            Keyboard::VolumeMute => EnigoKeyboard::VolumeMute,
            Keyboard::VolumeUp => EnigoKeyboard::VolumeUp,
            Keyboard::XButton1 => EnigoKeyboard::XButton1,
            Keyboard::XButton2 => EnigoKeyboard::XButton2,
            Keyboard::Zoom => EnigoKeyboard::Zoom,
            Keyboard::Layout(char) => EnigoKeyboard::Layout(*char),
            Keyboard::Raw(code) => EnigoKeyboard::Raw(*code),
        }
    }
}

impl EmulateAbsoluteValue for Keyboard {
    type Value = bool;

    fn change_to(&self, to: Self::Value) -> &Self {
        if to {
            Enigo.key_down(self.as_enigo());
        } else {
            Enigo.key_up(self.as_enigo());
        }
        self
    }
}

crate_impls! { Keyboard }

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Back,
    Forward,
}

impl MouseButton {
    fn as_enigo(&self) -> EnigoMouseButton {
        match self {
            MouseButton::Left => EnigoMouseButton::Left,
            MouseButton::Middle => EnigoMouseButton::Middle,
            MouseButton::Right => EnigoMouseButton::Right,
            MouseButton::Back => EnigoMouseButton::Back,
            MouseButton::Forward => EnigoMouseButton::Forward,
        }
    }
}

crate_impls! { MouseButton }

impl EmulateAbsoluteValue for MouseButton {
    type Value = bool;

    fn change_to(&self, to: Self::Value) -> &Self {
        if to {
            Enigo.mouse_down(self.as_enigo());
        } else {
            Enigo.mouse_up(self.as_enigo());
        }
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MousePosition;

impl MousePosition {
    pub fn get() -> (i32, i32) {
        Enigo.mouse_location()
    }
}

crate_impls! { MousePosition }

impl EmulateAbsoluteValue for MousePosition {
    type Value = (i32, i32);

    fn change_to(&self, by: Self::Value) -> &Self {
        Enigo.mouse_move_to(by.0, by.1);
        self
    }
}

impl EmulateRelativeValue for MousePosition {
    type Value = (i32, i32);

    fn change_by(&self, by: Self::Value) -> &Self {
        Enigo.mouse_move_relative(by.0, by.1);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MouseScroll;

crate_impls! { MouseScroll }

impl EmulateRelativeValue for MouseScroll {
    type Value = (i32, i32);

    fn change_by(&self, by: Self::Value) -> &Self {
        Enigo.mouse_scroll_x(by.0);
        Enigo.mouse_scroll_y(by.1);
        self
    }
}
