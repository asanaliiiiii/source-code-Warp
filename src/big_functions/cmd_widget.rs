pub fn spawn_terminal_canvas(x: i32, y: i32, size: i32) -> String {
    format!(
        "🖥️ [Warp Shell Application Window spawned context at window coords ({}, {}) with viewport dimension {}px]",
        x, y, size
    )
}