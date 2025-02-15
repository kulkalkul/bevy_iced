use bevy_input::prelude::KeyCode as BevyKeyCode;
use bevy_input::prelude::MouseButton;
use iced_native::keyboard::KeyCode as IcedKeyCode;

pub fn key_code(virtual_keycode: BevyKeyCode) -> IcedKeyCode {
    match virtual_keycode {
        BevyKeyCode::Key1 => IcedKeyCode::Key1,
        BevyKeyCode::Key2 => IcedKeyCode::Key2,
        BevyKeyCode::Key3 => IcedKeyCode::Key3,
        BevyKeyCode::Key4 => IcedKeyCode::Key4,
        BevyKeyCode::Key5 => IcedKeyCode::Key5,
        BevyKeyCode::Key6 => IcedKeyCode::Key6,
        BevyKeyCode::Key7 => IcedKeyCode::Key7,
        BevyKeyCode::Key8 => IcedKeyCode::Key8,
        BevyKeyCode::Key9 => IcedKeyCode::Key9,
        BevyKeyCode::Key0 => IcedKeyCode::Key0,
        BevyKeyCode::A => IcedKeyCode::A,
        BevyKeyCode::B => IcedKeyCode::B,
        BevyKeyCode::C => IcedKeyCode::C,
        BevyKeyCode::D => IcedKeyCode::D,
        BevyKeyCode::E => IcedKeyCode::E,
        BevyKeyCode::F => IcedKeyCode::F,
        BevyKeyCode::G => IcedKeyCode::G,
        BevyKeyCode::H => IcedKeyCode::H,
        BevyKeyCode::I => IcedKeyCode::I,
        BevyKeyCode::J => IcedKeyCode::J,
        BevyKeyCode::K => IcedKeyCode::K,
        BevyKeyCode::L => IcedKeyCode::L,
        BevyKeyCode::M => IcedKeyCode::M,
        BevyKeyCode::N => IcedKeyCode::N,
        BevyKeyCode::O => IcedKeyCode::O,
        BevyKeyCode::P => IcedKeyCode::P,
        BevyKeyCode::Q => IcedKeyCode::Q,
        BevyKeyCode::R => IcedKeyCode::R,
        BevyKeyCode::S => IcedKeyCode::S,
        BevyKeyCode::T => IcedKeyCode::T,
        BevyKeyCode::U => IcedKeyCode::U,
        BevyKeyCode::V => IcedKeyCode::V,
        BevyKeyCode::W => IcedKeyCode::W,
        BevyKeyCode::X => IcedKeyCode::X,
        BevyKeyCode::Y => IcedKeyCode::Y,
        BevyKeyCode::Z => IcedKeyCode::Z,
        BevyKeyCode::Escape => IcedKeyCode::Escape,
        BevyKeyCode::F1 => IcedKeyCode::F1,
        BevyKeyCode::F2 => IcedKeyCode::F2,
        BevyKeyCode::F3 => IcedKeyCode::F3,
        BevyKeyCode::F4 => IcedKeyCode::F4,
        BevyKeyCode::F5 => IcedKeyCode::F5,
        BevyKeyCode::F6 => IcedKeyCode::F6,
        BevyKeyCode::F7 => IcedKeyCode::F7,
        BevyKeyCode::F8 => IcedKeyCode::F8,
        BevyKeyCode::F9 => IcedKeyCode::F9,
        BevyKeyCode::F10 => IcedKeyCode::F10,
        BevyKeyCode::F11 => IcedKeyCode::F11,
        BevyKeyCode::F12 => IcedKeyCode::F12,
        BevyKeyCode::F13 => IcedKeyCode::F13,
        BevyKeyCode::F14 => IcedKeyCode::F14,
        BevyKeyCode::F15 => IcedKeyCode::F15,
        BevyKeyCode::F16 => IcedKeyCode::F16,
        BevyKeyCode::F17 => IcedKeyCode::F17,
        BevyKeyCode::F18 => IcedKeyCode::F18,
        BevyKeyCode::F19 => IcedKeyCode::F19,
        BevyKeyCode::F20 => IcedKeyCode::F20,
        BevyKeyCode::F21 => IcedKeyCode::F21,
        BevyKeyCode::F22 => IcedKeyCode::F22,
        BevyKeyCode::F23 => IcedKeyCode::F23,
        BevyKeyCode::F24 => IcedKeyCode::F24,
        BevyKeyCode::Snapshot => IcedKeyCode::Snapshot,
        BevyKeyCode::Scroll => IcedKeyCode::Scroll,
        BevyKeyCode::Pause => IcedKeyCode::Pause,
        BevyKeyCode::Insert => IcedKeyCode::Insert,
        BevyKeyCode::Home => IcedKeyCode::Home,
        BevyKeyCode::Delete => IcedKeyCode::Delete,
        BevyKeyCode::End => IcedKeyCode::End,
        BevyKeyCode::PageDown => IcedKeyCode::PageDown,
        BevyKeyCode::PageUp => IcedKeyCode::PageUp,
        BevyKeyCode::Left => IcedKeyCode::Left,
        BevyKeyCode::Up => IcedKeyCode::Up,
        BevyKeyCode::Right => IcedKeyCode::Right,
        BevyKeyCode::Down => IcedKeyCode::Down,
        BevyKeyCode::Back => IcedKeyCode::Backspace,
        BevyKeyCode::Return => IcedKeyCode::Enter,
        BevyKeyCode::Space => IcedKeyCode::Space,
        BevyKeyCode::Compose => IcedKeyCode::Compose,
        BevyKeyCode::Caret => IcedKeyCode::Caret,
        BevyKeyCode::Numlock => IcedKeyCode::Numlock,
        BevyKeyCode::Numpad0 => IcedKeyCode::Numpad0,
        BevyKeyCode::Numpad1 => IcedKeyCode::Numpad1,
        BevyKeyCode::Numpad2 => IcedKeyCode::Numpad2,
        BevyKeyCode::Numpad3 => IcedKeyCode::Numpad3,
        BevyKeyCode::Numpad4 => IcedKeyCode::Numpad4,
        BevyKeyCode::Numpad5 => IcedKeyCode::Numpad5,
        BevyKeyCode::Numpad6 => IcedKeyCode::Numpad6,
        BevyKeyCode::Numpad7 => IcedKeyCode::Numpad7,
        BevyKeyCode::Numpad8 => IcedKeyCode::Numpad8,
        BevyKeyCode::Numpad9 => IcedKeyCode::Numpad9,
        BevyKeyCode::AbntC1 => IcedKeyCode::AbntC1,
        BevyKeyCode::AbntC2 => IcedKeyCode::AbntC2,
        BevyKeyCode::NumpadAdd => IcedKeyCode::NumpadAdd,
        BevyKeyCode::Plus => IcedKeyCode::Plus,
        BevyKeyCode::Apostrophe => IcedKeyCode::Apostrophe,
        BevyKeyCode::Apps => IcedKeyCode::Apps,
        BevyKeyCode::At => IcedKeyCode::At,
        BevyKeyCode::Ax => IcedKeyCode::Ax,
        BevyKeyCode::Backslash => IcedKeyCode::Backslash,
        BevyKeyCode::Calculator => IcedKeyCode::Calculator,
        BevyKeyCode::Capital => IcedKeyCode::Capital,
        BevyKeyCode::Colon => IcedKeyCode::Colon,
        BevyKeyCode::Comma => IcedKeyCode::Comma,
        BevyKeyCode::Convert => IcedKeyCode::Convert,
        BevyKeyCode::NumpadDecimal => IcedKeyCode::NumpadDecimal,
        BevyKeyCode::NumpadDivide => IcedKeyCode::NumpadDivide,
        BevyKeyCode::Equals => IcedKeyCode::Equals,
        BevyKeyCode::Grave => IcedKeyCode::Grave,
        BevyKeyCode::Kana => IcedKeyCode::Kana,
        BevyKeyCode::Kanji => IcedKeyCode::Kanji,
        BevyKeyCode::LAlt => IcedKeyCode::LAlt,
        BevyKeyCode::LBracket => IcedKeyCode::LBracket,
        BevyKeyCode::LControl => IcedKeyCode::LControl,
        BevyKeyCode::LShift => IcedKeyCode::LShift,
        BevyKeyCode::LWin => IcedKeyCode::LWin,
        BevyKeyCode::Mail => IcedKeyCode::Mail,
        BevyKeyCode::MediaSelect => IcedKeyCode::MediaSelect,
        BevyKeyCode::MediaStop => IcedKeyCode::MediaStop,
        BevyKeyCode::Minus => IcedKeyCode::Minus,
        BevyKeyCode::NumpadMultiply => IcedKeyCode::NumpadMultiply,
        BevyKeyCode::Mute => IcedKeyCode::Mute,
        BevyKeyCode::MyComputer => IcedKeyCode::MyComputer,
        BevyKeyCode::NavigateForward => IcedKeyCode::NavigateForward,
        BevyKeyCode::NavigateBackward => IcedKeyCode::NavigateBackward,
        BevyKeyCode::NextTrack => IcedKeyCode::NextTrack,
        BevyKeyCode::NoConvert => IcedKeyCode::NoConvert,
        BevyKeyCode::NumpadComma => IcedKeyCode::NumpadComma,
        BevyKeyCode::NumpadEnter => IcedKeyCode::NumpadEnter,
        BevyKeyCode::NumpadEquals => IcedKeyCode::NumpadEquals,
        BevyKeyCode::Oem102 => IcedKeyCode::OEM102,
        BevyKeyCode::Period => IcedKeyCode::Period,
        BevyKeyCode::PlayPause => IcedKeyCode::PlayPause,
        BevyKeyCode::Power => IcedKeyCode::Power,
        BevyKeyCode::PrevTrack => IcedKeyCode::PrevTrack,
        BevyKeyCode::RAlt => IcedKeyCode::RAlt,
        BevyKeyCode::RBracket => IcedKeyCode::RBracket,
        BevyKeyCode::RControl => IcedKeyCode::RControl,
        BevyKeyCode::RShift => IcedKeyCode::RShift,
        BevyKeyCode::RWin => IcedKeyCode::RWin,
        BevyKeyCode::Semicolon => IcedKeyCode::Semicolon,
        BevyKeyCode::Slash => IcedKeyCode::Slash,
        BevyKeyCode::Sleep => IcedKeyCode::Sleep,
        BevyKeyCode::Stop => IcedKeyCode::Stop,
        BevyKeyCode::NumpadSubtract => IcedKeyCode::NumpadSubtract,
        BevyKeyCode::Sysrq => IcedKeyCode::Sysrq,
        BevyKeyCode::Tab => IcedKeyCode::Tab,
        BevyKeyCode::Underline => IcedKeyCode::Underline,
        BevyKeyCode::Unlabeled => IcedKeyCode::Unlabeled,
        BevyKeyCode::VolumeDown => IcedKeyCode::VolumeDown,
        BevyKeyCode::VolumeUp => IcedKeyCode::VolumeUp,
        BevyKeyCode::Wake => IcedKeyCode::Wake,
        BevyKeyCode::WebBack => IcedKeyCode::WebBack,
        BevyKeyCode::WebFavorites => IcedKeyCode::WebFavorites,
        BevyKeyCode::WebForward => IcedKeyCode::WebForward,
        BevyKeyCode::WebHome => IcedKeyCode::WebHome,
        BevyKeyCode::WebRefresh => IcedKeyCode::WebRefresh,
        BevyKeyCode::WebSearch => IcedKeyCode::WebSearch,
        BevyKeyCode::WebStop => IcedKeyCode::WebStop,
        BevyKeyCode::Yen => IcedKeyCode::Yen,
        BevyKeyCode::Copy => IcedKeyCode::Copy,
        BevyKeyCode::Paste => IcedKeyCode::Paste,
        BevyKeyCode::Cut => IcedKeyCode::Cut,
        BevyKeyCode::Asterisk => IcedKeyCode::Asterisk,
    }
}

pub fn mouse_button(button: MouseButton) -> iced_native::mouse::Button {
    use iced_native::mouse::Button;
    match button {
        MouseButton::Left => Button::Left,
        MouseButton::Right => Button::Right,
        MouseButton::Middle => Button::Middle,
        MouseButton::Other(val) => Button::Other(val as u8),
    }
}
