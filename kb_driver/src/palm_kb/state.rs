use heapless::Vec;
use usbd_hid::descriptor::KeyboardReport;

use crate::{debug, error, key_codes::{KeyCode, Modifiers}, warn};

#[derive(Default, PartialEq, Eq)]
pub struct State {
    last_key_up: Option<KeyCode>,
    keycodes: Vec<KeyCode, 6>,
    modifiers: Modifiers,
    fn_triggered: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum InputType {
    KeyUp,
    KeyDown
}

impl State {
    pub const fn new() -> Self {
        Self {
            last_key_up: None,
            keycodes: Vec::new(),
            modifiers: Modifiers::empty(),
            fn_triggered: false
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Uses values received directly from the UART line to update the state
    pub fn update_from_kb_input(&mut self, input: u8) {
        if let Some(key) = KeyCode::try_from_matrix_key(input) {
            let input_type = InputType::from(input);
            debug!("received key {:?} with input type {:?}", key, input_type);

            match input_type {
                InputType::KeyUp => {
                    if Some(key) == self.last_key_up {
                        self.keycodes.truncate(0);
                        self.modifiers = Modifiers::empty();
                    } else {
                        self.keycodes.retain(|k| *k != key);
                        self.modifiers = self.modifiers.difference(Modifiers::from(key));
                    }
                    self.last_key_up = Some(key);
                },
                InputType::KeyDown => {
                    if !self.keycodes.contains(&key) {
                        match self.keycodes.push(key) {
                            Ok(_) => (),
                            Err(_) => {
                                warn!("tried to push new key code into full keycode vec");
                                self.keycodes.remove(0);
                                let _ = self.keycodes.push(key);
                            },
                        }
                    } else {
                        warn!("tried to insert pressed key that was already pressed")
                    }
                    self.modifiers = self.modifiers.union(Modifiers::from(key));
                    self.last_key_up = None;
                },
            }
        } else {
            error!("received invalid matrix coordinates from device")
        }
    }

    #[inline]
    pub fn raw_keycode_arr(&self) -> [u8; 6] {
        let mut out = [0u8; 6];
        for (i, val) in self.keycodes.iter().enumerate() {
            out[i] = *val as u8;
        }
        out
    }
}

impl From<&State> for KeyboardReport {
    fn from(value: &State) -> Self {
        KeyboardReport {
            modifier: value.modifiers.bits(),
            reserved: 0,
            leds: 0,
            keycodes: value.raw_keycode_arr(),
        }
    }
}

impl From<u8> for InputType {
    /// MUST be used with a value received directly from the UART line
    ///
    /// MSB 1 means KeyUp, MSB 0 means KeyDown
    fn from(value: u8) -> Self {
        if value & (1 << 7) == 0 { Self::KeyDown } else { Self::KeyUp }
    }
}
