use kalosm::language::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ChatbotV3 {
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
    // Storing a single chat session is not enough: it mixes messages from different users
    // together!
    // Need to store one chat session per user.
    // Think of some kind of data structure that can help you with this.
    model: Llama,
    sessions: HashMap<String, Chat<Llama>>,
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        return ChatbotV3 {
            // Make sure you initialize your struct members here
            model,
            sessions: HashMap::new(),
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        if !self.sessions.contains_key(&username) {
            let chat_session = self.model
                .chat()
                .with_system_prompt("The assistant will act like a Vietnamese grandpa");
            self.sessions.insert(username.clone(), chat_session);
        }

        let chat_session = self.sessions.get_mut(&username).unwrap();
        let response = chat_session.add_message(message).await.unwrap();
        response.to_string()
        
    }

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
       if let Some(chat_session) = self.sessions.get(&username) {
            return chat_session.session().unwrap().history()
                .iter()
                .filter(|m| m.role() != MessageType::SystemPrompt)
                .map(|m| m.content().to_string())
                .collect();
        }
        return Vec::new();
    }
}