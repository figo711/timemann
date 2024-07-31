use strum::EnumIs;


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumIs)]
pub enum AppState {
    #[default]
    Stopped,
    Running,
    Setup,
}