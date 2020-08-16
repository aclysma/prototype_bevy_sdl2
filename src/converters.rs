use bevy_input::{
    keyboard::{ElementState, KeyCode, KeyboardInput},
    mouse::MouseButton,
};

pub fn convert_keyboard_input(
    keycode: sdl2::keyboard::Keycode,
    scancode: sdl2::keyboard::Scancode,
    element_state: ElementState,
) -> KeyboardInput {
    KeyboardInput {
        scan_code: scancode as u32,
        key_code: convert_virtual_key_code(keycode),
        state: element_state,
    }
}

pub fn convert_mouse_button(mouse_button: sdl2::mouse::MouseButton) -> MouseButton {
    match mouse_button {
        sdl2::mouse::MouseButton::Left => MouseButton::Left,
        sdl2::mouse::MouseButton::Right => MouseButton::Right,
        sdl2::mouse::MouseButton::Middle => MouseButton::Middle,
        sdl2::mouse::MouseButton::X1 => MouseButton::Other(0),
        sdl2::mouse::MouseButton::X2 => MouseButton::Other(1),
        sdl2::mouse::MouseButton::Unknown => MouseButton::Other(2),
    }
}

pub fn convert_virtual_key_code(keycode: sdl2::keyboard::Keycode) -> Option<KeyCode> {
    // I tried...
    match keycode {
        sdl2::keyboard::Keycode::Num1 => Some(KeyCode::Key1),
        sdl2::keyboard::Keycode::Num2 => Some(KeyCode::Key2),
        sdl2::keyboard::Keycode::Num3 => Some(KeyCode::Key3),
        sdl2::keyboard::Keycode::Num4 => Some(KeyCode::Key4),
        sdl2::keyboard::Keycode::Num5 => Some(KeyCode::Key5),
        sdl2::keyboard::Keycode::Num6 => Some(KeyCode::Key6),
        sdl2::keyboard::Keycode::Num7 => Some(KeyCode::Key7),
        sdl2::keyboard::Keycode::Num8 => Some(KeyCode::Key8),
        sdl2::keyboard::Keycode::Num9 => Some(KeyCode::Key9),
        sdl2::keyboard::Keycode::Num0 => Some(KeyCode::Key0),
        sdl2::keyboard::Keycode::A => Some(KeyCode::A),
        sdl2::keyboard::Keycode::B => Some(KeyCode::B),
        sdl2::keyboard::Keycode::C => Some(KeyCode::C),
        sdl2::keyboard::Keycode::D => Some(KeyCode::D),
        sdl2::keyboard::Keycode::E => Some(KeyCode::E),
        sdl2::keyboard::Keycode::F => Some(KeyCode::F),
        sdl2::keyboard::Keycode::G => Some(KeyCode::G),
        sdl2::keyboard::Keycode::H => Some(KeyCode::H),
        sdl2::keyboard::Keycode::I => Some(KeyCode::I),
        sdl2::keyboard::Keycode::J => Some(KeyCode::J),
        sdl2::keyboard::Keycode::K => Some(KeyCode::K),
        sdl2::keyboard::Keycode::L => Some(KeyCode::L),
        sdl2::keyboard::Keycode::M => Some(KeyCode::M),
        sdl2::keyboard::Keycode::N => Some(KeyCode::N),
        sdl2::keyboard::Keycode::O => Some(KeyCode::O),
        sdl2::keyboard::Keycode::P => Some(KeyCode::P),
        sdl2::keyboard::Keycode::Q => Some(KeyCode::Q),
        sdl2::keyboard::Keycode::R => Some(KeyCode::R),
        sdl2::keyboard::Keycode::S => Some(KeyCode::S),
        sdl2::keyboard::Keycode::T => Some(KeyCode::T),
        sdl2::keyboard::Keycode::U => Some(KeyCode::U),
        sdl2::keyboard::Keycode::V => Some(KeyCode::V),
        sdl2::keyboard::Keycode::W => Some(KeyCode::W),
        sdl2::keyboard::Keycode::X => Some(KeyCode::X),
        sdl2::keyboard::Keycode::Y => Some(KeyCode::Y),
        sdl2::keyboard::Keycode::Z => Some(KeyCode::Z),
        sdl2::keyboard::Keycode::Escape => Some(KeyCode::Escape),
        sdl2::keyboard::Keycode::F1 => Some(KeyCode::F1),
        sdl2::keyboard::Keycode::F2 => Some(KeyCode::F2),
        sdl2::keyboard::Keycode::F3 => Some(KeyCode::F3),
        sdl2::keyboard::Keycode::F4 => Some(KeyCode::F4),
        sdl2::keyboard::Keycode::F5 => Some(KeyCode::F5),
        sdl2::keyboard::Keycode::F6 => Some(KeyCode::F6),
        sdl2::keyboard::Keycode::F7 => Some(KeyCode::F7),
        sdl2::keyboard::Keycode::F8 => Some(KeyCode::F8),
        sdl2::keyboard::Keycode::F9 => Some(KeyCode::F9),
        sdl2::keyboard::Keycode::F10 => Some(KeyCode::F10),
        sdl2::keyboard::Keycode::F11 => Some(KeyCode::F11),
        sdl2::keyboard::Keycode::F12 => Some(KeyCode::F12),
        sdl2::keyboard::Keycode::F13 => Some(KeyCode::F13),
        sdl2::keyboard::Keycode::F14 => Some(KeyCode::F14),
        sdl2::keyboard::Keycode::F15 => Some(KeyCode::F15),
        sdl2::keyboard::Keycode::F16 => Some(KeyCode::F16),
        sdl2::keyboard::Keycode::F17 => Some(KeyCode::F17),
        sdl2::keyboard::Keycode::F18 => Some(KeyCode::F18),
        sdl2::keyboard::Keycode::F19 => Some(KeyCode::F19),
        sdl2::keyboard::Keycode::F20 => Some(KeyCode::F20),
        sdl2::keyboard::Keycode::F21 => Some(KeyCode::F21),
        sdl2::keyboard::Keycode::F22 => Some(KeyCode::F22),
        sdl2::keyboard::Keycode::F23 => Some(KeyCode::F23),
        sdl2::keyboard::Keycode::F24 => Some(KeyCode::F24),
        sdl2::keyboard::Keycode::PrintScreen => Some(KeyCode::Snapshot),
        sdl2::keyboard::Keycode::ScrollLock => Some(KeyCode::Scroll),
        sdl2::keyboard::Keycode::Pause => Some(KeyCode::Pause),
        sdl2::keyboard::Keycode::Insert => Some(KeyCode::Insert),
        sdl2::keyboard::Keycode::Home => Some(KeyCode::Home),
        sdl2::keyboard::Keycode::Delete => Some(KeyCode::Delete),
        sdl2::keyboard::Keycode::End => Some(KeyCode::End),
        sdl2::keyboard::Keycode::PageDown => Some(KeyCode::PageDown),
        sdl2::keyboard::Keycode::PageUp => Some(KeyCode::PageUp),
        sdl2::keyboard::Keycode::Left => Some(KeyCode::Left),
        sdl2::keyboard::Keycode::Up => Some(KeyCode::Up),
        sdl2::keyboard::Keycode::Right => Some(KeyCode::Right),
        sdl2::keyboard::Keycode::Down => Some(KeyCode::Down),
        sdl2::keyboard::Keycode::Backspace => Some(KeyCode::Back),
        sdl2::keyboard::Keycode::Return => Some(KeyCode::Return),
        sdl2::keyboard::Keycode::Space => Some(KeyCode::Space),
        //sdl2::keyboard::Keycode::Application => Some(KeyCode::Compose),
        sdl2::keyboard::Keycode::Caret => Some(KeyCode::Caret),
        sdl2::keyboard::Keycode::NumLockClear => Some(KeyCode::Numlock),
        sdl2::keyboard::Keycode::Kp0 => Some(KeyCode::Numpad0),
        sdl2::keyboard::Keycode::Kp1 => Some(KeyCode::Numpad1),
        sdl2::keyboard::Keycode::Kp2 => Some(KeyCode::Numpad2),
        sdl2::keyboard::Keycode::Kp3 => Some(KeyCode::Numpad3),
        sdl2::keyboard::Keycode::Kp4 => Some(KeyCode::Numpad4),
        sdl2::keyboard::Keycode::Kp5 => Some(KeyCode::Numpad5),
        sdl2::keyboard::Keycode::Kp6 => Some(KeyCode::Numpad6),
        sdl2::keyboard::Keycode::Kp7 => Some(KeyCode::Numpad7),
        sdl2::keyboard::Keycode::Kp8 => Some(KeyCode::Numpad8),
        sdl2::keyboard::Keycode::Kp9 => Some(KeyCode::Numpad9),
        //sdl2::keyboard::Keycode::AbntC1 => KeyCode::AbntC1,
        //sdl2::keyboard::Keycode::AbntC2 => KeyCode::AbntC2,
        sdl2::keyboard::Keycode::KpMemAdd => Some(KeyCode::Add),
        sdl2::keyboard::Keycode::Quote => Some(KeyCode::Apostrophe),
        sdl2::keyboard::Keycode::Application => Some(KeyCode::Apps),
        sdl2::keyboard::Keycode::At => Some(KeyCode::At),
        //sdl2::keyboard::Keycode::Ax => KeyCode::Ax,
        sdl2::keyboard::Keycode::Backslash => Some(KeyCode::Backslash),
        sdl2::keyboard::Keycode::Calculator => Some(KeyCode::Calculator),
        sdl2::keyboard::Keycode::CapsLock => Some(KeyCode::Capital),
        sdl2::keyboard::Keycode::Colon => Some(KeyCode::Colon),
        sdl2::keyboard::Keycode::Comma => Some(KeyCode::Comma),
        //sdl2::keyboard::Keycode::Convert => KeyCode::Convert,
        sdl2::keyboard::Keycode::KpPeriod => Some(KeyCode::Decimal),
        sdl2::keyboard::Keycode::KpDivide => Some(KeyCode::Divide),
        sdl2::keyboard::Keycode::Equals => Some(KeyCode::Equals),
        sdl2::keyboard::Keycode::Backquote => Some(KeyCode::Grave),
        //sdl2::keyboard::Keycode::Kana => KeyCode::Kana,
        //sdl2::keyboard::Keycode::Kanji => KeyCode::Kanji,
        sdl2::keyboard::Keycode::LAlt => Some(KeyCode::LAlt),
        sdl2::keyboard::Keycode::LeftBracket => Some(KeyCode::LBracket),
        sdl2::keyboard::Keycode::LCtrl => Some(KeyCode::LControl),
        sdl2::keyboard::Keycode::LShift => Some(KeyCode::LShift),
        sdl2::keyboard::Keycode::LGui => Some(KeyCode::LWin),
        sdl2::keyboard::Keycode::Mail => Some(KeyCode::Mail),
        sdl2::keyboard::Keycode::MediaSelect => Some(KeyCode::MediaSelect),
        sdl2::keyboard::Keycode::AudioStop => Some(KeyCode::MediaStop),
        sdl2::keyboard::Keycode::Minus => Some(KeyCode::Minus),
        sdl2::keyboard::Keycode::KpMultiply => Some(KeyCode::Multiply),
        sdl2::keyboard::Keycode::Mute => Some(KeyCode::Mute),
        sdl2::keyboard::Keycode::Computer => Some(KeyCode::MyComputer),
        sdl2::keyboard::Keycode::AcForward => Some(KeyCode::NavigateForward),
        sdl2::keyboard::Keycode::AcBack => Some(KeyCode::NavigateBackward),
        sdl2::keyboard::Keycode::AudioNext => Some(KeyCode::NextTrack),
        //sdl2::keyboard::Keycode::NoConvert => KeyCode::NoConvert,
        sdl2::keyboard::Keycode::KpComma => Some(KeyCode::NumpadComma),
        sdl2::keyboard::Keycode::KpEnter => Some(KeyCode::NumpadEnter),
        sdl2::keyboard::Keycode::KpEquals => Some(KeyCode::NumpadEquals),
        //sdl2::keyboard::Keycode::OEM102 => KeyCode::OEM102,
        sdl2::keyboard::Keycode::Period => Some(KeyCode::Period),
        sdl2::keyboard::Keycode::AudioPlay => Some(KeyCode::PlayPause),
        sdl2::keyboard::Keycode::Power => Some(KeyCode::Power),
        sdl2::keyboard::Keycode::AudioPrev => Some(KeyCode::PrevTrack),
        sdl2::keyboard::Keycode::RAlt => Some(KeyCode::RAlt),
        sdl2::keyboard::Keycode::RightBracket => Some(KeyCode::RBracket),
        sdl2::keyboard::Keycode::RCtrl => Some(KeyCode::RControl),
        sdl2::keyboard::Keycode::RShift => Some(KeyCode::RShift),
        sdl2::keyboard::Keycode::RGui => Some(KeyCode::RWin),
        sdl2::keyboard::Keycode::Semicolon => Some(KeyCode::Semicolon),
        sdl2::keyboard::Keycode::Slash => Some(KeyCode::Slash),
        sdl2::keyboard::Keycode::Sleep => Some(KeyCode::Sleep),
        sdl2::keyboard::Keycode::Stop => Some(KeyCode::Stop),
        sdl2::keyboard::Keycode::KpMinus => Some(KeyCode::Subtract),
        sdl2::keyboard::Keycode::Sysreq => Some(KeyCode::Sysrq),
        sdl2::keyboard::Keycode::Tab => Some(KeyCode::Tab),
        sdl2::keyboard::Keycode::Underscore => Some(KeyCode::Underline),
        //sdl2::keyboard::Keycode::Unlabeled => KeyCode::Unlabeled,
        sdl2::keyboard::Keycode::VolumeDown => Some(KeyCode::VolumeDown),
        sdl2::keyboard::Keycode::VolumeUp => Some(KeyCode::VolumeUp),
        //sdl2::keyboard::Keycode::Wake => KeyCode::Wake,
        //sdl2::keyboard::Keycode::AcBack => Some(KeyCode::WebBack),
        sdl2::keyboard::Keycode::AcBookmarks => Some(KeyCode::WebFavorites),
        //sdl2::keyboard::Keycode::AcForward => Some(KeyCode::WebForward),
        sdl2::keyboard::Keycode::AcHome => Some(KeyCode::WebHome),
        sdl2::keyboard::Keycode::AcRefresh => Some(KeyCode::WebRefresh),
        sdl2::keyboard::Keycode::AcSearch => Some(KeyCode::WebSearch),
        sdl2::keyboard::Keycode::AcStop => Some(KeyCode::WebStop),
        //sdl2::keyboard::Keycode::Yen => KeyCode::Yen,
        sdl2::keyboard::Keycode::Copy => Some(KeyCode::Copy),
        sdl2::keyboard::Keycode::Paste => Some(KeyCode::Paste),
        sdl2::keyboard::Keycode::Cut => Some(KeyCode::Cut),
        _ => None,
    }
}
