extern crate slack;

use action;
use self::slack::{Event, EventHandler, Message, RtmClient};

pub struct Handler;

#[allow(unused_variables)]
impl EventHandler for Handler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
//        println!("on_event(event: {:?})", event);

        match event.clone() {
            Event::Message(message) => self.handle_message(*message, cli, &event),
            _ => return
        };
    }

    fn on_close(&mut self, cli: &RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        println!("on_connect");
    }
}

#[allow(unused_variables)]
impl Handler {
    fn handle_message(&mut self, message: Message, cli: &RtmClient, event: &Event) {
        let bot_name = "rustbot";
        let message_standard = match message {
            Message::Standard(message_standard) => message_standard,
            _ => return
        };
        let exp_channel = "DA03FJJH3";
        let exp_user = "W6B1UEMEF";
        let channel: String = message_standard.channel.unwrap();
        let user: String = message_standard.user.unwrap();

        let bot_id: &str = cli.start_response().slf.as_ref().unwrap().id.as_ref().unwrap();
        println!("Bot ID: {} | channel: {} | user: {}", bot_id, channel, user);
        if user == bot_id {
            println!("Is own message");
            return
        }

        let text: String = message_standard.text.unwrap();

        if !user.eq(exp_user) {
            println!("Is not right user");
            return
        }

        if text.contains(bot_name) {
            println!("say hi");
            action::respond_hi(&bot_id, &text, &channel, &cli);
        }

        return
    }
}
