/// Loads input for a specific Advent of Code day.
///
/// Arguments:
/// * `day`: The day number (e.g., 1, 2, 25).
///
/// Priority:
/// 1. Command-line argument (e.g., `cargo run -- inputs/custom.txt`)
/// 2. Default pattern: `inputs/dayXX.txt` (where XX is zero-padded, e.g., "01")
pub fn get_input(day: u8) -> std::io::Result<String> {
    let file_path = std::env::args().nth(1).unwrap_or_else(|| {
        // {:02} pads the number with a leading zero if it's less than 10.
        // 1 -> "inputs/day01.txt"
        // 15 -> "inputs/day15.txt"
        format!("inputs/day{:02}.txt", day)
    });

    std::fs::read_to_string(file_path)
}
