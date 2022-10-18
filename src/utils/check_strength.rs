use sycamore::reactive::ReadSignal;

pub fn check_strength(pass: &ReadSignal<String>) -> u8 {
    // Yeah. This is a placeholder
    match pass.get().len() {
        1..=6 => 1,
        7..=10 => 2,
        11..=14 => 3,
        15..=16 => 4,
        _ => 0,
    }
}
