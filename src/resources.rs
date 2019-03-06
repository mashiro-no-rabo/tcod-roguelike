#[derive(Debug, Default)]
pub struct InputMapping {
    pub key: Option<VirtualKey>,
    pub mouse: Option<(i32, i32)>,
}

#[derive(Debug, PartialEq)]
pub enum VirtualKey {
    NoAction,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Exit,
    PickItem,
    DropItem,
}

#[derive(Debug, Default)]
pub struct MesLogs {
    pub messages: Vec<String>,
    pub logs: Vec<String>,
}

#[derive(Debug, Default)]
pub struct PlayerExit(pub bool);
