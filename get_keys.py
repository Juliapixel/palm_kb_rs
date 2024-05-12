# used this to generate the KeyCodes enum mostly automatically

import json
import re

def replace_ands(matches: re.Match[str]) -> str:
    return matches[1]

def replace_dashes(matches: re.Match[str]) -> str:
    return matches[1]+matches[2].upper()

def bool_to_emoji(val: bool) -> str:
    if val:
        return "✅"
    else:
        return "❌"

file = open("./hid_keycodes_data.json", "r").read()

char_names: dict[str, str] = {
    "Non-US": "NonUs",
    "/": "",
    "!": "Exclamation",
    "?": "QuestionMark",
    "@": "At",
    "#": "Sharp",
    "$": "DollarSign",
    "%": "Percent",
    "∧": "Caret",
    "&": "Ampersand",
    "*": "Asterisk",
    "\\": "Backslash",
    "*": "Asterisk",
    "‘": "SingleQuote",
    "“": "DoubleQuote",
    "(": "LeftParentheses",
    ")": "RightParentheses",
    "[": "LeftSquareBrackets",
    "]": "RightSquareBrackets",
    "{": "LeftCurlyBrackets",
    "}": "RightCurlyBrackets",
    "<": "LessThan",
    ">": "GreaterThan",
    "~": "Tilde",
    ";": "Semicolon",
    ":": "Colon",
    ".": "Period",
    ",": "Comma",
    "+/-": "PlusOrMinus",
    "+": "Plus",
    "-": "Minus",
    "||": "DoublePipe",
    "|": "Pipe",
    "=": "Equals",
    "LANG": "Lang"
}

parsed = json.loads(file)
for i in parsed:
    var_name = re.sub(" [a-z] and ([A-Z])", replace_ands, i["name"], 1)
    var_name = re.sub("\\ (\\ |$)", " Slash ", var_name, 1)
    var_name = re.sub("(\\w)-(\\w)", replace_dashes, var_name, 1)
    for [key, val] in char_names.items():
        var_name = var_name.replace(key, val)
    var_name = var_name.replace(" ", "")
    print(f"/// {i["name"]}")
    print(f"/// # Platform Support")
    print(f"/// | Windows | Mac | Unix |")
    print(f"/// | ------- | --- | ---- |")
    print(f"/// | {bool_to_emoji(i["pc_at"])} | {bool_to_emoji(i["mac"])} | {bool_to_emoji(i["unix"])} |")
    print(f"{var_name} = {i["id"]},")
