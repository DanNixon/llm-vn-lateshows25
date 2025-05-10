use crate::character::Character;
use jiff::Timestamp;
use log::{debug, info};
use ollama_rs::{
    generation::{
        chat::{request::ChatMessageRequest, ChatMessage},
        parameters::{FormatType, JsonStructure},
    },
    Ollama,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Conversation {
    started_at: Timestamp,
    character: Character,
    transcript: Vec<TranscriptEntry>,
    history: Vec<ChatMessage>,
}

impl Conversation {
    fn new(character: Character) -> Self {
        Self {
            started_at: Timestamp::now(),
            character,
            transcript: Default::default(),
            history: Default::default(),
        }
    }

    pub(crate) fn save_in(&self, dir: &Path) -> anyhow::Result<()> {
        let filename = dir.join(format!(
            "{0:.0} - {1}.json",
            self.started_at, self.character.name,
        ));
        info!("Saving conversation to {filename:?}");

        let f = std::fs::File::create(&filename)?;
        serde_json::to_writer_pretty(f, self)?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum TranscriptEntry {
    Character(String),
    User(String),
}

#[derive(JsonSchema, Deserialize, Debug)]
pub(crate) struct VnOutput {
    pub response: String,
    pub user_reply_1: String,
    pub user_reply_2: String,
    pub user_reply_3: String,
}

impl VnOutput {
    pub(crate) fn is_end_of_conversation(&self) -> bool {
        self.user_reply_1.is_empty() || self.user_reply_2.is_empty() || self.user_reply_3.is_empty()
    }

    pub(crate) fn sanitise(self) -> Self {
        fn sanitise_string(s: String) -> String {
            s.replace("’", "'")
                .replace("–", "-")
                .replace("…", "...")
                .replace("—", "-")
        }

        Self {
            response: sanitise_string(self.response),
            user_reply_1: sanitise_string(self.user_reply_1),
            user_reply_2: sanitise_string(self.user_reply_2),
            user_reply_3: sanitise_string(self.user_reply_3),
        }
    }
}

pub(crate) struct ConversationClient {
    client: Ollama,
    conversation: Conversation,
    format: FormatType,
}

impl ConversationClient {
    pub(crate) fn new(client: &Ollama, character: Character) -> Self {
        let format = FormatType::StructuredJson(JsonStructure::new::<VnOutput>());
        Self {
            client: client.clone(),
            conversation: Conversation::new(character),
            format,
        }
    }

    pub(crate) fn conversation(self) -> Conversation {
        self.conversation
    }

    pub(crate) fn character(&self) -> &Character {
        &self.conversation.character
    }

    pub(crate) async fn interact(&mut self, user_message: String) -> VnOutput {
        self.conversation
            .transcript
            .push(TranscriptEntry::User(user_message.clone()));

        let user_message = ChatMessage::user(user_message);
        info!("{user_message:?}");

        let result = self
            .client
            .send_chat_messages_with_history(
                &mut self.conversation.history,
                ChatMessageRequest::new(
                    self.conversation.character.model_name.clone(),
                    vec![user_message],
                )
                .format(self.format.clone()),
            )
            .await
            .unwrap();

        let response: VnOutput = serde_json::from_str(&result.message.content).unwrap();
        debug!("Original response: {response:?}");
        let response = response.sanitise();
        info!("{response:?}");

        self.conversation
            .transcript
            .push(TranscriptEntry::Character(response.response.clone()));

        response
    }
}
