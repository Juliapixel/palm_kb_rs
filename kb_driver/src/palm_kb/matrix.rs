use crate::key_codes::KeyCode as Kc;

/// The coordinates of keys on the physical key matrix of the keyboard, as shown
/// [here](https://www.splorp.com/pdf/stowawayhwref.pdf) on page 18
///
/// |     | X0        | X1         | X2          | X3          | X4 | X5 | X6 | X7      |
/// |-----|-----------|------------|-------------|-------------|----|----|----|---------|
/// | Y0  | 1         | 2          | 3           | Z           | 4  | 5  | 6  | 7       |
/// | Y1  | CMD       | Q          | W           | E           | R  | T  | Y  | `       |
/// | Y2  | X         | A          | S           | D           | F  | G  | H  | SPACE 1 |
/// | Y3  | CAPS LOCK | TAB        | CTRL        |             |    |    |    |         |
/// | Y4  |           |            | FN          | ALT         |    |    |    |         |
/// | Y5  |           |            |             |             | C  | V  | B  | N       |
/// | Y6  | -         | =          | BACKSPACE   | SPECIAL FN1 | 8  | 9  | 0  | SPACE 2 |
/// | Y7  | [         | ]          | \           | SPECIAL FN2 | U  | I  | O  | P       |
/// | Y8  | '         | ENTER      | SPECIAL FN3 |             | J  | K  | L  | ;       |
/// | Y9  | /         | UP ARROW   | SPECIAL FN4 |             | M  | ,  | .  | DONE    |
/// | Y10 | DEL       | LEFT ARROW | DOWN ARROW  | RIGHT ARROW |    |    |    |         |
/// | Y11 | LSHIFT    | RSHIFT     |             |             |    |    |    |         |
pub const MATRIX: [Option<(Kc, Option<Kc>)>; 90] = [
    // Y0
    Some((Kc::Keyboard1AndExclamation, None)),
    Some((Kc::Keyboard2AndAt, None)),
    Some((Kc::Keyboard3AndSharp, None)),
    Some((Kc::KeyboardZ, None)),
    Some((Kc::Keyboard4AndDollarSign, None)),
    Some((Kc::Keyboard5AndPercent, None)),
    Some((Kc::Keyboard6AndCaret, None)),
    Some((Kc::Keyboard7AndAmpersand, None)),
    // Y1
    Some((Kc::KeyboardLeftGui, None)),
    Some((Kc::KeyboardQ, None)),
    Some((Kc::KeyboardW, None)),
    Some((Kc::KeyboardE, None)),
    Some((Kc::KeyboardR, None)),
    Some((Kc::KeyboardT, None)),
    Some((Kc::KeyboardY, None)),
    Some((Kc::KeyboardGraveAccentAndTilde, None)),
    // Y2
    Some((Kc::KeyboardX, None)),
    Some((Kc::KeyboardA, None)),
    Some((Kc::KeyboardS, None)),
    Some((Kc::KeyboardD, None)),
    Some((Kc::KeyboardF, None)),
    Some((Kc::KeyboardG, None)),
    Some((Kc::KeyboardH, None)),
    Some((Kc::KeyboardSpacebar, None)),
    // Y3
    Some((Kc::KeyboardCapsLock, None)),
    Some((Kc::KeyboardTab, Some(Kc::KeyboardEscape))),
    Some((Kc::KeyboardLeftControl, None)),
    None,
    None,
    None,
    None,
    None,
    // Y4
    None,
    None,
    Some((Kc::KeyboardFn, None)), // Fn key
    Some((Kc::KeyboardLeftAlt, None)),
    None,
    None,
    None,
    None,
    // Y5
    None,
    None,
    None,
    None,
    Some((Kc::KeyboardC, None)),
    Some((Kc::KeyboardV, None)),
    Some((Kc::KeyboardB, None)),
    Some((Kc::KeyboardN, None)),
    // Y6
    Some((Kc::KeyboardMinusAndUnderscore, None)),
    Some((Kc::KeyboardEqualsAndPlus, None)),
    Some((Kc::KeyboardBackspace, None)),
    Some((Kc::KeyBoardNoKey, None)), // special function one
    Some((Kc::Keyboard8AndAsterisk, None)),
    Some((Kc::Keyboard9AndRightParentheses, None)),
    Some((Kc::Keyboard0AndLeftParentheses, None)),
    Some((Kc::KeyboardSpacebar, None)),
    // Y7
    Some((Kc::KeyboardLeftSquareBracketAndCurlyBracket, None)),
    Some((Kc::KeyboardRightSquareBracketAndCurlyBracket, None)),
    Some((Kc::KeyboardBackslashAndPipe, None)),
    Some((Kc::KeyBoardNoKey, None)), // special function 2
    Some((Kc::KeyboardU, None)),
    Some((Kc::KeyboardI, None)),
    Some((Kc::KeyboardO, None)),
    Some((Kc::KeyboardP, None)),
    // Y8
    Some((Kc::KeyboardSingleAndDoubleQuotes, None)),
    Some((Kc::KeyboardEnter, None)),
    Some((Kc::KeyBoardNoKey, None)), // special function 3
    None,
    Some((Kc::KeyboardJ, None)),
    Some((Kc::KeyboardK, None)),
    Some((Kc::KeyboardL, None)),
    Some((Kc::KeyboardSemicolonAndColon, None)),
    // Y9
    Some((Kc::KeyboardSlashAndQuestionMark, None)),
    Some((Kc::KeyboardUpArrow, None)),
    Some((Kc::KeyBoardNoKey, None)), // special function 4
    None,
    Some((Kc::KeyboardM, None)),
    Some((Kc::KeyboardCommaAndLessThan, None)),
    Some((Kc::KeyboardPeriodAndGreaterThan, None)),
    Some((Kc::KeyboardEnter, None)), // matrix says DONE but idk wtf that is
    // Y10
    Some((Kc::KeyboardDelete, None)),
    Some((Kc::KeyboardLeftArrow, None)),
    Some((Kc::KeyboardDownArrow, None)),
    Some((Kc::KeyboardRightArrow, None)),
    None,
    None,
    None,
    None,
    // Y11
    Some((Kc::KeyboardLeftShift, None)),
    Some((Kc::KeyboardRightShift, None))
];
