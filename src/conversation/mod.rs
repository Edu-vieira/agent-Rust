use std::collections::VecDeque;

use crate::llm::ChatMessage;

const MAX_HISTORY_MESSAGES: usize = 20;

pub struct Conversation {
    messages: VecDeque<ChatMessage>,
}

impl Conversation {
    pub fn new() -> Self {
        Self {
            messages: VecDeque::new(),
        }
    }

    pub fn push(&mut self, message: ChatMessage) {
        self.messages.push_back(message);

        // Limita o tamanho do histórico.
        while self.messages.len() > MAX_HISTORY_MESSAGES {
            self.messages.pop_front();
        }
    }

    pub fn messages_with_system(&self, system_message: ChatMessage) -> Vec<ChatMessage> {
        let mut messages = Vec::with_capacity(self.messages.len() + 1);

        // O system sempre permanece no início.
        messages.push(system_message);

        // Recupera o histórico da conversa.
        messages.extend(self.messages.iter().cloned());

        messages
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}
