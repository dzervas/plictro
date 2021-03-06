/* automatically generated by rust-bindgen */
/* Generated by grep "#define" (and some modifications) on
   https://github.com/adafruit/Adafruit_TinyUSB_ArduinoCore/blob/master/tinyusb/src/class/hid/hid.h */


//--------------------------------------------------------------------+
// REPORT DESCRIPTOR
//--------------------------------------------------------------------+

macro_rules! HID_REPORT_ITEM {
	($tag: expr, $type: expr, $size: expr) => {
		(($tag << 4) | ($type << 2) | $size)
	};
}

pub const RI_TYPE_MAIN: u32 = 0;
pub const RI_TYPE_GLOBAL: u32 = 1;
pub const RI_TYPE_LOCAL: u32 = 2;

//------------- MAIN ITEMS 6.2.2.4 -------------//
macro_rules! HID_INPUT { ($data: expr) =>          { HID_REPORT_ITEM!(8, RI_TYPE_MAIN, 1) }; }
macro_rules! HID_OUTPUT { ($data: expr) =>         { HID_REPORT_ITEM!(9, RI_TYPE_MAIN, 1) }; }
macro_rules! HID_COLLECTION { ($data: expr) =>     { HID_REPORT_ITEM!(10, RI_TYPE_MAIN, 1) }; }
// macro_rules! HID_FEATURE { ($data: expr) =>        { HID_REPORT_ITEM!(11, RI_TYPE_MAIN, 1) }; }
macro_rules! HID_COLLECTION_END { () =>            { HID_REPORT_ITEM!(12, RI_TYPE_MAIN, 0) }; }

//------------- GLOBAL ITEMS 6.2.2.7 -------------//
macro_rules! HID_USAGE_PAGE { ($data: expr) =>     { HID_REPORT_ITEM!(0, RI_TYPE_GLOBAL, 1) }; }
macro_rules! HID_LOGICAL_MIN { ($data: expr) =>    { HID_REPORT_ITEM!(1, RI_TYPE_GLOBAL, 1) }; }
macro_rules! HID_LOGICAL_MAX { ($data: expr) =>    { HID_REPORT_ITEM!(2, RI_TYPE_GLOBAL, 1) }; }
macro_rules! HID_LOGICAL_MAX_N { ($data: expr, $size: expr) =>    { HID_REPORT_ITEM!(2, RI_TYPE_GLOBAL, $size) }; }
// macro_rules! HID_PHYSICAL_MIN { ($data: expr) =>   { HID_REPORT_ITEM!(3, RI_TYPE_GLOBAL, 1) }; }
// macro_rules! HID_PHYSICAL_MAX { ($data: expr) =>   { HID_REPORT_ITEM!(4, RI_TYPE_GLOBAL, 1) }; }
// macro_rules! HID_UNIT_EXPONENT { ($data: expr) =>  { HID_REPORT_ITEM!(5, RI_TYPE_GLOBAL, 1) }; }
// macro_rules! HID_UNIT    { ($data: expr) =>        { HID_REPORT_ITEM!(6, RI_TYPE_GLOBAL, 1) }; }
macro_rules! HID_REPORT_SIZE { ($data: expr) =>    { HID_REPORT_ITEM!(7, RI_TYPE_GLOBAL, 1) }; }
macro_rules! HID_REPORT_ID { ($data: expr) =>      { HID_REPORT_ITEM!(8, RI_TYPE_GLOBAL, 1) }; }
macro_rules! HID_REPORT_COUNT { ($data: expr) =>   { HID_REPORT_ITEM!(9, RI_TYPE_GLOBAL, 1) }; }

// #define HID_PUSH                  HID_REPORT_ITEM(x, 10, RI_TYPE_GLOBAL, 0)
// #define HID_POP                   HID_REPORT_ITEM(x, 11, RI_TYPE_GLOBAL, 0)

//------------- LOCAL ITEMS 6.2.2.8 -------------//
macro_rules! HID_USAGE { ($data: expr) =>          { HID_REPORT_ITEM!(0, RI_TYPE_LOCAL, 1) }; }
macro_rules! HID_USAGE_MIN { ($data: expr) =>      { HID_REPORT_ITEM!(1, RI_TYPE_LOCAL, 1) }; }
macro_rules! HID_USAGE_MAX { ($data: expr) =>      { HID_REPORT_ITEM!(2, RI_TYPE_LOCAL, 1) }; }
macro_rules! HID_USAGE_MAX_N { ($data: expr, $size: expr) =>      { HID_REPORT_ITEM!(2, RI_TYPE_LOCAL, $size) }; }

//--------------------------------------------------------------------+
// HID KEYCODE
//--------------------------------------------------------------------+
pub const HID_KEY_NONE: u32 = 0;
pub const HID_KEY_A: u32 = 4;
pub const HID_KEY_B: u32 = 5;
pub const HID_KEY_C: u32 = 6;
pub const HID_KEY_D: u32 = 7;
pub const HID_KEY_E: u32 = 8;
pub const HID_KEY_F: u32 = 9;
pub const HID_KEY_G: u32 = 10;
pub const HID_KEY_H: u32 = 11;
pub const HID_KEY_I: u32 = 12;
pub const HID_KEY_J: u32 = 13;
pub const HID_KEY_K: u32 = 14;
pub const HID_KEY_L: u32 = 15;
pub const HID_KEY_M: u32 = 16;
pub const HID_KEY_N: u32 = 17;
pub const HID_KEY_O: u32 = 18;
pub const HID_KEY_P: u32 = 19;
pub const HID_KEY_Q: u32 = 20;
pub const HID_KEY_R: u32 = 21;
pub const HID_KEY_S: u32 = 22;
pub const HID_KEY_T: u32 = 23;
pub const HID_KEY_U: u32 = 24;
pub const HID_KEY_V: u32 = 25;
pub const HID_KEY_W: u32 = 26;
pub const HID_KEY_X: u32 = 27;
pub const HID_KEY_Y: u32 = 28;
pub const HID_KEY_Z: u32 = 29;
pub const HID_KEY_1: u32 = 30;
pub const HID_KEY_2: u32 = 31;
pub const HID_KEY_3: u32 = 32;
pub const HID_KEY_4: u32 = 33;
pub const HID_KEY_5: u32 = 34;
pub const HID_KEY_6: u32 = 35;
pub const HID_KEY_7: u32 = 36;
pub const HID_KEY_8: u32 = 37;
pub const HID_KEY_9: u32 = 38;
pub const HID_KEY_0: u32 = 39;
pub const HID_KEY_RETURN: u32 = 40;
pub const HID_KEY_ESCAPE: u32 = 41;
pub const HID_KEY_BACKSPACE: u32 = 42;
pub const HID_KEY_TAB: u32 = 43;
pub const HID_KEY_SPACE: u32 = 44;
pub const HID_KEY_MINUS: u32 = 45;
pub const HID_KEY_EQUAL: u32 = 46;
pub const HID_KEY_BRACKET_LEFT: u32 = 47;
pub const HID_KEY_BRACKET_RIGHT: u32 = 48;
pub const HID_KEY_BACKSLASH: u32 = 49;
pub const HID_KEY_EUROPE_1: u32 = 50;
pub const HID_KEY_SEMICOLON: u32 = 51;
pub const HID_KEY_APOSTROPHE: u32 = 52;
pub const HID_KEY_GRAVE: u32 = 53;
pub const HID_KEY_COMMA: u32 = 54;
pub const HID_KEY_PERIOD: u32 = 55;
pub const HID_KEY_SLASH: u32 = 56;
pub const HID_KEY_CAPS_LOCK: u32 = 57;
pub const HID_KEY_F1: u32 = 58;
pub const HID_KEY_F2: u32 = 59;
pub const HID_KEY_F3: u32 = 60;
pub const HID_KEY_F4: u32 = 61;
pub const HID_KEY_F5: u32 = 62;
pub const HID_KEY_F6: u32 = 63;
pub const HID_KEY_F7: u32 = 64;
pub const HID_KEY_F8: u32 = 65;
pub const HID_KEY_F9: u32 = 66;
pub const HID_KEY_F10: u32 = 67;
pub const HID_KEY_F11: u32 = 68;
pub const HID_KEY_F12: u32 = 69;
pub const HID_KEY_PRINT_SCREEN: u32 = 70;
pub const HID_KEY_SCROLL_LOCK: u32 = 71;
pub const HID_KEY_PAUSE: u32 = 72;
pub const HID_KEY_INSERT: u32 = 73;
pub const HID_KEY_HOME: u32 = 74;
pub const HID_KEY_PAGE_UP: u32 = 75;
pub const HID_KEY_DELETE: u32 = 76;
pub const HID_KEY_END: u32 = 77;
pub const HID_KEY_PAGE_DOWN: u32 = 78;
pub const HID_KEY_ARROW_RIGHT: u32 = 79;
pub const HID_KEY_ARROW_LEFT: u32 = 80;
pub const HID_KEY_ARROW_DOWN: u32 = 81;
pub const HID_KEY_ARROW_UP: u32 = 82;
pub const HID_KEY_NUM_LOCK: u32 = 83;
pub const HID_KEY_KEYPAD_DIVIDE: u32 = 84;
pub const HID_KEY_KEYPAD_MULTIPLY: u32 = 85;
pub const HID_KEY_KEYPAD_SUBTRACT: u32 = 86;
pub const HID_KEY_KEYPAD_ADD: u32 = 87;
pub const HID_KEY_KEYPAD_ENTER: u32 = 88;
pub const HID_KEY_KEYPAD_1: u32 = 89;
pub const HID_KEY_KEYPAD_2: u32 = 90;
pub const HID_KEY_KEYPAD_3: u32 = 91;
pub const HID_KEY_KEYPAD_4: u32 = 92;
pub const HID_KEY_KEYPAD_5: u32 = 93;
pub const HID_KEY_KEYPAD_6: u32 = 94;
pub const HID_KEY_KEYPAD_7: u32 = 95;
pub const HID_KEY_KEYPAD_8: u32 = 96;
pub const HID_KEY_KEYPAD_9: u32 = 97;
pub const HID_KEY_KEYPAD_0: u32 = 98;
pub const HID_KEY_KEYPAD_DECIMAL: u32 = 99;
pub const HID_KEY_EUROPE_2: u32 = 100;
pub const HID_KEY_APPLICATION: u32 = 101;
pub const HID_KEY_POWER: u32 = 102;
pub const HID_KEY_KEYPAD_EQUAL: u32 = 103;
pub const HID_KEY_F13: u32 = 104;
pub const HID_KEY_F14: u32 = 105;
pub const HID_KEY_F15: u32 = 106;
pub const HID_KEY_CONTROL_LEFT: u32 = 224;
pub const HID_KEY_SHIFT_LEFT: u32 = 225;
pub const HID_KEY_ALT_LEFT: u32 = 226;
pub const HID_KEY_GUI_LEFT: u32 = 227;
pub const HID_KEY_CONTROL_RIGHT: u32 = 228;
pub const HID_KEY_SHIFT_RIGHT: u32 = 229;
pub const HID_KEY_ALT_RIGHT: u32 = 230;
pub const HID_KEY_GUI_RIGHT: u32 = 231;

pub const HID_DATA: u32 = 0;
pub const HID_CONSTANT: u32 = 1;
pub const HID_ARRAY: u32 = 0;
pub const HID_VARIABLE: u32 = 2;
pub const HID_ABSOLUTE: u32 = 0;
pub const HID_RELATIVE: u32 = 4;
pub const HID_WRAP_NO: u32 = 0;
pub const HID_WRAP: u32 = 8;
pub const HID_LINEAR: u32 = 0;
pub const HID_NONLINEAR: u32 = 16;
pub const HID_PREFERRED_STATE: u32 = 0;
pub const HID_PREFERRED_NO: u32 = 32;
pub const HID_NO_NULL_POSITION: u32 = 0;
pub const HID_NULL_STATE: u32 = 64;
pub const HID_NON_VOLATILE: u32 = 0;
pub const HID_VOLATILE: u32 = 128;
pub const HID_BITFIELD: u32 = 0;
pub const HID_BUFFERED_BYTES: u32 = 256;

//--------------------------------------------------------------------+
// Usage Table
//--------------------------------------------------------------------+

/// HID Usage Table - Table 1: Usage Page Summary
pub const HID_USAGE_PAGE_DESKTOP: u32 = 1;
pub const HID_USAGE_PAGE_SIMULATE: u32 = 2;
pub const HID_USAGE_PAGE_VIRTUAL_REALITY: u32 = 3;
pub const HID_USAGE_PAGE_SPORT: u32 = 4;
pub const HID_USAGE_PAGE_GAME: u32 = 5;
pub const HID_USAGE_PAGE_GENERIC_DEVICE: u32 = 6;
pub const HID_USAGE_PAGE_KEYBOARD: u32 = 7;
pub const HID_USAGE_PAGE_LED: u32 = 8;
pub const HID_USAGE_PAGE_BUTTON: u32 = 9;
pub const HID_USAGE_PAGE_ORDINAL: u32 = 10;
pub const HID_USAGE_PAGE_TELEPHONY: u32 = 11;
pub const HID_USAGE_PAGE_CONSUMER: u32 = 12;
pub const HID_USAGE_PAGE_DIGITIZER: u32 = 13;
pub const HID_USAGE_PAGE_PID: u32 = 15;
pub const HID_USAGE_PAGE_UNICODE: u32 = 16;
pub const HID_USAGE_PAGE_ALPHA_DISPLAY: u32 = 20;
pub const HID_USAGE_PAGE_MEDICAL: u32 = 64;
pub const HID_USAGE_PAGE_MONITOR: u32 = 128;
pub const HID_USAGE_PAGE_POWER: u32 = 132;
pub const HID_USAGE_PAGE_BARCODE_SCANNER: u32 = 140;
pub const HID_USAGE_PAGE_SCALE: u32 = 141;
pub const HID_USAGE_PAGE_MSR: u32 = 142;
pub const HID_USAGE_PAGE_CAMERA: u32 = 144;
pub const HID_USAGE_PAGE_ARCADE: u32 = 145;
pub const HID_USAGE_PAGE_VENDOR: u32 = 65280;

/// HID Usage Table - Table 6: Generic Desktop Page
pub const HID_USAGE_DESKTOP_POINTER: u32 = 1;
pub const HID_USAGE_DESKTOP_MOUSE: u32 = 2;
pub const HID_USAGE_DESKTOP_JOYSTICK: u32 = 4;
pub const HID_USAGE_DESKTOP_GAMEPAD: u32 = 5;
pub const HID_USAGE_DESKTOP_KEYBOARD: u32 = 6;
pub const HID_USAGE_DESKTOP_KEYPAD: u32 = 7;
pub const HID_USAGE_DESKTOP_MULTI_AXIS_CONTROLLER: u32 = 8;
pub const HID_USAGE_DESKTOP_TABLET_PC_SYSTEM: u32 = 9;
pub const HID_USAGE_DESKTOP_X: u32 = 48;
pub const HID_USAGE_DESKTOP_Y: u32 = 49;
pub const HID_USAGE_DESKTOP_Z: u32 = 50;
pub const HID_USAGE_DESKTOP_RX: u32 = 51;
pub const HID_USAGE_DESKTOP_RY: u32 = 52;
pub const HID_USAGE_DESKTOP_RZ: u32 = 53;
pub const HID_USAGE_DESKTOP_SLIDER: u32 = 54;
pub const HID_USAGE_DESKTOP_DIAL: u32 = 55;
pub const HID_USAGE_DESKTOP_WHEEL: u32 = 56;
pub const HID_USAGE_DESKTOP_HAT_SWITCH: u32 = 57;
pub const HID_USAGE_DESKTOP_COUNTED_BUFFER: u32 = 58;
pub const HID_USAGE_DESKTOP_BYTE_COUNT: u32 = 59;
pub const HID_USAGE_DESKTOP_MOTION_WAKEUP: u32 = 60;
pub const HID_USAGE_DESKTOP_START: u32 = 61;
pub const HID_USAGE_DESKTOP_SELECT: u32 = 62;
pub const HID_USAGE_DESKTOP_VX: u32 = 64;
pub const HID_USAGE_DESKTOP_VY: u32 = 65;
pub const HID_USAGE_DESKTOP_VZ: u32 = 66;
pub const HID_USAGE_DESKTOP_VBRX: u32 = 67;
pub const HID_USAGE_DESKTOP_VBRY: u32 = 68;
pub const HID_USAGE_DESKTOP_VBRZ: u32 = 69;
pub const HID_USAGE_DESKTOP_VNO: u32 = 70;
pub const HID_USAGE_DESKTOP_FEATURE_NOTIFICATION: u32 = 71;
pub const HID_USAGE_DESKTOP_RESOLUTION_MULTIPLIER: u32 = 72;
pub const HID_USAGE_DESKTOP_SYSTEM_CONTROL: u32 = 128;
pub const HID_USAGE_DESKTOP_SYSTEM_POWER_DOWN: u32 = 129;
pub const HID_USAGE_DESKTOP_SYSTEM_SLEEP: u32 = 130;
pub const HID_USAGE_DESKTOP_SYSTEM_WAKE_UP: u32 = 131;
pub const HID_USAGE_DESKTOP_SYSTEM_CONTEXT_MENU: u32 = 132;
pub const HID_USAGE_DESKTOP_SYSTEM_MAIN_MENU: u32 = 133;
pub const HID_USAGE_DESKTOP_SYSTEM_APP_MENU: u32 = 134;
pub const HID_USAGE_DESKTOP_SYSTEM_MENU_HELP: u32 = 135;
pub const HID_USAGE_DESKTOP_SYSTEM_MENU_EXIT: u32 = 136;
pub const HID_USAGE_DESKTOP_SYSTEM_MENU_SELECT: u32 = 137;
pub const HID_USAGE_DESKTOP_SYSTEM_MENU_RIGHT: u32 = 138;
pub const HID_USAGE_DESKTOP_SYSTEM_MENU_LEFT: u32 = 139;
pub const HID_USAGE_DESKTOP_SYSTEM_MENU_UP: u32 = 140;
pub const HID_USAGE_DESKTOP_SYSTEM_MENU_DOWN: u32 = 141;
pub const HID_USAGE_DESKTOP_SYSTEM_COLD_RESTART: u32 = 142;
pub const HID_USAGE_DESKTOP_SYSTEM_WARM_RESTART: u32 = 143;
pub const HID_USAGE_DESKTOP_DPAD_UP: u32 = 144;
pub const HID_USAGE_DESKTOP_DPAD_DOWN: u32 = 145;
pub const HID_USAGE_DESKTOP_DPAD_RIGHT: u32 = 146;
pub const HID_USAGE_DESKTOP_DPAD_LEFT: u32 = 147;
pub const HID_USAGE_DESKTOP_SYSTEM_DOCK: u32 = 160;
pub const HID_USAGE_DESKTOP_SYSTEM_UNDOCK: u32 = 161;
pub const HID_USAGE_DESKTOP_SYSTEM_SETUP: u32 = 162;
pub const HID_USAGE_DESKTOP_SYSTEM_BREAK: u32 = 163;
pub const HID_USAGE_DESKTOP_SYSTEM_DEBUGGER_BREAK: u32 = 164;
pub const HID_USAGE_DESKTOP_APPLICATION_BREAK: u32 = 165;
pub const HID_USAGE_DESKTOP_APPLICATION_DEBUGGER_BREAK: u32 = 166;
pub const HID_USAGE_DESKTOP_SYSTEM_SPEAKER_MUTE: u32 = 167;
pub const HID_USAGE_DESKTOP_SYSTEM_HIBERNATE: u32 = 168;
pub const HID_USAGE_DESKTOP_SYSTEM_DISPLAY_INVERT: u32 = 176;
pub const HID_USAGE_DESKTOP_SYSTEM_DISPLAY_INTERNAL: u32 = 177;
pub const HID_USAGE_DESKTOP_SYSTEM_DISPLAY_EXTERNAL: u32 = 178;
pub const HID_USAGE_DESKTOP_SYSTEM_DISPLAY_BOTH: u32 = 179;
pub const HID_USAGE_DESKTOP_SYSTEM_DISPLAY_DUAL: u32 = 180;
pub const HID_USAGE_DESKTOP_SYSTEM_DISPLAY_TOGGLE_INT_EXT: u32 = 181;
pub const HID_USAGE_DESKTOP_SYSTEM_DISPLAY_SWAP_PRIMARY_SECONDARY: u32 = 182;
pub const HID_USAGE_DESKTOP_SYSTEM_DISPLAY_LCD_AUTOSCALE: u32 = 183;

/// HID Usage Table: Consumer Page (0x0C)
/// Only contains controls that supported by Windows (whole list is too long)
pub const HID_USAGE_CONSUMER_CONTROL: u32 = 1;
pub const HID_USAGE_CONSUMER_POWER: u32 = 48;
pub const HID_USAGE_CONSUMER_RESET: u32 = 49;
pub const HID_USAGE_CONSUMER_SLEEP: u32 = 50;
pub const HID_USAGE_CONSUMER_BRIGHTNESS_INCREMENT: u32 = 111;
pub const HID_USAGE_CONSUMER_BRIGHTNESS_DECREMENT: u32 = 112;
pub const HID_USAGE_CONSUMER_WIRELESS_RADIO_CONTROLS: u32 = 12;
pub const HID_USAGE_CONSUMER_WIRELESS_RADIO_BUTTONS: u32 = 198;
pub const HID_USAGE_CONSUMER_WIRELESS_RADIO_LED: u32 = 199;
pub const HID_USAGE_CONSUMER_WIRELESS_RADIO_SLIDER_SWITCH: u32 = 200;
pub const HID_USAGE_CONSUMER_PLAY_PAUSE: u32 = 205;
pub const HID_USAGE_CONSUMER_SCAN_NEXT: u32 = 181;
pub const HID_USAGE_CONSUMER_SCAN_PREVIOUS: u32 = 182;
pub const HID_USAGE_CONSUMER_STOP: u32 = 183;
pub const HID_USAGE_CONSUMER_VOLUME: u32 = 224;
pub const HID_USAGE_CONSUMER_MUTE: u32 = 226;
pub const HID_USAGE_CONSUMER_BASS: u32 = 227;
pub const HID_USAGE_CONSUMER_TREBLE: u32 = 228;
pub const HID_USAGE_CONSUMER_BASS_BOOST: u32 = 229;
pub const HID_USAGE_CONSUMER_VOLUME_INCREMENT: u32 = 233;
pub const HID_USAGE_CONSUMER_VOLUME_DECREMENT: u32 = 234;
pub const HID_USAGE_CONSUMER_BASS_INCREMENT: u32 = 338;
pub const HID_USAGE_CONSUMER_BASS_DECREMENT: u32 = 339;
pub const HID_USAGE_CONSUMER_TREBLE_INCREMENT: u32 = 340;
pub const HID_USAGE_CONSUMER_TREBLE_DECREMENT: u32 = 341;
pub const HID_USAGE_CONSUMER_AL_CONSUMER_CONTROL_CONFIGURATION: u32 = 387;
pub const HID_USAGE_CONSUMER_AL_EMAIL_READER: u32 = 394;
pub const HID_USAGE_CONSUMER_AL_CALCULATOR: u32 = 402;
pub const HID_USAGE_CONSUMER_AL_LOCAL_BROWSER: u32 = 404;
pub const HID_USAGE_CONSUMER_AC_SEARCH: u32 = 545;
pub const HID_USAGE_CONSUMER_AC_HOME: u32 = 547;
pub const HID_USAGE_CONSUMER_AC_BACK: u32 = 548;
pub const HID_USAGE_CONSUMER_AC_FORWARD: u32 = 549;
pub const HID_USAGE_CONSUMER_AC_STOP: u32 = 550;
pub const HID_USAGE_CONSUMER_AC_REFRESH: u32 = 551;
pub const HID_USAGE_CONSUMER_AC_BOOKMARKS: u32 = 554;
pub const HID_USAGE_CONSUMER_AC_PAN: u32 = 568;

//------------- COLLECTION ITEM 6.2.2.6 -------------//
pub const HID_COLLECTION_PHYSICAL: u32 = 0;
pub const HID_COLLECTION_APPLICATION: u32 = 1;
pub const HID_COLLECTION_LOGICAL: u32 = 2;
pub const HID_COLLECTION_REPORT: u32 = 3;
pub const HID_COLLECTION_NAMED_ARRAY: u32 = 4;
pub const HID_COLLECTION_USAGE_SWITCH: u32 = 5;
pub const HID_COLLECTION_USAGE_MODIFIER: u32 = 6;

pub const REPORT_ID_KEYBOARD: u32 = 1;
pub const REPORT_ID_CONSUMER_CONTROL: u32 = 2;
pub const REPORT_ID_MOUSE: u32 = 3;
pub const REPORT_ID_GAMEPAD: u32 = 4;


pub const HID_REPORT_MAP: [u32; 88] = [
  //------------- Keyboard Report  -------------//
  HID_USAGE_PAGE!  ( HID_USAGE_PAGE_DESKTOP     ), HID_USAGE_PAGE_DESKTOP     ,
  HID_USAGE!       ( HID_USAGE_DESKTOP_KEYBOARD ), HID_USAGE_DESKTOP_KEYBOARD ,
  HID_COLLECTION!  ( HID_COLLECTION_APPLICATION ), HID_COLLECTION_APPLICATION ,
    HID_REPORT_ID! ( REPORT_ID_KEYBOARD         ), REPORT_ID_KEYBOARD         ,
    HID_USAGE_PAGE!( HID_USAGE_PAGE_KEYBOARD    ), HID_USAGE_PAGE_KEYBOARD    ,
      // 8 bits Modifier Keys (Shfit, Control, Alt)
      HID_USAGE_MIN!    ( 224                                    ), 224                                    ,
      HID_USAGE_MAX!    ( 231                                    ), 231                                    ,
      HID_LOGICAL_MIN!  ( 0                                      ), 0                                      ,
      HID_LOGICAL_MAX!  ( 1                                      ), 1                                      ,

      HID_REPORT_COUNT! ( 8                                      ), 8                                      ,
      HID_REPORT_SIZE!  ( 1                                      ), 1                                      ,
      HID_INPUT!        ( HID_DATA | HID_VARIABLE | HID_ABSOLUTE ), HID_DATA | HID_VARIABLE | HID_ABSOLUTE ,

      // 8 bit reserved
      HID_REPORT_COUNT! ( 1                                      ), 1                                      ,
      HID_REPORT_SIZE!  ( 8                                      ), 8                                      ,
      HID_INPUT!        ( HID_CONSTANT                           ), HID_CONSTANT                           ,

    // 6-byte Keycodes
    HID_USAGE_PAGE! (HID_USAGE_PAGE_KEYBOARD),HID_USAGE_PAGE_KEYBOARD,
      HID_USAGE_MIN!    ( 0                                   ), 0                                   ,
      HID_USAGE_MAX!    ( 255                                 ), 255                                 ,
      HID_LOGICAL_MIN!  ( 0                                   ), 0                                   ,
      HID_LOGICAL_MAX!  ( 255                                 ), 255                                 ,

      HID_REPORT_COUNT! ( 6                                   ), 6                                   ,
      HID_REPORT_SIZE!  ( 8                                   ), 8                                   ,
      HID_INPUT!        ( HID_DATA | HID_ARRAY | HID_ABSOLUTE ), HID_DATA | HID_ARRAY | HID_ABSOLUTE ,

    // LED Indicator Kana | Compose | Scroll Lock | CapsLock | NumLock
    HID_USAGE_PAGE!  ( HID_USAGE_PAGE_LED                   ), HID_USAGE_PAGE_LED                   ,
      /* 5-bit Led report */
      HID_USAGE_MIN!    ( 1                                       ), 1                                       ,
      HID_USAGE_MAX!    ( 5                                       ), 5                                       ,
      HID_REPORT_COUNT! ( 5                                       ), 5                                       ,
      HID_REPORT_SIZE!  ( 1                                       ), 1                                       ,
      HID_OUTPUT!       ( HID_DATA | HID_VARIABLE | HID_ABSOLUTE  ), HID_DATA | HID_VARIABLE | HID_ABSOLUTE  ,
      /* led padding */
      HID_REPORT_COUNT! ( 1                                       ), 1                                       ,
      HID_REPORT_SIZE!  ( 3                                       ), 3                                       ,
      HID_OUTPUT!       ( HID_CONSTANT                            ), HID_CONSTANT                            ,
  HID_COLLECTION_END!(),

  //------------- Consumer Control Report -------------//
  HID_USAGE_PAGE! ( HID_USAGE_PAGE_CONSUMER    ), HID_USAGE_PAGE_CONSUMER    ,
  HID_USAGE!      ( HID_USAGE_CONSUMER_CONTROL ), HID_USAGE_CONSUMER_CONTROL ,
  HID_COLLECTION! ( HID_COLLECTION_APPLICATION ), HID_COLLECTION_APPLICATION ,
    HID_REPORT_ID!( REPORT_ID_CONSUMER_CONTROL ), REPORT_ID_CONSUMER_CONTROL ,
    HID_LOGICAL_MIN!  ( 0x00                                ), 0x00                                ,
    HID_LOGICAL_MAX_N!( 0x03FF, 2                           ), 0x03FF,
    HID_USAGE_MIN!    ( 0x00                                ), 0x00                                ,
    HID_USAGE_MAX_N!  ( 0x03FF, 2                           ), 0x03FF,
    HID_REPORT_COUNT! ( 1                                   ), 1                                   ,
    HID_REPORT_SIZE!  ( 16                                  ), 16                                  ,
    HID_INPUT!        ( HID_DATA | HID_ARRAY | HID_ABSOLUTE ), HID_DATA | HID_ARRAY | HID_ABSOLUTE ,
  HID_COLLECTION_END!(),
];
