pub struct OxAgTool {
    x: i32,
}

impl OxAgTool {
    pub fn new() -> Self {
        Self { x: 30 }
    }
}

impl Default for OxAgTool {
    fn default() -> Self {
        Self::new()
    }
}
