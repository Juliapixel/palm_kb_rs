use crate::key_codes::KeyCode as Kc;

/// The coordinates of keys on the physical key matrix of the keyboard, as shown
/// [here](https://www.splorp.com/pdf/stowawayhwref.pdf) on page 18
pub const MATRIX: [Option<Kc>; 90] = [
    // Y0
    Some(Kc::Keyboard1AndExclamation),
    Some(Kc::Keyboard2AndAt),
    Some(Kc::Keyboard3AndSharp),
    Some(Kc::KeyboardZ),
    Some(Kc::Keyboard4AndDollarSign),
    Some(Kc::Keyboard5AndPercent),
    Some(Kc::Keyboard6AndCaret),
    Some(Kc::Keyboard7AndAmpersand),

    // Y1
    Some(Kc::KeyboardLeftGui),
    Some(Kc::KeyboardQ),
    Some(Kc::KeyboardW),
    Some(Kc::KeyboardE),
    Some(Kc::KeyboardR),
    Some(Kc::KeyboardT),
    Some(Kc::KeyboardY),
    Some(Kc::KeyboardGraveAccentAndTilde),

    // Y2
    Some(Kc::KeyboardX),
    Some(Kc::KeyboardA),
    Some(Kc::KeyboardS),
    Some(Kc::KeyboardD),
    Some(Kc::KeyboardF),
    Some(Kc::KeyboardG),
    Some(Kc::KeyboardH),
    Some(Kc::KeyboardSpacebar),

    // Y3
    Some(Kc::KeyboardCapsLock),
    Some(Kc::KeyboardTab),
    Some(Kc::KeyboardLeftControl),
    None,
    None,
    None,
    None,
    None,

    // Y4
    None,
    None,
    Some(Kc::KeyBoardNoKey), // Fn key
    Some(Kc::KeyboardLeftAlt),
    None,
    None,
    None,
    None,

    // Y5
    None,
    None,
    None,
    None,
    Some(Kc::KeyboardC),
    Some(Kc::KeyboardV),
    Some(Kc::KeyboardB),
    Some(Kc::KeyboardN),

    // Y6
    Some(Kc::KeyboardMinusAndUnderscore),
    Some(Kc::KeyboardEqualsAndPlus),
    Some(Kc::KeyboardBackspace),
    Some(Kc::KeyBoardNoKey), // special function one
    Some(Kc::Keyboard8AndAsterisk),
    Some(Kc::Keyboard9AndRightParentheses),
    Some(Kc::Keyboard0AndLeftParentheses),
    Some(Kc::KeyboardSpacebar),

    // Y7
    Some(Kc::KeyboardLeftSquareBracketAndCurlyBracket),
    Some(Kc::KeyboardRightSquareBracketAndCurlyBracket),
    Some(Kc::KeyboardBackslashAndPipe),
    Some(Kc::KeyBoardNoKey), // special function 2
    Some(Kc::KeyboardU),
    Some(Kc::KeyboardI),
    Some(Kc::KeyboardO),
    Some(Kc::KeyboardP),

    // Y8

    Some(Kc::KeyboardSingleAndDoubleQuotes),
    Some(Kc::KeyboardEnter),
    Some(Kc::KeyBoardNoKey), // special function 3
    None,
    Some(Kc::KeyboardJ),
    Some(Kc::KeyboardK),
    Some(Kc::KeyboardL),
    Some(Kc::KeyboardSemicolonAndColon),

    // Y9
    Some(Kc::KeyboardSlashAndQuestionMark),
    Some(Kc::KeyboardUpArrow),
    Some(Kc::KeyBoardNoKey), // special function 4
    None,
    Some(Kc::KeyboardM),
    Some(Kc::KeyboardCommaAndLessThan),
    Some(Kc::KeyboardPeriodAndGreaterThan),
    Some(Kc::KeyboardEnter), // matrix says DONE but idk wtf that is

    // Y10
    Some(Kc::KeyboardDelete),
    Some(Kc::KeyboardLeftArrow),
    Some(Kc::KeyboardDownArrow),
    Some(Kc::KeyboardRightArrow),
    None,
    None,
    None,
    None,

    // Y11
    Some(Kc::KeyboardLeftShift),
    Some(Kc::KeyboardRightShift)
];
