use crate::palm_kb::matrix::MATRIX;

/// USB-HID key codes
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum KeyCode {
    /// Keyboard no key
    #[default]
    KeyBoardNoKey = 0,
    /// Keyboard ErrorRollOver
    KeyboardErrorRollOver = 1,
    /// Keyboard POSTFail
    KeyboardPostFail = 2,
    /// Keyboard ErrorUndefined
    KeyboardErrorUndefined = 3,
    /// Keyboard a and A
    KeyboardA = 4,
    /// Keyboard b and B
    KeyboardB = 5,
    /// Keyboard c and C
    KeyboardC = 6,
    /// Keyboard d and D
    KeyboardD = 7,
    /// Keyboard e and E
    KeyboardE = 8,
    /// Keyboard f and F
    KeyboardF = 9,
    /// Keyboard g and G
    KeyboardG = 10,
    /// Keyboard h and H
    KeyboardH = 11,
    /// Keyboard i and I
    KeyboardI = 12,
    /// Keyboard j and J
    KeyboardJ = 13,
    /// Keyboard k and K
    KeyboardK = 14,
    /// Keyboard l and L
    KeyboardL = 15,
    /// Keyboard m and M
    KeyboardM = 16,
    /// Keyboard n and N
    KeyboardN = 17,
    /// Keyboard o and O
    KeyboardO = 18,
    /// Keyboard p and P
    KeyboardP = 19,
    /// Keyboard q and Q
    KeyboardQ = 20,
    /// Keyboard r and R
    KeyboardR = 21,
    /// Keyboard s and S
    KeyboardS = 22,
    /// Keyboard t and T
    KeyboardT = 23,
    /// Keyboard u and U
    KeyboardU = 24,
    /// Keyboard v and V
    KeyboardV = 25,
    /// Keyboard w and W
    KeyboardW = 26,
    /// Keyboard x and X
    KeyboardX = 27,
    /// Keyboard y and Y
    KeyboardY = 28,
    /// Keyboard z and Z
    KeyboardZ = 29,
    /// Keyboard 1 and !
    Keyboard1AndExclamation = 30,
    /// Keyboard 2 and @
    Keyboard2AndAt = 31,
    /// Keyboard 3 and #
    Keyboard3AndSharp = 32,
    /// Keyboard 4 and $
    Keyboard4AndDollarSign = 33,
    /// Keyboard 5 and %
    Keyboard5AndPercent = 34,
    /// Keyboard 6 and ∧
    Keyboard6AndCaret = 35,
    /// Keyboard 7 and &
    Keyboard7AndAmpersand = 36,
    /// Keyboard 8 and *
    Keyboard8AndAsterisk = 37,
    /// Keyboard 9 and (
    Keyboard9AndRightParentheses = 38,
    /// Keyboard 0 and )
    Keyboard0AndLeftParentheses = 39,
    /// Keyboard Return (ENTER)
    KeyboardEnter = 40,
    /// Keyboard ESCAPE
    KeyboardEscape = 41,
    /// Keyboard DELETE (Backspace)
    KeyboardBackspace = 42,
    /// Keyboard Tab
    KeyboardTab = 43,
    /// Keyboard Spacebar
    KeyboardSpacebar = 44,
    /// Keyboard - and (underscore)
    KeyboardMinusAndUnderscore = 45,
    /// Keyboard = and +
    KeyboardEqualsAndPlus = 46,
    /// Keyboard [ and {
    KeyboardLeftSquareBracketAndCurlyBracket = 47,
    /// Keyboard ] and }
    KeyboardRightSquareBracketAndCurlyBracket = 48,
    /// Keyboard \ and |
    KeyboardBackslashAndPipe = 49,
    /// Keyboard Non-US # and ̃
    KeyboardNonUsSharpAnd̃Tilde = 50,
    /// Keyboard ; and :
    KeyboardSemicolonAndColon = 51,
    /// Keyboard ' and "
    KeyboardSingleAndDoubleQuotes = 52,
    /// Keyboard Grave Accent and Tilde
    KeyboardGraveAccentAndTilde = 53,
    /// Keyboard , and <
    KeyboardCommaAndLessThan = 54,
    /// Keyboard . and >
    KeyboardPeriodAndGreaterThan = 55,
    /// Keyboard / and ?
    KeyboardSlashAndQuestionMark = 56,
    /// Keyboard Caps Lock
    KeyboardCapsLock = 57,
    /// Keyboard F1
    KeyboardF1 = 58,
    /// Keyboard F2
    KeyboardF2 = 59,
    /// Keyboard F3
    KeyboardF3 = 60,
    /// Keyboard F4
    KeyboardF4 = 61,
    /// Keyboard F5
    KeyboardF5 = 62,
    /// Keyboard F6
    KeyboardF6 = 63,
    /// Keyboard F7
    KeyboardF7 = 64,
    /// Keyboard F8
    KeyboardF8 = 65,
    /// Keyboard F9
    KeyboardF9 = 66,
    /// Keyboard F10
    KeyboardF10 = 67,
    /// Keyboard F11
    KeyboardF11 = 68,
    /// Keyboard F12
    KeyboardF12 = 69,
    /// Keyboard PrintScreen
    KeyboardPrintScreen = 70,
    /// Keyboard Scroll Lock
    KeyboardScrollLock = 71,
    /// Keyboard Pause
    KeyboardPause = 72,
    /// Keyboard Insert
    KeyboardInsert = 73,
    /// Keyboard Home
    KeyboardHome = 74,
    /// Keyboard PageUp
    KeyboardPageUp = 75,
    /// Keyboard Delete Forward
    KeyboardDelete = 76,
    /// Keyboard End
    KeyboardEnd = 77,
    /// Keyboard PageDown
    KeyboardPageDown = 78,
    /// Keyboard RightArrow
    KeyboardRightArrow = 79,
    /// Keyboard LeftArrow
    KeyboardLeftArrow = 80,
    /// Keyboard DownArrow
    KeyboardDownArrow = 81,
    /// Keyboard UpArrow
    KeyboardUpArrow = 82,
    /// Keypad Num Lock and Clear
    KeypadNumLockAndClear = 83,
    /// Keypad /
    KeypadSlash = 84,
    /// Keypad *
    KeypadAsterisk = 85,
    /// Keypad -
    KeypadMinus = 86,
    /// Keypad +
    KeypadPlus = 87,
    /// Keypad ENTER
    KeypadEnter = 88,
    /// Keypad 1 and End
    Keypad1AndEnd = 89,
    /// Keypad 2 and Down Arrow
    Keypad2AndDownArrow = 90,
    /// Keypad 3 and PageDn
    Keypad3AndPageDn = 91,
    /// Keypad 4 and Left Arrow
    Keypad4AndLeftArrow = 92,
    /// Keypad 5
    Keypad5 = 93,
    /// Keypad 6 and Right Arrow
    Keypad6AndRightArrow = 94,
    /// Keypad 7 and Home
    Keypad7AndHome = 95,
    /// Keypad 8 and Up Arrow
    Keypad8AndUpArrow = 96,
    /// Keypad 9 and PageUp
    Keypad9AndPageUp = 97,
    /// Keypad 0 and Insert
    Keypad0AndInsert = 98,
    /// Keypad . and Delete
    KeypadPeriodAndDelete = 99,
    /// Keyboard Non-US \ and |
    KeyboardNonUsBackslashAndPipe = 100,
    /// Keyboard Application
    KeyboardApplication = 101,
    /// Keyboard Power
    KeyboardPower = 102,
    /// Keypad =
    KeypadEquals = 103,
    /// Keyboard F13
    KeyboardF13 = 104,
    /// Keyboard F14
    KeyboardF14 = 105,
    /// Keyboard F15
    KeyboardF15 = 106,
    /// Keyboard F16
    KeyboardF16 = 107,
    /// Keyboard F17
    KeyboardF17 = 108,
    /// Keyboard F18
    KeyboardF18 = 109,
    /// Keyboard F19
    KeyboardF19 = 110,
    /// Keyboard F20
    KeyboardF20 = 111,
    /// Keyboard F21
    KeyboardF21 = 112,
    /// Keyboard F22
    KeyboardF22 = 113,
    /// Keyboard F23
    KeyboardF23 = 114,
    /// Keyboard F24
    KeyboardF24 = 115,
    /// Keyboard Execute
    KeyboardExecute = 116,
    /// Keyboard Help
    KeyboardHelp = 117,
    /// Keyboard Menu
    KeyboardMenu = 118,
    /// Keyboard Select
    KeyboardSelect = 119,
    /// Keyboard Stop
    KeyboardStop = 120,
    /// Keyboard Again
    KeyboardAgain = 121,
    /// Keyboard Undo
    KeyboardUndo = 122,
    /// Keyboard Cut
    KeyboardCut = 123,
    /// Keyboard Copy
    KeyboardCopy = 124,
    /// Keyboard Paste
    KeyboardPaste = 125,
    /// Keyboard Find
    KeyboardFind = 126,
    /// Keyboard Mute
    KeyboardMute = 127,
    /// Keyboard Volume Up
    KeyboardVolumeUp = 128,
    /// Keyboard Volume Down
    KeyboardVolumeDown = 129,
    /// Keyboard Locking Caps Lock
    KeyboardLockingCapsLock = 130,
    /// Keyboard Locking Num Lock
    KeyboardLockingNumLock = 131,
    /// Keyboard Locking Scroll Lock
    KeyboardLockingScrollLock = 132,
    /// Keypad Comma
    KeypadComma = 133,
    /// Keypad Equal Sign
    KeypadEqualSign = 134,
    /// Keyboard International1
    KeyboardInternational1 = 135,
    /// Keyboard International2
    KeyboardInternational2 = 136,
    /// Keyboard International3
    KeyboardInternational3 = 137,
    /// Keyboard International4
    KeyboardInternational4 = 138,
    /// Keyboard International5
    KeyboardInternational5 = 139,
    /// Keyboard International6
    KeyboardInternational6 = 140,
    /// Keyboard International7
    KeyboardInternational7 = 141,
    /// Keyboard International8
    KeyboardInternational8 = 142,
    /// Keyboard International9
    KeyboardInternational9 = 143,
    /// Keyboard LANG1
    KeyboardLang1 = 144,
    /// Keyboard LANG2
    KeyboardLang2 = 145,
    /// Keyboard LANG3
    KeyboardLang3 = 146,
    /// Keyboard LANG4
    KeyboardLang4 = 147,
    /// Keyboard LANG5
    KeyboardLang5 = 148,
    /// Keyboard LANG6
    KeyboardLang6 = 149,
    /// Keyboard LANG7
    KeyboardLang7 = 150,
    /// Keyboard LANG8
    KeyboardLang8 = 151,
    /// Keyboard LANG9
    KeyboardLang9 = 152,
    /// Keyboard Alternate Erase
    KeyboardAlternateErase = 153,
    /// Keyboard SysReq/Attention
    KeyboardSysReqAttention = 154,
    /// Keyboard Cancel
    KeyboardCancel = 155,
    /// Keyboard Clear
    KeyboardClear = 156,
    /// Keyboard Prior
    KeyboardPrior = 157,
    /// Keyboard Return
    KeyboardReturn = 158,
    /// Keyboard Separator
    KeyboardSeparator = 159,
    /// Keyboard Out
    KeyboardOut = 160,
    /// Keyboard Oper
    KeyboardOper = 161,
    /// Keyboard Clear/Again
    KeyboardClearAgain = 162,
    /// Keyboard CrSel/Props
    KeyboardCrSelProps = 163,
    /// Keyboard ExSel
    KeyboardExSel = 164,
    /// Keypad 00
    Keypad00 = 176,
    /// Keypad 000
    Keypad000 = 177,
    /// Thousands Separator
    ThousandsSeparator = 178,
    /// Decimal Separator
    DecimalSeparator = 179,
    /// Currency Unit
    CurrencyUnit = 180,
    /// Currency Sub-unit
    CurrencySubUnit = 181,
    /// Keypad (
    KeypadRightParentheses = 182,
    /// Keypad )
    KeypadLeftParentheses = 183,
    /// Keypad {
    KeypadLeftCurlyBracket = 184,
    /// Keypad }
    KeypadRightCurlyBracket = 185,
    /// Keypad Tab
    KeypadTab = 186,
    /// Keypad Backspace
    KeypadBackspace = 187,
    /// Keypad A
    KeypadA = 188,
    /// Keypad B
    KeypadB = 189,
    /// Keypad C
    KeypadC = 190,
    /// Keypad D
    KeypadD = 191,
    /// Keypad E
    KeypadE = 192,
    /// Keypad F
    KeypadF = 193,
    /// Keypad XOR
    KeypadXOR = 194,
    /// Keypad ∧
    KeypadCaret = 195,
    /// Keypad %
    KeypadPercent = 196,
    /// Keypad <
    KeypadLessThan = 197,
    /// Keypad >
    KeypadGreaterThan = 198,
    /// Keypad &
    KeypadAmpersand = 199,
    /// Keypad &&
    KeypadDoubleAmpersand = 200,
    /// Keypad |
    KeypadPipe = 201,
    /// Keypad ||
    KeypadDoublePipe = 202,
    /// Keypad :
    KeypadColon = 203,
    /// Keypad #
    KeypadSharp = 204,
    /// Keypad Space
    KeypadSpace = 205,
    /// Keypad @
    KeypadAt = 206,
    /// Keypad !
    KeypadExclamation = 207,
    /// Keypad Memory Store
    KeypadMemoryStore = 208,
    /// Keypad Memory Recall
    KeypadMemoryRecall = 209,
    /// Keypad Memory Clear
    KeypadMemoryClear = 210,
    /// Keypad Memory Add
    KeypadMemoryAdd = 211,
    /// Keypad Memory Subtract
    KeypadMemorySubtract = 212,
    /// Keypad Memory Multiply
    KeypadMemoryMultiply = 213,
    /// Keypad Memory Divide
    KeypadMemoryDivide = 214,
    /// Keypad +/-
    KeypadPlusMinus = 215,
    /// Keypad Clear
    KeypadClear = 216,
    /// Keypad Clear Entry
    KeypadClearEntry = 217,
    /// Keypad Binary
    KeypadBinary = 218,
    /// Keypad Octal
    KeypadOctal = 219,
    /// Keypad Decimal
    KeypadDecimal = 220,
    /// Keypad Hexadecimal
    KeypadHexadecimal = 221,
    /// Keyboard LeftControl
    KeyboardLeftControl = 224,
    /// Keyboard LeftShift
    KeyboardLeftShift = 225,
    /// Keyboard LeftAlt
    KeyboardLeftAlt = 226,
    /// Keyboard Left GUI
    /// A.K.A. Meta or Super or Windows or CMD
    KeyboardLeftGui = 227,
    /// Keyboard RightControl
    KeyboardRightControl = 228,
    /// Keyboard RightShift
    KeyboardRightShift = 229,
    /// Keyboard RightAlt
    KeyboardRightAlt = 230,
    /// Keyboard Right GUI
    /// A.K.A. Meta or Super or Windows or CMD
    KeyboardRightGui = 231
}

bitflags::bitflags! {
    /// Keyboard modifiers
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    pub struct Modifiers: u8 {
        const LEFT_CTRL   = 0b0000_0001;
        const LEFT_SHIFT  = 0b0000_0010;
        const LEFT_ALT    = 0b0000_0100;
        const LEFT_META   = 0b0000_1000;
        const RIGHT_CTRL  = 0b0001_0000;
        const RIGHT_SHIFT = 0b0010_0000;
        const RIGHT_ALT   = 0b0100_0000;
        const RIGHT_META  = 0b1000_0000;
    }
}

impl KeyCode {
    #[inline]
    pub fn try_from_matrix_key(key: u8) -> Option<Self> {
        let pos = key & 0b0111_1111;
        if let Some(Some(val)) = MATRIX.get(pos as usize) {
            Some(*val)
        } else {
            None
        }
    }
}

impl From<KeyCode> for Modifiers {
    #[inline]
    fn from(value: KeyCode) -> Self {
        match value {
            KeyCode::KeyboardLeftShift => Self::LEFT_SHIFT,
            KeyCode::KeyboardRightShift => Self::RIGHT_SHIFT,
            KeyCode::KeyboardLeftAlt => Self::LEFT_ALT,
            KeyCode::KeyboardRightAlt => Self::RIGHT_ALT,
            KeyCode::KeyboardLeftControl => Self::LEFT_CTRL,
            KeyCode::KeyboardRightControl => Self::RIGHT_CTRL,
            KeyCode::KeyboardLeftGui => Self::LEFT_META,
            KeyCode::KeyboardRightGui => Self::RIGHT_META,
            _ => Self::empty()
        }
    }
}
