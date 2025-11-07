use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WebviewWindowLabels {
    Launch,
    Initialization,
    Main,
    Settings,
}

impl Display for WebviewWindowLabels {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            WebviewWindowLabels::Launch => write!(f, "launch"),
            WebviewWindowLabels::Initialization => write!(f, "initialization"),
            WebviewWindowLabels::Main => write!(f, "main"),
            WebviewWindowLabels::Settings => write!(f, "settings"),
        }
    }
}

impl From<WebviewWindowLabels> for String {
    fn from(window: WebviewWindowLabels) -> Self {
        match window {
            WebviewWindowLabels::Launch => "launch".into(),
            WebviewWindowLabels::Initialization => "initialization".into(),
            WebviewWindowLabels::Main => "main".into(),
            WebviewWindowLabels::Settings => "settings".into(),
        }
    }
}

impl From<String> for WebviewWindowLabels {
    fn from(window: String) -> Self {
        match window {
            window if window == "launch" => WebviewWindowLabels::Launch,
            window if window == "initialization" => WebviewWindowLabels::Initialization,
            window if window == "main" => WebviewWindowLabels::Main,
            window if window == "settings" => WebviewWindowLabels::Settings,
            _ => panic!("invalid window name: {}", window),
        }
    }
}
