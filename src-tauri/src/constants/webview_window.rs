use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WebviewWindow {
    Launch,
    Main,
    Settings,
}

impl Display for WebviewWindow {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            WebviewWindow::Launch => write!(f, "launch"),
            WebviewWindow::Main => write!(f, "main"),
            WebviewWindow::Settings => write!(f, "settings"),
        }
    }
}

impl From<WebviewWindow> for String {
    fn from(window: WebviewWindow) -> Self {
        match window {
            WebviewWindow::Launch => "launch".into(),
            WebviewWindow::Main => "main".into(),
            WebviewWindow::Settings => "settings".into(),
        }
    }
}

impl From<String> for WebviewWindow {
    fn from(window: String) -> Self {
        match window {
            window if window == "launch" => WebviewWindow::Launch,
            window if window == "main" => WebviewWindow::Main,
            window if window == "settings" => WebviewWindow::Settings,
            _ => panic!("invalid window name: {}", window),
        }
    }
}
