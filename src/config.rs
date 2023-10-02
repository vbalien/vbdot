use std::collections::HashMap;

use serde::Deserialize;

pub type Config = HashMap<String, Profile>;

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub link: HashMap<String, (String, String)>,
}
