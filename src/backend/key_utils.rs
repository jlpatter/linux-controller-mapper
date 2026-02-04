use enigo::Key::Unicode;
use enigo::{Button as MouseButton, Key};
use iced::keyboard::Key as IcedKey;
use iced::keyboard::Key::Character;
use iced::keyboard::key::Named;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub fn get_enigo_key_from_iced_key(key: IcedKey) -> Option<Key> {
    if let Character(c) = key {
        Some(Unicode(c.chars().next()?))
    } else if let IcedKey::Named(named) = key {
        let named_chars_map: HashMap<Named, Key> = HashMap::from([
            (Named::Control, Key::Control),
            (Named::Alt, Key::Alt),
            (Named::Shift, Key::Shift),
            (Named::Tab, Key::Tab),
            (Named::Escape, Key::Escape),
            (Named::Meta, Key::Meta),
            (Named::Backspace, Key::Backspace),
        ]);

        Some(*named_chars_map.get(&named)?)
    } else {
        None
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MouseButtonOrKey {
    MouseButton(MouseButton),
    Key(Key),
}

impl Display for MouseButtonOrKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MouseButtonOrKey::MouseButton(mb) => write!(f, "{:?} (Mouse Button)", mb),
            MouseButtonOrKey::Key(k) => {
                if let Unicode(u) = k {
                    write!(f, "{}", u.to_string().to_uppercase())
                } else {
                    write!(f, "{:?}", k)
                }
            }
        }
    }
}
