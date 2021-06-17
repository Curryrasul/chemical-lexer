pub fn is_lowercase(c: char) -> bool {
    return (c as u8) > 96 && (c as u8) < 123
}

pub fn is_uppercase(c: char) -> bool {
    return (c as u8) > 64 && (c as u8) < 91
}
