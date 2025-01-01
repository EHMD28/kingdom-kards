pub fn clear_screen() {
    print!("\x1b[2J\x1b[H");
}

pub fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}
