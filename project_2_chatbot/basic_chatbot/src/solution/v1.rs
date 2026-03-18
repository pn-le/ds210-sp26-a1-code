use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV1 {
    model: Llama,
}

impl ChatbotV1 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV1 { //executes when launched for first time to constuct chatbot
        return ChatbotV1 { model: model };
    }

    #[allow(dead_code)]     
    pub async fn chat_with_user(&mut self, message: String) -> String { //async is large function so lets subsequent functions run asw
        //takes 2 arguements, one for the chatbot and another for the user msg to chatbot
        let mut chat_session: Chat<Llama> = self.model //selects the model
            .chat()
            .with_system_prompt("The assistant will act like a german");


        let asynchronous_output = chat_session.add_message(message);
        let output = asynchronous_output.await;

        return output.unwrap(); //extracts the success value, or crashes the program if there's an error

        // consider https://docs.rs/kalosm/0.4.0/kalosm/language/struct.Chat.html#method.add_message
        // Hint: make sure you transform/extract the response message as a **String**.

        return String::from("Hello, I am not a bot (yet)!");
    }
}

//every time I input a message, the chat_with_user function is called, creating a NEW chat_session each prompt
//so each time i promt, the block from chat_with_user is reinitialised, so the old variables go out of scope
//we want to try and retain the chatbot memory (the message history)