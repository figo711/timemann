#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    // COUNTDOWN Tab
    SetNumber(u8),
    Edit,

    // COMMON
    ToggleStartPause,
    ChangeTab,
    Clear,
    Tick,
    Quit,
}