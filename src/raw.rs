//! Direct bindings to the API without the concept of `Sdk`.
//!
//! You probably shouldn't use these; they're only here because I plan to use them to create Deno bindings.

use super::bindings::*;
use super::Key;
use std::ffi::OsString;
use std::os::raw::c_int;
use std::os::windows::ffi::OsStrExt;

/// Makes sure there isn’t already another instance running and then makes
/// necessary initializations. It saves the current lighting for all connected and supported devices.
///
/// This function will also stop any effect currently running on the connected devices.
///
/// ## Return value
/// If the function succeeds, it returns true. Otherwise false.
///
/// If it returns false, it means that the connection with Logitech Gaming Software is broken.
pub fn init() -> bool {
    unsafe { LogiLedInit() }
}

/// Makes sure there isn’t already another instance running and then makes necessary initializations.
/// It saves the current lighting for all connected and supported devices.
/// This function will also stop any effect currently running on the connected devices. Passing a name into this
/// function will make the integration show up with a given custom name. The name is set only once, the
/// first time this function or LogiLedInit() is called.
///
/// ## Parameters
/// - name: The referred name for this integration to show up as.
///
/// ## Return value
/// If the function succeeds, it returns true. Otherwise false.
///
/// If it returns false, it means that the connection with Logitech Gaming Software is broken.
pub fn init_with_name(name: &str) -> bool {
    let c_str = std::ffi::CString::new(name)
        .expect(format!("{} cannot be converted to a C string", name).as_str());
    unsafe { LogiLedInitWithName(c_str.as_ptr()) }
}

/// Retrieves the version of the SDK installed on the user’s system.
///
/// If it returns None, means that there is no SDK installed on the user system, or the sdk version could not
/// be retrieved.
pub fn get_sdk_version() -> Option<super::Color> {
    let mut major: c_int = 0;
    let mut minor: c_int = 0;
    let mut patch: c_int = 0;

    if unsafe { LogiLedGetSdkVersion(&mut major, &mut minor, &mut patch) } {
        Some((major.into(), minor.into(), patch.into()))
    } else {
        None
    }
}

/// Sets the target device type for future calls. The default target
/// device is LOGI_DEVICETYPE_ALL, therefore, if no call is made to LogiLedSetTargetDevice the SDK will
/// apply any function to all the connected devices.
/// ## Parameters
/// - targetDevice: one or a combinaton of the following values:
///   - `LOGI_DEVICETYPE_MONOCHROME`
///   - `LOGI_DEVICETYPE_RGB`
///   - `LOGI_DEVICETYPE_PERKEY_RGB`
///   - `LOGI_DEVICETYPE_ALL`
///
/// ## Return value
/// If the function succeeds, it returns true. Otherwise false.
///
/// The function will return false if LogiLedInit() hasn’t been called, the parameter is wrong, or if the
/// connection with Logitech Gaming Software was lost.
pub fn set_target_device(target_device: i32) -> bool {
    unsafe { LogiLedSetTargetDevice(target_device) }
}

/// Saves the current lighting so that it can be restored after
/// a temporary effect is finished. For example if flashing a red warning sign for a few seconds, you would
/// call the LogiLedSaveCurrentLighting() function just before starting the warning effect.
///
/// On per-key backlighting supporting devices, this function will save the current state for each key.
/// bool LogiLedSaveCurrentLighting();
/// ## Return value
/// If the function succeeds, it returns true. Otherwise false.
///
/// The function will return false if LogiLedInit() hasn’t been called or if the connection with Logitech
/// Gaming Software was lost.
pub fn save_current_lighting() -> bool {
    unsafe { LogiLedSaveCurrentLighting() }
}

/// The LogiLedSetLighting() function sets the lighting on connected and supported devices.
/// ## Parameters
/// - red: amount of red. Range is 0 to 100.
/// - green: amount of green. Range is 0 to 100.
/// - blue: amount of biue. Range is 0 to 100.
/// ## Return value
/// If the function succeeds, it returns true. Otherwise false.
///
/// The function will return false if LogiLedInit() hasn’t been called or if the connection with Logitech
/// Gaming Software was lost.
///
/// ## Remarks
/// Do not call this function immediately after LogiLedInit(). Instead leave a little bit of time after
/// LogiLedInit().
/// For devices that only support a single color, the highest percentage value given of the three colors will
/// define the intensity. For monochrome backlighting device, Logitech Gaming Software will reduce
/// proportionally the value of the highest color, according to the user hardware brightness setting.
pub fn set_lighting(color: super::Color) -> bool {
    unsafe { LogiLedSetLighting(color.0, color.1, color.2) }
}

/// Sets lighting on a specific zone for all connected
/// zonal devices that match the device type.
/// ## Parameters
/// - deviceType: one of the device types from the enum DeviceType:
///   - Keyboard = 0x0,
///   - Mouse = 0x3,
///   - Mousemat = 0x4,
///   - Headset = 0x8,
///   - Speaker = 0xe
/// - zone: the zone ID to set iightng on. For device zone IDs, consult "Features of lighting capable
/// Logitech Gaming devices")
/// - red: amount of red. Range is 0 to 100.
/// - green: amount of green. Range is 0 to 100.
/// - biuePercentage: amount of biue. Range is 0 to 100.
/// ## Return value
/// If the function succeeds, it returns true. Otherwise false.
///
/// The function will return false if LogiLedInit() hasn’t been called or if the connection with Logitech
/// Gaming Software was lost.
/// ## Remarks
/// This function will only affect devices with Zonal Lighting. This excludes keyboards with single key RGB
/// support. Additionally, setting a zone will affect all connected devices of specified type.
pub fn set_lighting_for_target_zone(
    device_type: super::DeviceType,
    zone: i32,
    color: super::Color
) -> bool {
    unsafe { LogiLedSetLightingForTargetZone(device_type.into(), zone, color.0, color.1, color.2) }
}

/// Restores the last saved lighting. It should be called after a
/// temporary effect is finished. For example if flashing a red warning sign for a few seconds, you would call
/// this function right after the warning effect is finished.
///
/// On per-key backlighting supporting devices, this function will restore the saved state for each key.
/// ## Return value
/// If the function succeeds, it returns true. Otherwise false.
/// The function will return false if LogiLedInit() hasn’t been called or if the connection with Logitech
/// Gaming Software was lost.
pub fn restore_lighting() -> bool {
    unsafe { LogiLedRestoreLighting() }
}

pub fn flash_lighting(color: super::Color, duration: i32, interval: i32) -> bool {
    unsafe { LogiLedFlashLighting(color.0, color.1, color.2, duration, interval) }
}

/// Saves the current lighting, plays the pulsing effect on the
/// targeted devices and, finally, restores the saved lighting.
/// ## Parameters
/// - red: amount of red. Range is 0 to 100.
/// - green: amount of green. Range is 0 to 100.
/// - biue: amount of biue. Range is 0 to 100.
/// - duration : duration of the efect in milliseconds, this parameter can be set to
/// LOGI_LED_DURATION_INFINITE to make the effect run unti stopped through
/// LogiLedStopEfects().
/// - milliSecondsIntervai : duration of the flashing interval in milliseconds
/// ## Return value
/// If the function succeeds, it returns true. Otherwise false.
///
/// The function will return false if LogiLedInit() hasn’t been called, if the connection with Logitech Gaming
/// Software was lost or if another effect is currently running.
pub fn pulse_lighting(color: super::Color, duration: i32, interval: i32) -> bool {
    unsafe { LogiLedPulseLighting(color.0, color.1, color.2, duration, interval) }
}

/// Stops any of the presets effects (started from LogiLedFlashLighting or LogiLedPulseLighting).
/// ## Return value
/// If the function succeeds, it returns true. Otherwise false.
///
/// The function will return false if LogiLedInit() hasn’t been called or if the connection with Logitech
/// Gaming Software was lost.
pub fn stop_effects() -> bool {
    unsafe { LogiLedStopEffects() }
}

/// Sets the array of bytes passed as parameter as colors
/// to per-key backlighting featured connected devices.
/// bool LogiLedSetLightingFromBitmap(unsigned char bitmap[]);
/// ## Parameters
/// - bitmap: a unsigned char array containing the colors to assign to each ket on the er-iightng
/// device connected. The size required for this bitmap is defned bt LOGI_LED_BITMAP_SIZE.
/// The array of pixels is organized as a rectangular area, 21x6, representing the keys on the device. Each
/// color is represented by four consecutive bytes (RGBA).
/// 32 bit values are stored in 4 consecutive bytes that represent the RGB color values for that pixel.
/// These values use the same top left to bottom right raster style transform to the flat character array with
/// the exception that each pixel value is specified using 4 consecutive bytes. The illustration below shows
/// the data arrangement for these RGB quads.
///
/// Each of the bytes in the RGB quad specify the intensity of the given color. The value ranges from 0 (the
/// darkest color value) to 255 (brightest color value).
/// ## Return value
/// If the function succeeds, it returns true. Otherwise false.
///
/// The function will return false if LogiLedInit() hasn’t been called or if the connection with Logitech
/// Gaming Software was lost.
/// ## Remarks
/// The array passed in has to be allocated by the caller of the size LOGI_LED_BITMAP_SIZE. If the array is
/// smaller, the function will apply the effect to a smaller portion of the keyboard and set everything else to
/// black. If the array is bigger, the remaining part will be ignored. To create partial bitmaps and update only
/// parts of the keyboard, set the alpha channel for the keys to ignore to 0. This will allow to update just
/// portion of the keyboard, without overriding the other keys.
pub fn set_lighting_from_bitmap(bitmap: &mut [u8]) -> bool {
    unsafe { LogiLedSetLightingFromBitmap(bitmap.as_mut_ptr()) }
}

/// The LogiLedExcludeKeysFromBitmap() function sets a list of keys, defined by keynames to be
/// ignored when calling the function LogiLedSetLightingFromBitmap. This is useful when creating effects on
/// the bitmap during gameplay loop, but still wanting to set some keys on top of that using the
/// LogiLedSetLightingFromKeyName.
/// ## Parameters
/// - keys: A preallocated array of LogiLed::KeyNames) to be excluded when calling
/// LogiLedSetLightingFromKeyName.
/// - iistCount: the number of items in the iist K’etList
pub fn exclude_keys_from_bitmap(keys: &[super::Key]) -> bool {
    unsafe {
        LogiLedExcludeKeysFromBitmap(
            keys.iter()
                .cloned()
                .map(super::Key::into)
                .collect::<Vec<c_int>>()
                .as_mut_ptr(),
            keys.len() as c_int,
        )
    }
}

/// Sets the key identified by the scancode
/// passed as parameter to the desired color. This function only affects per-key backlighting featured
/// connected devices.
/// Parameters
/// - keyCode: the scan-code of the key to set.
/// - red: amount of red. Range is 0 to 100.
/// - green: amount of green. Range is 0 to 100.
/// - biue: amount of biue. Range is 0 to 100.
/// ## Return value
/// If the function succeeds, it returns true. Otherwise false.
///
/// The function will return false if LogiLedInit() hasn’t been called or if the connection with Logitech
/// Gaming Software was lost.
pub fn set_lighting_for_key_with_scan_code(code: i32, color: super::Color) -> bool {
    unsafe { LogiLedSetLightingForKeyWithScanCode(code, color.0, color.1, color.2) }
}

/// Sets the key identified by the hid code
/// passed as parameter to the desired color. This function only affects per-key backlighting featured
/// connected devices.
/// ## Parameters
/// - key: the hid-code of the key to set.
/// - red: amount of red. Range is 0 to 100.
/// - green: amount of green. Range is 0 to 100.
/// - blue: amount of biue. Range is 0 to 100.
/// ## Return value
/// If the function succeeds, it returns true. Otherwise false.
///
/// The function will return false if LogiLedInit() hasn’t been called or if the connection with Logitech
/// Gaming Software was lost.
pub fn set_lighting_for_key_with_hid_code(code: i32, color: super::Color) -> bool {
    unsafe { LogiLedSetLightingForKeyWithHidCode(code, color.0, color.1, color.2) }
}

/// Sets the key identified by the quartz code
/// passed as parameter to the desired color. This function only affects per-key backlighting featured
/// connected devices.
/// ## Parameters
/// - key: the quartz-code of the key to set
/// - red: amount of red. Range is 0 to 100.
/// - green: amount of green. Range is 0 to 100.
/// - blue: amount of biue. Range is 0 to 100.
/// ## Return value
/// If the function succeeds, it returns true. Otherwise false.
///
/// The function will return false if LogiLedInit() hasn’t been called or if the connection with Logitech
/// Gaming Software was lost.
pub fn set_lighting_for_key_with_quartz_code(code: i32, color: super::Color) -> bool {
    unsafe { LogiLedSetLightingForKeyWithQuartzCode(code, color.0, color.1, color.2) }
}

/// Sets the key identified by the code passed
/// as parameter to the desired color. This function only affects per-key backlighting featured connected
/// devices.
/// ## Parameters
/// - key: one of the key codes from the enum Key.
/// - red: amount of red. Range is 0 to 100.
/// - green: amount of green. Range is 0 to 100.
/// - blue: amount of blue. Range is 0 to 100.
/// ## Return value
/// If the function succeeds, it returns true. Otherwise false.
/// The function will return false if LogiLedInit() hasn’t been called or if the connection with Logitech
/// Gaming Software was lost.
pub fn set_lighting_for_key_with_key_name(key: Key, color: super::Color) -> bool {
    unsafe { LogiLedSetLightingForKeyWithKeyName(key.into(), color.0, color.1, color.2) }
}

/// Saves the current color on the keycode passed as
/// argument. Use this function with the LogiLedRestoreLightingForKey to preserve the state of a key before
/// applying any effect.
///
/// This function only applies to device of the family LOGI_DEVICETYPE_PERKEY_RGB.
/// Parameters
/// - keyName: The key to save the color for. A value from the LogiLed::KeyName enum.
/// ##Return value
/// If the function succeeds, it returns true. Otherwise false.
///
/// The function will return false if LogiLedInit() hasn’t been called or if the connection with Logitech
/// Gaming Software was lost.
pub fn save_lighting_for_key(key: Key) -> bool {
    unsafe { LogiLedSaveLightingForKey(key.into()) }
}

/// Restores the saved color on the keycode passed as
/// argument. Use this function with the LogiLedSaveLightingForKey to preserve the state of a key before
/// applying any effect.
///
/// This function only applies to device of the family LOGI_DEVICETYPE_PERKEY_RGB.
/// ## Parameters
/// - keyName: The key to restore the color on. A value from the LogiLed::KeyName enum.
/// ## Return value
/// If the function succeeds, it returns true. Otherwise false.
///
/// The function will return false if LogiLedInit() hasn’t been called or if the connection with Logitech
/// Gaming Software was lost.
pub fn restore_lighting_for_key(key: Key) -> bool {
    unsafe { LogiLedRestoreLightingForKey(key.into()) }
}

/// The LogiLedFlashSingleKey() function starts a flashing effect on the key passed as parameter. The
/// key will be flashing with an interval as defined by msInterval for msDuration milliseconds, alternating the
/// color passed in as parameter and black. This function only applies to device of the family
/// LOGI_DEVICETYPE_PERKEY_RGB.
/// ## Parameters
/// - keyName: The key to restore the color on. A vaiue from the LogiLed::KeyName enum.
/// - red : amount of red in the actve coior of the fash efect. eange is 0 to 100.
/// - green : amount of green in the actve coior of the fash efect. eange is 0 to 100.
/// - blue : amount of biue in the actve coior of the fash efect. eange is 0 to 100.
/// - msDuraton : duraton in miiiiseconds of the efect on the singie ket. This parameter can be set
/// to LOGI_LED_DURATION_INFINITE to make the efect run unti stopped through
/// LogiLedStopEfects() or LogiLedStopEfectsOnKey().
/// ## Return value
/// If the function succeeds, it returns true. Otherwise false.
/// The function will return false if LogiLedInit() hasn’t been called or if the connection with Logitech
/// Gaming Software was lost.
pub fn flash_single_key(key: Key, color: super::Color, duration: i32, interval: i32) -> bool {
    unsafe { LogiLedFlashSingleKey(key.into(), color.0, color.1, color.2, duration, interval) }
}

/// The LogiLedPulseSingleKey() function starts a pulsing effect on the key passed as parameter. The key
/// will be pulsing with from start color to finish color for msDuration milliseconds. This function only applies
/// to device of the family LOGI_DEVICETYPE_PERKEY_RGB.
/// ## Parameters
/// - key: The key to restore the color on. A value from the LogiLed::KeyName enum.
/// - start_red: amount of red in the start color of the pulse effect. Range is 0 to 100.
/// - start_green: amount of green in the start color of the pulse effect. Rnge is 0 to 100.
/// - start_blue: amount of blue in the start color of the pulse effect. Range is 0 to 100.
/// - end_red: amount of red in the finish color of the pulse effect. Range is 0 to 100.
/// - end_green: amount of green in the finish color of the pulse effect. Range is 0 to 100.
/// - end_blue: amount of blue in the finish color of the pulse effect. Range is 0 to 100.
/// - duration: duration of the effect on the singie key.
/// - infinite : if this is set to true the effect will loop infinitely until stopped with a call to
/// LogiLedStopEfects() or LogiLedStopEfectsOnKey()
/// ## Return value
/// If the function succeeds, it returns true. Otherwise false.
///
/// The function will return false if LogiLedInit() hasn’t been called or if the connection with Logitech
/// Gaming Software was lost.
pub fn pulse_single_key(
    key: Key,
    start: super::Color,
    end: super::Color,
    duration: i32,
    infinite: bool,
) -> bool {
    unsafe {
        LogiLedPulseSingleKey(
            key.into(),
            start.0,
            start.1,
            start.2,
            end.0,
            end.1,
            end.2,
            duration,
            infinite,
        )
    }
}

pub fn stop_effects_on_key(key: Key) -> bool {
    unsafe { LogiLedStopEffectsOnKey(key.into()) }
}

/// Restores the last saved lighting and frees memory used by the SDK.
pub fn shutdown() {
    unsafe { LogiLedShutdown() }
}

pub fn set_config_option_label(path: &str, label: &str) {
    let os_path = OsString::from(path);
    let path_ptr = os_path.encode_wide().collect::<Vec<u16>>().as_ptr();
    let os_label = OsString::from(label);
    let label_ptr = os_label.encode_wide().collect::<Vec<u16>>().as_mut_ptr();
    unsafe {
        assert!(
            LogiLedSetConfigOptionLabel(path_ptr, label_ptr),
            "LogiLedSetConfigOptionLabel failed"
        );
    }
}

pub fn get_config_option_number(path: &str, default: f64) -> f64 {
    let mut value = default;
    let os_string = OsString::from(path);
    let path_ptr = os_string.encode_wide().collect::<Vec<u16>>().as_ptr();
    unsafe {
        assert!(
            LogiLedGetConfigOptionNumber(path_ptr, &mut value),
            "LogiLedGetConfigOptionNumber failed"
        );
    }
    value
}

pub fn get_config_option_bool(path: &str, default: bool) -> bool {
    let mut value = default;
    let os_string = OsString::from(path);
    let path_ptr = os_string.encode_wide().collect::<Vec<u16>>().as_ptr();
    unsafe {
        assert!(
            LogiLedGetConfigOptionBool(path_ptr, &mut value),
            "LogiLedGetConfigOptionBool failed"
        );
    }
    value
}

pub fn get_config_option_color(
    path: &str,
    default: super::Color
) -> super::Color {
    let mut red = default.0;
    let mut green = default.1;
    let mut blue = default.2;
    let os_string = OsString::from(path);
    let path_ptr = os_string.encode_wide().collect::<Vec<u16>>().as_ptr();
    unsafe {
        assert!(
            LogiLedGetConfigOptionColor(path_ptr, &mut red, &mut green, &mut blue),
            "LogiLedGetConfigOptionColor failed"
        );
    }
    (red, green, blue)
}

pub fn get_config_option_range(path: &str, default: i32, min: i32, max: i32) -> i32 {
    let mut value = default;
    let os_string = OsString::from(path);
    let path_ptr = os_string.encode_wide().collect::<Vec<u16>>().as_ptr();
    unsafe {
        assert!(
            LogiLedGetConfigOptionRange(path_ptr, &mut value, min, max),
            "LogiLedGetConfigOptionRange failed"
        );
    }
    value
}

pub fn get_config_option_rect(
    path: &str,
    default_x: i32,
    default_y: i32,
    default_width: i32,
    default_height: i32,
) -> (i32, i32, i32, i32) {
    let mut x = default_x;
    let mut y = default_y;
    let mut width = default_width;
    let mut height = default_height;
    let os_string = OsString::from(path);
    let path_ptr = os_string.encode_wide().collect::<Vec<u16>>().as_ptr();
    unsafe {
        assert!(
            LogiLedGetConfigOptionRect(path_ptr, &mut x, &mut y, &mut width, &mut height),
            "LogiLedGetConfigOptionRect failed"
        );
    }
    (x, y, width, height)
}

// Missing option types: KeyInput and Select, I'm not too sure how they're represented. (and they're undocumented)
