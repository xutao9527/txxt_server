use std::sync::{Arc, OnceLock, RwLock};

static APP_DATA: OnceLock<Arc<RwLock<AppData>>> = OnceLock::new();

pub enum TerminalMode {
    Connect,
    Control,
}

pub enum ConnectState {
    Normal,
    Selected,
}
pub struct ConnectInfo {
    pub table_no: String,
    pub password: String,
    pub state: ConnectState,
    pub connected:bool
}

impl ConnectInfo {
    fn default() -> Vec<Self> {
        vec![
            ConnectInfo {
                table_no: "101".into(),
                password: "7735a36e802561ef44249c039c8db410".into(),
                state: ConnectState::Selected,
                connected:false
            },
            ConnectInfo {
                table_no: "102".into(),
                password: "7735a36e802561ef44249c039c8db410".into(),
                state: ConnectState::Normal,
                connected:false
            },
            ConnectInfo {
                table_no: "103".into(),
                password: "7735a36e802561ef44249c039c8db410".into(),
                state: ConnectState::Normal,
                connected:true
            },
            ConnectInfo {
                table_no: "104".into(),
                password: "7735a36e802561ef44249c039c8db410".into(),
                state: ConnectState::Normal,
                connected:false
            },
            ConnectInfo {
                table_no: "105".into(),
                password: "7735a36e802561ef44249c039c8db410".into(),
                state: ConnectState::Normal,
                connected:false
            },
            ConnectInfo {
                table_no: "106".into(),
                password: "7735a36e802561ef44249c039c8db410".into(),
                state: ConnectState::Normal,
                connected:false
            },
            ConnectInfo {
                table_no: "107".into(),
                password: "7735a36e802561ef44249c039c8db410".into(),
                state: ConnectState::Normal,
                connected:false
            },
            ConnectInfo {
                table_no: "108".into(),
                password: "7735a36e802561ef44249c039c8db410".into(),
                state: ConnectState::Normal,
                connected:false
            },
            ConnectInfo {
                table_no: "109".into(),
                password: "7735a36e802561ef44249c039c8db410".into(),
                state: ConnectState::Normal,
                connected:false
            },
            ConnectInfo {
                table_no: "110".into(),
                password: "7735a36e802561ef44249c039c8db410".into(),
                state: ConnectState::Normal,
                connected:false
            },
            ConnectInfo {
                table_no: "111".into(),
                password: "7735a36e802561ef44249c039c8db410".into(),
                state: ConnectState::Normal,
                connected:false
            },
            ConnectInfo {
                table_no: "112".into(),
                password: "7735a36e802561ef44249c039c8db410".into(),
                state: ConnectState::Normal,
                connected:false
            },
        ]
    }
}

pub struct AppData {
    pub should_exit: bool,
    pub terminal_mode: TerminalMode,
    pub connects: Vec<ConnectInfo>,
}

impl AppData {
    pub fn singleton() -> Arc<RwLock<AppData>> {
        APP_DATA
            .get_or_init(|| Arc::new(RwLock::new(
                AppData {
                    should_exit: false,
                    terminal_mode: TerminalMode::Connect,
                    connects: ConnectInfo::default(),
                })))
            .clone()
    }
}
