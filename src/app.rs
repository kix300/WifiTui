use std::collections::HashMap;

pub enum CurrentScreen {
    Main,
    Wifi,
    Info,
    Exiting,
}

pub enum CurrentKey {
    Key,
    Value,
}

pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub current_key: Option<CurrentKey>,
}

impl App {
    pub fn new() -> App {
        App{
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            current_key: None,
        }
    }
    
    /// Can write password in ui + nmcli
    /// enter a password will be saved to ui and nmcli
    pub fn edit_connection(&mut self){}

    ///Used to save the change
    ///.To be defined
    pub fn save_change(&mut self){}
}
