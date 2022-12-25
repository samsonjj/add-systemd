use serde_derive::Serialize;
use strum_macros::{Display, EnumIter, EnumString, ToString};

#[derive(Serialize)]
pub struct Systemd {
    pub Unit: Unit,
    pub Service: Service,
    pub Install: Install,
}

impl Default for Systemd {
    fn default() -> Self {
        Self {
            Unit: Unit {
                Description: "This is a description".to_string(),
                After: "network.target".to_string(),
            },
            Service: Service {
                ..Default::default()
            },
            Install: Install {
                WantedBy: "multi-user.target".to_string(),
            },
        }
    }
}

#[derive(Serialize)]
pub struct Unit {
    pub Description: String,
    pub After: String,
}

#[derive(Serialize)]
pub struct Service {
    pub User: String,
    pub ExecStart: String,
    pub Restart: Restart,
}

impl Default for Service {
    fn default() -> Self {
        Self {
            User: "gamemaster".to_string(),
            ExecStart: "/home/gamemaster/minecraft-server/start-server.sh".to_string(),
            Restart: Restart::ALWAYS,
        }
    }
}

#[derive(Serialize)]
pub struct Install {
    pub WantedBy: String,
}

// no, on-success, on-failure, on-abnormal, on-watchdog, on-abort, or always
#[allow(non_camel_case_types)]
#[derive(Serialize, EnumString, Display, EnumIter, Clone, Copy)]
pub enum Restart {
    #[serde(rename = "no")]
    NO,
    #[serde(rename = "on-success")]
    ON_SUCCESS,
    #[serde(rename = "on-failure")]
    ON_FAILURE,
    #[serde(rename = "on-abnormal")]
    ON_ABNORMAL,
    #[serde(rename = "on-watchdog")]
    ON_WATCHDOG,
    #[serde(rename = "on-abort")]
    ON_ABORT,
    #[serde(rename = "always")]
    ALWAYS,
}

fn do_thing(somehing: Restart) {
    println!("{}", somehing)
}
