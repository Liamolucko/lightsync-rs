#![cfg(windows)]

//! This is a wrapper around Logitech's LED SDK.

mod bindings;
pub mod raw;

use bindings::root::{LogiLed, LOGI_LED_BITMAP_SIZE};
use phf::phf_map;
use std::convert::TryInto;
use std::time::Duration;
pub use LogiLed::DeviceType;
pub use LogiLed::KeyName as Key;

pub const BITMAP_SIZE: i32 = LOGI_LED_BITMAP_SIZE;

/// Types of devices with different kinds of lighting.
pub mod lighting {
    use super::bindings::root::*;

    pub const MONOCHROME: i32 = LOGI_DEVICETYPE_MONOCHROME;
    pub const RGB: i32 = LOGI_DEVICETYPE_RGB;
    pub const PERKEY_RGB: i32 = LOGI_DEVICETYPE_PERKEY_RGB;
    pub const ALL: i32 = LOGI_DEVICETYPE_ALL;
}

static ASCII_MAP: phf::Map<char, Key> = phf_map! {
    '`' => Key::Tilde,
    '~' => Key::Tilde,
    '1' => Key::One,
    '!' => Key::One,
    '2' => Key::Two,
    '@' => Key::Two,
    '3' => Key::Three,
    '#' => Key::Three,
    '4' => Key::Four,
    '$' => Key::Four,
    '5' => Key::Five,
    '%' => Key::Five,
    '6' => Key::Six,
    '^' => Key::Six,
    '7' => Key::Seven,
    '&' => Key::Seven,
    '8' => Key::Eight,
    '*' => Key::Eight,
    '9' => Key::Nine,
    '(' => Key::Nine,
    '0' => Key::Zero,
    ')' => Key::Zero,
    '-' => Key::Minus,
    '_' => Key::Minus,
    '=' => Key::Equals,
    '+' => Key::Equals,
    '\x08' => Key::Backspace,
    '\t' => Key::Tab,
    'q' => Key::Q,
    'Q' => Key::Q,
    'w' => Key::W,
    'W' => Key::W,
    'e' => Key::E,
    'E' => Key::E,
    'r' => Key::R,
    'R' => Key::R,
    't' => Key::T,
    'T' => Key::T,
    'y' => Key::Y,
    'Y' => Key::Y,
    'u' => Key::U,
    'U' => Key::U,
    'i' => Key::I,
    'I' => Key::I,
    'o' => Key::O,
    'O' => Key::O,
    'p' => Key::P,
    'P' => Key::P,
    '[' => Key::OpenBracket,
    '{' => Key::OpenBracket,
    ']' => Key::CloseBracket,
    '}' => Key::CloseBracket,
    '\\' => Key::Backslash,
    '|' => Key::Backslash,
    'a' => Key::A,
    'A' => Key::A,
    's' => Key::S,
    'S' => Key::S,
    'd' => Key::D,
    'D' => Key::D,
    'f' => Key::F,
    'F' => Key::F,
    'g' => Key::G,
    'G' => Key::G,
    'h' => Key::H,
    'H' => Key::H,
    'j' => Key::J,
    'J' => Key::J,
    'k' => Key::K,
    'K' => Key::K,
    'l' => Key::L,
    'L' => Key::L,
    ';' => Key::Semicolon,
    ':' => Key::Semicolon,
    '\'' => Key::Apostrophe,
    '"' => Key::Apostrophe,
    '\n' => Key::Enter,
    '\r' => Key::Enter,
    'z' => Key::Z,
    'Z' => Key::Z,
    'x' => Key::X,
    'X' => Key::X,
    'c' => Key::C,
    'C' => Key::C,
    'v' => Key::V,
    'V' => Key::V,
    'b' => Key::B,
    'B' => Key::B,
    'n' => Key::N,
    'N' => Key::N,
    'm' => Key::M,
    'M' => Key::M,
    ',' => Key::Comma,
    '<' => Key::Comma,
    '.' => Key::Period,
    '>' => Key::Period,
    '/' => Key::ForwardSlash,
    '?' => Key::ForwardSlash,
};

impl Key {
    fn scan_code() {
        todo!()
    }

    fn hid_code() {
        todo!()
    }

    fn quartz_code() {
        todo!()
    }

    /// Get a `Key` value from an ascii `char`.
    ///
    /// This function will panic if `char` is not an ascii value.
    pub fn from_ascii(char: &char) -> Key {
        ASCII_MAP
            .get(char)
            .expect(format!("Character {} cannot be mapped to keyboard", char).as_str())
            .clone()
    }
}

impl PartialEq<char> for Key {
    fn eq(&self, other: &char) -> bool {
        ASCII_MAP.get(other).map_or(false, |key| key == self)
    }
}

/// A simple struct to represent an RGB color.
pub struct Color {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}
impl Color {
    pub fn new(red: i32, green: i32, blue: i32) -> Color {
        Color { red, green, blue }
    }
}
impl From<(i32, i32, i32)> for Color {
    fn from(color: (i32, i32, i32)) -> Self {
        Color {
            red: color.0,
            green: color.1,
            blue: color.2,
        }
    }
}

/// This empty struct represents an initialised Logitech LED SDK.
///
/// This means it's impossible to call SDK functions without first initializing it,
/// and the SDK is automaticaly shut down when the `Sdk` is dropped.
pub struct Sdk;
impl Drop for Sdk {
    fn drop(&mut self) {
        raw::shutdown();
    }
}
impl Sdk {
    /// If there isn't already another instance running,
    /// makes necessary initializations and disables any existing effects before returning an `Sdk` object.
    ///
    /// If it returns None, it means that either the connection with the SDK is broken
    /// or there's already an instance of the SDK running.
    ///
    /// # Example
    /// ```
    /// let sdk = lightsync::Sdk::init().unwrap();
    /// // do stuff
    /// ```
    pub fn init() -> Option<Sdk> {
        if raw::init() {
            Some(Sdk)
        } else {
            None
        }
    }

    /// If there isn't already another instance running,
    /// makes necessary initializations and disables any existing effects before returning an `Sdk` object.
    ///
    /// It registers the integration with the name provided.
    ///
    /// If it returns None, it means that either the connection with the SDK is broken
    /// or there's already an instance of the SDK running.
    ///
    /// # Panics
    /// Panics if `name` cannot be converted to a C string (contains any null bytes).
    ///
    /// # Example
    /// ```
    /// let sdk = lightsync::Sdk::init_with_name("foo").unwrap();
    /// // do stuff
    /// ```
    pub fn init_with_name(name: &str) -> Option<Sdk> {
        if raw::init_with_name(name) {
            Some(Sdk)
        } else {
            None
        }
    }

    /// Retrieves the version of the SDK installed on the user’s system.
    ///
    /// # Panics
    /// Panics if the connection to the SDK has been lost.
    ///
    /// # Example
    /// ```
    /// let sdk = lightsync::init().unwrap();
    /// println!("{:?}", sdk.version());
    /// ```
    pub fn version(&self) -> (i32, i32, i32) {
        raw::get_sdk_version().expect("LogiLedGetSdkVersion failed")
    }

    /// Sets the target devices for future calls. By default, all devices are targeted.
    ///
    /// The different kinds of devices are `MONOCHROME`, `RGB`, and `PERKEY_RGB`.
    ///
    /// # Panics
    /// Panics if the connection to the SDK has been lost.
    ///
    /// # Example
    /// ```
    /// use lightsync::{Color, Key};
    /// use std::time::Duration;
    ///
    /// let sdk = lightsync::init().unwrap();
    ///
    /// sdk.set_target_devices(lightsync::lighting::RGB | lightsync::lighting::MONOCHROME);
    /// // This call will only affect MONOCHROME and RGB devices,
    /// // and PERKEY_RGB devices like a keybard won't be affected.
    /// sdk.set_lighting(Color::new(100, 0, 0));
    ///
    /// sdk.set_target_devices(lightsync::lighting::PERKEY_RGB);
    /// // These calls will _only_ affect PERKEY_RGB devices.
    /// sdk.set_lighting_for_key(Key::ArrowDown, Color::new(100, 0, 0));
    /// sdk.flash_lighting(Color::new(50, 50, 50), Duration::from_millis(0), Duration::from_millis(300));
    ///
    /// sdk.set_target_devices(lightsync::lighting::ALL);
    /// // Calls will now affect all connected devices again.
    /// sdk.set_lighting(Color::new(50, 0, 0));
    /// ```
    pub fn set_target_devices(&self, target_devices: i32) {
        assert!(
            raw::set_target_device(target_devices),
            "LogiLedSetTargetDevice failed"
        );
    }

    /// Sets the lighting color of all connected devices.
    ///
    /// # Panics
    /// Panics if the connection to the SDK has been lost.
    ///
    /// # Example
    /// ```
    /// let sdk = lightsync::init().unwrap();
    ///
    /// // Green
    /// sdk.set_lighting(lightsync::Color::new(0, 100, 0));
    /// ```
    pub fn set_lighting(&self, color: Color) {
        assert!(
            raw::set_lighting(color.red, color.green, color.blue),
            "LogiLedSetLighting failed"
        );
    }

    /// Sets the lighting in a specific zone of a device.
    ///
    /// # Panics
    /// Panics if the connection to the SDK has been lost.
    ///
    /// # Example
    /// ```
    /// let sdk = lightsync::init().unwrap();
    ///
    /// // Set the logo on mice to green. (zones vary for specific devices)
    /// sdk.set_lighting_for_zone(lightsync::DeviceType::Mouse, 1, lightsync::Color::new(0, 100, 0));
    /// ```
    pub fn set_lighting_for_zone(&self, device_type: DeviceType, zone: i32, color: Color) {
        assert!(
            raw::set_lighting_for_target_zone(
                device_type,
                zone,
                color.red,
                color.green,
                color.blue
            ),
            "LogiLedSetLightingForTargetZone failed"
        )
    }

    /// Saves the current lighting so it can be restored after a temporary effect is finished.
    ///
    /// # Panics
    /// Panics if the connection to the SDK has been lost.
    ///
    /// # Example
    /// ```
    /// use lightsync::Color;
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// let sdk = lightsync::init().unwrap();
    /// sdk.set_lighting(Color::new(0, 100, 0));
    ///
    /// // Save the green lighting
    /// sdk.save_lighting();
    ///
    /// // Set the lighting to red for a second
    /// sdk.set_lighting(Color::new(100, 0, 0));
    /// thread::sleep(Duration::from_millis(1000));
    ///
    /// // Restore the green lighting
    /// sdk.restore_lighting();
    /// ```
    pub fn save_lighting(&self) {
        assert!(
            raw::save_current_lighting(),
            "LogiLedSaveCurrentLighting failed"
        );
    }

    /// Restores the last saved lighting.
    ///
    /// # Panics
    /// Panics if the connection to the SDK has been lost.
    ///
    /// # Example
    /// ```
    /// use lightsync::Color;
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// let sdk = lightsync::init().unwrap();
    /// sdk.set_lighting(Color::new(0, 100, 0));
    ///
    /// // Save the green lighting
    /// sdk.save_lighting();
    ///
    /// // Set the lighting to red for a second
    /// sdk.set_lighting(Color::new(100, 0, 0));
    /// thread::sleep(Duration::from_millis(1000));
    ///
    /// // Restore the green lighting
    /// sdk.restore_lighting();
    /// ```
    pub fn restore_lighting(&self) {
        assert!(raw::restore_lighting(), "LogiLedRestoreLighting failed");
    }

    /// Saves the current lighting, plays a flashing effect at `interval` for `duration` and then restores the saved lighting.
    ///
    /// If you use a duration of 0, the effect will play until stopped with `stop_effects()`.
    ///
    /// # Panics
    /// Panics if the connection to the SDK has been lost.
    ///
    /// # Example
    /// ```
    /// use lightsync::Color;
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// let sdk = lightsync::init().unwrap();
    ///
    /// // Note that this doesn't pause the thread.
    /// sdk.flash_lighting(Color::new(100, 0, 0), Duration::from_millis(2000), Duration::from_millis(500));
    /// thread::sleep(Duration::from_millis(2000));
    /// ```
    pub fn flash_lighting(&self, color: Color, duration: Duration, interval: Duration) {
        assert!(
            raw::flash_lighting(
                color.red,
                color.green,
                color.blue,
                duration
                    .as_millis()
                    .try_into()
                    .expect("Duration is too long to pass to SDK"),
                interval
                    .as_millis()
                    .try_into()
                    .expect("Duration is too long to pass to SDK"),
            ),
            "LogiLedFlashLighting failed"
        )
    }

    /// Saves the current lighting, plays a pulsing effect at `interval` for `duration` and then restores the saved lighting.
    ///
    /// If you use a duration of 0, the effect will play until stopped with `stop_effects()`.
    ///
    /// # Panics
    /// Panics if the connection to the SDK has been lost.
    ///
    /// # Example
    /// ```
    /// use lightsync::Color;
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// let sdk = lightsync::init().unwrap();
    ///
    /// // Note that this doesn't pause the thread.
    /// sdk.pulse_lighting(Color::new(100, 0, 0), Duration::from_millis(2000), Duration::from_millis(500));
    /// thread::sleep(Duration::from_millis(2000));
    /// ```
    pub fn pulse_lighting(&self, color: Color, duration: Duration, interval: Duration) {
        assert!(
            raw::pulse_lighting(
                color.red,
                color.green,
                color.blue,
                duration
                    .as_millis()
                    .try_into()
                    .expect("Duration is too long to pass to SDK"),
                interval
                    .as_millis()
                    .try_into()
                    .expect("Duration is too long to pass to SDK"),
            ),
            "LogiLedPulseLighting failed"
        );
    }

    /// Stops any of the preset effects (flashing/pulsing).
    ///
    /// # Panics
    /// Panics if the connection to the SDK has been lost.
    ///
    /// # Example
    /// ```
    /// use lightsync::Color;
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// let sdk = lightsync::init().unwrap();
    ///
    /// // Flash lighting endlessly
    /// sdk.flash_lighting(Color::new(100, 0, 0), Duration::from_millis(0), Duration::from_millis(500));
    /// thread::sleep(Duration::from_millis(2000));
    ///
    /// // Stop the flashing
    /// sdk.stop_effects();
    /// ```
    pub fn stop_effects(&self) {
        assert!(raw::stop_effects(), "LogiLedStopEffects failed");
    }

    /// Sets the lighting of per-key devices to the grid of RGBA colors `bitmap`.
    ///
    /// To only update part of the keyboard, set the alpha of all other keys to 0.
    ///
    /// # Parameters
    /// - `bitmap`: A 21x6x4 array representing the grid of RGBA pixel values to display.
    /// Each byte ranges from 0 to 255.
    ///
    /// # Panics
    /// Panics if the connection to the SDK has been lost.
    ///
    /// # Layout
    /// The grid is arranged like so:
    ///
    /// |       | **0**       | **1**       | **2**   | **3** | **4** | **5** | **6** | **7** | **8** | **9**  | **10**       | **11**      | **12**       | **13**            | **14**         | **15**     | **16**    | **17**     | 18**     | **19**      | **20**   |
    /// |-------|-------------|-------------|---------|-------|-------|-------|-------|-------|-------|--------|--------------|-------------|--------------|-------------------|----------------|------------|-----------|------------|----------|-------------|----------|
    /// | **0** | Esc         | F1          | F2      | F3    | F4    | F5    | F6    | F7    | F8    | F9     | F10          | F11         | F12          | PrintScreen       | ScrollLock     | PauseBreak |           |            |          |             |          |
    /// | **1** | Tilde       | One         | Two     | Three | Four  | Five  | Six   | Seven | Eight | Nine   | Zero         | Minus       | Equals       | Backspace         | Insert         | Home       | PageUp    | NumLock    | NumSlash | NumAsterisk | NumMinus |
    /// | **2** | Tab         | Q           | W       | E     | R     | T     | Y     | U     | I     | O      | P            | OpenBracket | CloseBracket | Backslash         | KeyboardDelete | End        | PageDown  | NumSeven   | NumEight | NumNine     | NumPlus  |
    /// | **3** | CapsLock    | A           | S       | D     | F     | G     | H     | J     | K     | L      | Semicolon    | Apostrophe  |              | Enter             |                |            |           | NumFour    | NumFive  | NumSix      |          |
    /// | **4** | LeftShift   | Z           | X       | C     | V     | B     | N     | M     | Comma | Period | ForwardSlash |             |              | RightShift        |                | ArrowUp    |           | NumOne     | NumTwo   | NumThree    | NumEnter |
    /// | **5** | LeftControl | LeftWindows | LeftAlt |       |       | Space |       |       |       |        |              | RightAlt    | RightWindows | ApplicationSelect | RightControl   | ArrowLeft  | ArrowDown | ArrowRight | NumZero  | NumPeriod   |          |
    pub fn set_lighting_from_bitmap(&self, bitmap: &[[[u8; 4]; 21]; 6]) {
        let mut bitmap: Vec<u8> = bitmap
            .iter()
            .flat_map(|row| row.iter().flat_map(|cell| cell.iter().cloned()))
            .collect();
        assert!(
            raw::set_lighting_from_bitmap(&mut bitmap),
            "LogiLedSetLightingFromBitmap failed"
        )
    }

    /// Sets a list of keys to be ignored when calling `set_lighting_from_bitmap()`.
    pub fn exclude_keys_from_bitmap(&self, keys: &mut [Key]) {
        assert!(
            raw::exclude_keys_from_bitmap(keys),
            "LogiLedExcludeKeysFromBitmap failed"
        );
    }

    /// Sets the key `key` to the desired color.
    ///
    /// # Panics
    /// Panics if the connection to the SDK has been lost.
    ///
    /// # Example
    /// ```
    /// use lightsync::Color;
    ///
    /// let sdk = lightsync::init().unwrap();
    /// sdk.set_lighting_for_key(lightsync::Key::J, Color::new(100, 0, 0));
    /// ```
    pub fn set_lighting_for_key(&self, key: Key, color: Color) {
        assert!(
            raw::set_lighting_for_key_with_key_name(key, color.red, color.green, color.blue),
            "LogiLedSetLightingForKeyWithKeyName failed"
        );
    }

    /// Saves the current color of `key`, which can later be restored with `restore_lighting_for_key()`.
    ///
    /// # Panics
    /// Panics if the connection to the SDK has been lost.
    ///
    /// # Example
    /// ```
    /// use lightsync::Color;
    ///
    /// let sdk = lightsync::init().unwrap();
    ///
    /// // Make M red
    /// sdk.set_lighting_for_key(lightsync::Key::M, Color::new(100, 0, 0));
    /// sdk.save_lighting_for_key(lightsync::Key::M);
    /// // Make the rest of the keyboard green
    /// sdk.set_lighting(Color::new(0, 100, 0));
    /// // Change M back to red
    /// sdk.restore_lighting_for_key(lightsync::Key::M);
    /// ```
    pub fn save_lighting_for_key(&self, key: Key) {
        assert!(
            raw::save_lighting_for_key(key),
            "LogiLedSaveLightingForKey failed"
        );
    }

    /// Restores the saved color for `key`.
    ///
    /// # Panics
    /// Panics if the connection to the SDK has been lost.
    ///
    /// # Example
    /// ```
    /// use lightsync::Color;
    ///
    /// let sdk = lightsync::init().unwrap();
    ///
    /// // Make M red
    /// sdk.set_lighting_for_key(lightsync::Key::M, Color::new(100, 0, 0));
    /// sdk.save_lighting_for_key(lightsync::Key::M);
    /// // Make the rest of the keyboard green
    /// sdk.set_lighting(Color::new(0, 100, 0));
    /// // Change M back to red
    /// sdk.restore_lighting_for_key(lightsync::Key::M);
    /// ```
    pub fn restore_lighting_for_key(&self, key: Key) {
        assert!(
            raw::restore_lighting_for_key(key),
            "LogiLedRestoreLightingForKey failed"
        );
    }

    /// Starts a flashing effect at `interval` for `duration` on `key`.
    ///
    /// If you use a duration of 0, the effect will play until stopped with `stop_effects_on_key()`.
    ///
    /// # Panics
    /// Panics if the connection to the SDK has been lost.
    ///
    /// # Example
    /// ```
    /// use lightsync::Color;
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// let sdk = lightsync::init().unwrap();
    ///
    /// // Note that this doesn't pause the thread.
    /// sdk.flash_key(lightsync::Key::H, Color::new(100, 0, 0), Duration::from_millis(2000), Duration::from_millis(500));
    /// thread::sleep(Duration::from_millis(2000));
    /// ```
    pub fn flash_key(&self, key: Key, color: Color, duration: Duration, interval: Duration) {
        assert!(
            raw::flash_single_key(
                key,
                color.red,
                color.green,
                color.blue,
                duration
                    .as_millis()
                    .try_into()
                    .expect("Duration is too long to pass to SDK"),
                interval
                    .as_millis()
                    .try_into()
                    .expect("Duration is too long to pass to SDK")
            ),
            "LogiLedFlashSingleKey failed"
        );
    }

    /// Starts a pulsing effect from `start` to `end` for `duration` on `key`.
    ///
    /// With a normal duration, the color will fade from `start` to `end` and then back.
    ///
    /// You can specify `infinite` to have the effect play until stopped with `stop_effects_on_key()`.
    ///
    /// # Parameters
    /// - `key`: The key on which to apply the effect.
    /// - `start`: The color for the key to start on.
    /// - `end`: The color for the key to transition to.
    /// - `duration`: The duration of the effect.
    /// - `infinite`: Whether to run the effect until stopped.
    ///
    /// # Panics
    /// Panics if the connection to the SDK has been lost.
    ///
    /// # Example
    /// ```
    /// use lightsync::Color;
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// let sdk = lightsync::init().unwrap();
    ///
    /// // Note that this doesn't pause the thread.
    /// sdk.pulse_key(lightsync::Key::H, Color::new(100, 0, 0), Color::new(0, 100, 0), Duration::from_millis(2000), false);
    /// thread::sleep(Duration::from_millis(2000));
    /// ```
    pub fn pulse_key(
        &self,
        key: Key,
        start: Color,
        end: Color,
        duration: Duration,
        infinite: bool,
    ) {
        assert!(
            raw::pulse_single_key(
                key,
                start.red,
                start.green,
                start.blue,
                end.red,
                end.green,
                end.blue,
                duration
                    .as_millis()
                    .try_into()
                    .expect("Duration is too long to pass to SDK"),
                infinite,
            ),
            "LogiLedPulseSingleKey failed"
        );
    }

    /// Stops any ongoing effects on `key`.
    ///
    /// # Panics
    /// Panics if the connection to the SDK has been lost.
    ///
    /// # Example
    /// ```
    /// use lightsync::Color;
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// let sdk = lightsync::init().unwrap();
    ///
    /// // Flash the key endlessly
    /// sdk.flash_key(lightsync::Key::H, Color::new(100, 0, 0), Duration::from_millis(2000), Duration::from_millis(500));
    /// thread::sleep(Duration::from_millis(2000));
    ///
    /// // Stop the flashing
    /// sdk.stop_effects_on_key(lightsync::Key::H);
    /// ```
    pub fn stop_effects_on_key(&self, key: Key) {
        assert!(
            raw::stop_effects_on_key(key),
            "LogiLedStopEffectsOnKey failed"
        )
    }

    /// Restores the last saved lighting and frees memory used by the SDK.
    ///
    /// Dropping the Sdk will have the same effect.
    ///
    /// # Example
    /// ```
    /// let sdk = lightsync::init().unwrap();
    ///
    /// // Sdk object is consumed by shutdown and can't be mistakenly used afterwards.
    /// sdk.shutdown();
    /// ```
    pub fn shutdown(self) {
        drop(self);
    }
}

/// If there isn't already another instance running,
/// makes necessary initializations and disables any existing effects before returning an `Sdk` object.
///
/// If it returns None, it means that either the connection with the SDK is broken
/// or there's already an instance of the SDK running.
///
/// # Example
/// ```
/// let sdk = lightsync::Sdk::init().unwrap();
/// // do stuff
/// ```
pub fn init() -> Option<Sdk> {
    Sdk::init()
}

/// If there isn't already another instance running,
/// makes necessary initializations and disables any existing effects before returning an `Sdk` object.
///
/// It registers the integration with the name provided.
///
/// If it returns None, it means that either the connection with the SDK is broken
/// or there's already an instance of the SDK running.
///
/// # Panics
/// Panics if `name` cannot be converted to a C string (contains any null bytes).
///
/// # Example
/// ```
/// let sdk = lightsync::Sdk::init_with_name("foo").unwrap();
/// // do stuff
/// ```
pub fn init_with_name(name: &str) -> Option<Sdk> {
    Sdk::init_with_name(name)
}

// TODO: I could maybe define a trait `ConfigOption` which makes these functions generic.
// not sure if that's a good thing though and some (range) have custom config

/// Gets a color chosen by the user, or `default` if not chosen.
///
/// Path is the identifier for the option,
/// which can be either just a name (e.g. "Terrorist")
/// or a path in a two level tree (e.g. "Colors/Terrorist.")
///
/// If the path is two levels deep, it will be placed inside a section
/// (e.g. "Colors/Terrorist" would be the option "Terrorist" in the section "Colors").
///
/// You can also specify a label, if you want it to be different to the path.
pub fn get_color_option(
    path: &str,
    default: (i32, i32, i32),
    label: Option<&str>,
) -> (i32, i32, i32) {
    if let Some(label) = label {
        raw::set_config_option_label(path, label);
    }
    raw::get_config_option_color(path, default.0, default.1, default.2)
}

/// Gets a boolean chosen by the user, or `default` if not chosen.
///
/// Path is the identifier for the option,
/// which can be either just a name (e.g. "Terrorist")
/// or a path in a two level tree (e.g. "Colors/Terrorist.")
///
/// If the path is two levels deep, it will be placed inside a section
/// (e.g. "Colors/Terrorist" would be the option "Terrorist" in the section "Colors").
///
/// You can also specify a label, if you want it to be different to the path.
pub fn get_boolean_option(path: &str, default: bool, label: Option<&str>) -> bool {
    if let Some(label) = label {
        raw::set_config_option_label(path, label);
    }
    raw::get_config_option_bool(path, default)
}

/// Gets a number chosen by the user, or `default` if not chosen.
///
/// Path is the identifier for the option,
/// which can be either just a name (e.g. "Terrorist")
/// or a path in a two level tree (e.g. "Colors/Terrorist.")
///
/// If the path is two levels deep, it will be placed inside a section
/// (e.g. "Colors/Terrorist" would be the option "Terrorist" in the section "Colors").
///
/// You can also specify a label, if you want it to be different to the path.
pub fn get_number_option(path: &str, default: f64, label: Option<&str>) -> f64 {
    if let Some(label) = label {
        raw::set_config_option_label(path, label);
    }
    raw::get_config_option_number(path, default)
}

/// Gets a number within a range chosen by the user, or `default` if not chosen.
///
/// Path is the identifier for the option,
/// which can be either just a name (e.g. "Terrorist")
/// or a path in a two level tree (e.g. "Colors/Terrorist.")
///
/// If the path is two levels deep, it will be placed inside a section
/// (e.g. "Colors/Terrorist" would be the option "Terrorist" in the section "Colors").
///
/// You can also specify a label, if you want it to be different to the path.
pub fn get_range_option(path: &str, default: i32, min: i32, max: i32, label: Option<&str>) -> i32 {
    if let Some(label) = label {
        raw::set_config_option_label(path, label);
    }
    raw::get_config_option_range(path, default, min, max)
}

/// Gets a rectangle chosen by the user, or `default` if not chosen.
///
/// Path is the identifier for the option,
/// which can be either just a name (e.g. "Terrorist")
/// or a path in a two level tree (e.g. "Colors/Terrorist.")
///
/// If the path is two levels deep, it will be placed inside a section
/// (e.g. "Colors/Terrorist" would be the option "Terrorist" in the section "Colors").
///
/// You can also specify a label, if you want it to be different to the path.
pub fn get_rect_option(
    path: &str,
    default: (i32, i32, i32, i32),
    label: Option<&str>,
) -> (i32, i32, i32, i32) {
    if let Some(label) = label {
        raw::set_config_option_label(path, label);
    }
    raw::get_config_option_rect(path, default.0, default.1, default.2, default.3)
}

#[cfg(test)]
mod tests {}
