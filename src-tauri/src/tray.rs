use crate::state::SharedState;
use crate::windows::{MainWindow, WindowTabs};
use tauri::{
	AppHandle, Manager, ClipboardManager,
    SystemTrayHandle, SystemTrayEvent, SystemTray, 
    CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem
};

/// System tray handle
pub struct Tray(SystemTrayHandle);
impl Tray {
    /// Create a new handle to the system tray
    /// 
    /// # Arguments
    /// * `app_handle` - AppHandle
    pub fn new(tray_handle: SystemTrayHandle) -> Self {
        Self(tray_handle)
    }

    /// Update the tray's menu
    /// 
    /// # Arguments
    /// * `recent_history` - Items to display to the user
    pub fn update_menu(&self, recent_history: Vec<String>) {
        self.0.set_menu(Self::tray_menu(recent_history)).ok();
    }

    /// Construct a new menu for the system tray
    /// 
    /// # Arguments
    /// * `recent_history` - Items to display to the user
    pub fn tray_menu(recent_history: Vec<String>) -> SystemTrayMenu {
        let mut menu = SystemTrayMenu::new();
        if recent_history.is_empty() {
            // Default entry for no history
            menu = menu.add_item(CustomMenuItem::new("no_history".to_string(), "No history to display").disabled());
        } else {
            // Display history entries
            for (i, item) in recent_history.into_iter().enumerate() {
                menu = menu.add_item(CustomMenuItem::new(format!("{}", i), item));
            }
        }

        // Base menu options
        menu = menu
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new("settings".to_string(), "Settings"))
            .add_item(CustomMenuItem::new("exit".to_string(), "Exit"));

        menu
    }

    /// Construct a new system tray
    /// 
    /// # Arguments
    /// * `recent_history` - Items to display to the user
    pub fn new_tray(recent_history: Vec<String>) -> SystemTray {
        SystemTray::new().with_menu(Self::tray_menu(recent_history))
    }

    /// Handle system tray events
    pub fn handle_event(app: &AppHandle, e: SystemTrayEvent) {
        match e {
            SystemTrayEvent::DoubleClick {..} => {
                match MainWindow::new(app.clone()) {
                    Some(w) => w.show_tab(WindowTabs::History).ok(),
                    None => Some(())
                };
            },
            
            SystemTrayEvent::MenuItemClick {id, ..} => {
                match id.as_str() {
                    "settings" => {
                        match MainWindow::new(app.clone()) {
                            Some(w) => w.show_tab(WindowTabs::Settings).ok(),
                            None => Some(())
                        };
                    },
    
                    "exit" => {
                        let app_ = app.clone();
                        std::thread::spawn(move || {
                            app_.exit(0);
                        });
                    },
    
                    _ => {
                        if let Ok(n) = id.as_str().parse::<i32>() {
                            let state : tauri::State<SharedState> = app.state();
                            if let Some(lock) = state.0.lock().ok() {
                                let item = &lock.history[lock.history.len() - 1 - (n as usize)];
                                app.clipboard_manager().write_text(item.expression.clone()).ok();
                            };
                        }
                    }
                }
            }
    
            _ => {}
        }
    }
}