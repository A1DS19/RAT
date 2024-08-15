use rust_socketio::Payload;
use serde::{de::Error, Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct CommandData {
    pub target_id: String,
    pub command: String,
}

impl CommandData {
    pub fn new(target_id: String, command: String) -> Self {
        CommandData { target_id, command }
    }
}

pub fn parse_command_data(payload: Payload) -> Result<CommandData, serde_json::Error> {
    println!("Payload: {:?}", payload);

    match payload {
        Payload::Text(text_vec) => match text_vec.get(0) {
            Some(text) => {
                if let Some(txt) = text.as_str() {
                    println!("Text: {:?}", txt);
                    Ok(CommandData::new("".to_string(), txt.to_string()))
                } else {
                    Err(serde_json::Error::custom("Text is not a string"))?
                }
            }

            None => Err(serde_json::Error::custom("No text in payload"))?,
        },

        _ => Err(serde_json::Error::custom("Payload is not text"))?,
    }
}
