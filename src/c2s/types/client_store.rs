use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use socketioxide::extract::SocketRef;

pub type ClientStore = Arc<Mutex<HashMap<String, SocketRef>>>;
