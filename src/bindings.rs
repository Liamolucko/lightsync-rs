use std::os::raw::{c_int, c_char, c_uchar};

extern "C" {
    #[link_name = "\u{1}?LogiLedInit@@YA_NXZ"]
    pub fn LogiLedInit() -> bool;

    #[link_name = "\u{1}?LogiLedInitWithName@@YA_NQEBD@Z"]
    pub fn LogiLedInitWithName(name: *const c_char) -> bool;

    #[link_name = "\u{1}?LogiLedGetSdkVersion@@YA_NPEAH00@Z"]
    pub fn LogiLedGetSdkVersion(
        majorNum: *mut c_int,
        minorNum: *mut c_int,
        buildNum: *mut c_int,
    ) -> bool;

    #[link_name = "\u{1}?LogiLedGetConfigOptionNumber@@YA_NPEB_WPEAN@Z"]
    pub fn LogiLedGetConfigOptionNumber(configPath: *const u16, defaultValue: *mut f64) -> bool;

    #[link_name = "\u{1}?LogiLedGetConfigOptionBool@@YA_NPEB_WPEA_N@Z"]
    pub fn LogiLedGetConfigOptionBool(configPath: *const u16, defaultValue: *mut bool) -> bool;

    #[link_name = "\u{1}?LogiLedGetConfigOptionColor@@YA_NPEB_WPEAH11@Z"]
    pub fn LogiLedGetConfigOptionColor(
        configPath: *const u16,
        defaultRed: *mut c_int,
        defaultGreen: *mut c_int,
        defaultBlue: *mut c_int,
    ) -> bool;

    #[link_name = "\u{1}?LogiLedGetConfigOptionRect@@YA_NPEB_WPEAH111@Z"]
    pub fn LogiLedGetConfigOptionRect(
        configPath: *const u16,
        defaultX: *mut c_int,
        defaultY: *mut c_int,
        defaultWidth: *mut c_int,
        defaultHeight: *mut c_int,
    ) -> bool;

    #[link_name = "\u{1}?LogiLedGetConfigOptionString@@YA_NPEB_WPEA_WH@Z"]
    pub fn LogiLedGetConfigOptionString(
        configPath: *const u16,
        defaultValue: *mut u16,
        bufferSize: c_int,
    ) -> bool;

    #[link_name = "\u{1}?LogiLedGetConfigOptionKeyInput@@YA_NPEB_WPEA_WH@Z"]
    pub fn LogiLedGetConfigOptionKeyInput(
        configPath: *const u16,
        defaultValue: *mut u16,
        bufferSize: c_int,
    ) -> bool;

    #[link_name = "\u{1}?LogiLedGetConfigOptionSelect@@YA_NPEB_WPEA_WPEAH0H@Z"]
    pub fn LogiLedGetConfigOptionSelect(
        configPath: *const u16,
        defaultValue: *mut u16,
        valueSize: *mut c_int,
        values: *const u16,
        bufferSize: c_int,
    ) -> bool;

    #[link_name = "\u{1}?LogiLedGetConfigOptionRange@@YA_NPEB_WPEAHHH@Z"]
    pub fn LogiLedGetConfigOptionRange(
        configPath: *const u16,
        defaultValue: *mut c_int,
        min: c_int,
        max: c_int,
    ) -> bool;

    #[link_name = "\u{1}?LogiLedSetConfigOptionLabel@@YA_NPEB_WPEA_W@Z"]
    pub fn LogiLedSetConfigOptionLabel(configPath: *const u16, label: *mut u16) -> bool;

    #[link_name = "\u{1}?LogiLedSetTargetDevice@@YA_NH@Z"]
    pub fn LogiLedSetTargetDevice(targetDevice: c_int) -> bool;

    #[link_name = "\u{1}?LogiLedSaveCurrentLighting@@YA_NXZ"]
    pub fn LogiLedSaveCurrentLighting() -> bool;

    #[link_name = "\u{1}?LogiLedSetLighting@@YA_NHHH@Z"]
    pub fn LogiLedSetLighting(
        redPercentage: c_int,
        greenPercentage: c_int,
        bluePercentage: c_int,
    ) -> bool;

    #[link_name = "\u{1}?LogiLedRestoreLighting@@YA_NXZ"]
    pub fn LogiLedRestoreLighting() -> bool;

    #[link_name = "\u{1}?LogiLedFlashLighting@@YA_NHHHHH@Z"]
    pub fn LogiLedFlashLighting(
        redPercentage: c_int,
        greenPercentage: c_int,
        bluePercentage: c_int,
        milliSecondsDuration: c_int,
        milliSecondsInterval: c_int,
    ) -> bool;

    #[link_name = "\u{1}?LogiLedPulseLighting@@YA_NHHHHH@Z"]
    pub fn LogiLedPulseLighting(
        redPercentage: c_int,
        greenPercentage: c_int,
        bluePercentage: c_int,
        milliSecondsDuration: c_int,
        milliSecondsInterval: c_int,
    ) -> bool;

    #[link_name = "\u{1}?LogiLedStopEffects@@YA_NXZ"]
    pub fn LogiLedStopEffects() -> bool;

    #[link_name = "\u{1}?LogiLedSetLightingFromBitmap@@YA_NQEAE@Z"]
    pub fn LogiLedSetLightingFromBitmap(bitmap: *mut c_uchar) -> bool;

    #[link_name = "\u{1}?LogiLedSetLightingForKeyWithScanCode@@YA_NHHHH@Z"]
    pub fn LogiLedSetLightingForKeyWithScanCode(
        keyCode: c_int,
        redPercentage: c_int,
        greenPercentage: c_int,
        bluePercentage: c_int,
    ) -> bool;

    #[link_name = "\u{1}?LogiLedSetLightingForKeyWithHidCode@@YA_NHHHH@Z"]
    pub fn LogiLedSetLightingForKeyWithHidCode(
        keyCode: c_int,
        redPercentage: c_int,
        greenPercentage: c_int,
        bluePercentage: c_int,
    ) -> bool;

    #[link_name = "\u{1}?LogiLedSetLightingForKeyWithQuartzCode@@YA_NHHHH@Z"]
    pub fn LogiLedSetLightingForKeyWithQuartzCode(
        keyCode: c_int,
        redPercentage: c_int,
        greenPercentage: c_int,
        bluePercentage: c_int,
    ) -> bool;

    #[link_name = "\u{1}?LogiLedSetLightingForKeyWithKeyName@@YA_NW4KeyName@LogiLed@@HHH@Z"]
    pub fn LogiLedSetLightingForKeyWithKeyName(
        keyName: c_int,
        redPercentage: c_int,
        greenPercentage: c_int,
        bluePercentage: c_int,
    ) -> bool;

    #[link_name = "\u{1}?LogiLedSaveLightingForKey@@YA_NW4KeyName@LogiLed@@@Z"]
    pub fn LogiLedSaveLightingForKey(keyName: c_int) -> bool;

    #[link_name = "\u{1}?LogiLedRestoreLightingForKey@@YA_NW4KeyName@LogiLed@@@Z"]
    pub fn LogiLedRestoreLightingForKey(keyName: c_int) -> bool;

    #[link_name = "\u{1}?LogiLedExcludeKeysFromBitmap@@YA_NPEAW4KeyName@LogiLed@@H@Z"]
    pub fn LogiLedExcludeKeysFromBitmap(
        keyList: *mut c_int,
        listCount: c_int,
    ) -> bool;

    #[link_name = "\u{1}?LogiLedFlashSingleKey@@YA_NW4KeyName@LogiLed@@HHHHH@Z"]
    pub fn LogiLedFlashSingleKey(
        keyName: c_int,
        redPercentage: c_int,
        greenPercentage: c_int,
        bluePercentage: c_int,
        msDuration: c_int,
        msInterval: c_int,
    ) -> bool;

    #[link_name = "\u{1}?LogiLedPulseSingleKey@@YA_NW4KeyName@LogiLed@@HHHHHHH_N@Z"]
    pub fn LogiLedPulseSingleKey(
        keyName: c_int,
        startRedPercentage: c_int,
        startGreenPercentage: c_int,
        startBluePercentage: c_int,
        finishRedPercentage: c_int,
        finishGreenPercentage: c_int,
        finishBluePercentage: c_int,
        msDuration: c_int,
        isInfinite: bool,
    ) -> bool;

    #[link_name = "\u{1}?LogiLedStopEffectsOnKey@@YA_NW4KeyName@LogiLed@@@Z"]
    pub fn LogiLedStopEffectsOnKey(keyName: c_int) -> bool;

    #[link_name = "\u{1}?LogiLedSetLightingForTargetZone@@YA_NW4DeviceType@LogiLed@@HHHH@Z"]
    pub fn LogiLedSetLightingForTargetZone(
        deviceType: c_int,
        zone: c_int,
        redPercentage: c_int,
        greenPercentage: c_int,
        bluePercentage: c_int,
    ) -> bool;

    #[link_name = "\u{1}?LogiLedShutdown@@YAXXZ"]
    pub fn LogiLedShutdown();
}
