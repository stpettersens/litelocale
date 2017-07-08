/*
    litelocale.
    Lightweight localization library for Rust.
    Copyright (c) 2017 Sam Saint-Pettersen.

    Released under the MIT License.
*/

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)]
pub struct LocaleMessage {
    locstr: String,
    message: String,
}

impl LocaleMessage {
    fn get_str(&self) -> String {
        self.locstr.clone()
    }
    fn get_message(&self) -> String {
        self.message.clone()
    }
}

pub struct Locale {
    messages: Vec<LocaleMessage>,
}

impl Locale {
    pub fn new() -> Locale {
        Locale {
            messages: Vec::new(),
        }
    }
    pub fn add_message(&mut self, message: LocaleMessage) {
        self.messages.push(message);
    }
    pub fn get_message_str(&self, locstr: &str) -> String {
        let mut message_str = String::new();
        for message in &self.messages {
            if message.get_str() == locstr {
                message_str = message.get_message();
            }
        }
        message_str
    }
}

pub fn printloc(message: &str, locale: &Locale) {
    let mut localized: Vec<String> = Vec::new();
    let split = message.split(" ");
    let locstrs: Vec<&str> = split.collect();
    if !locale.get_message_str(locstrs[0]).is_empty() {
        for l in locstrs {
            localized.push(locale.get_message_str(l));
        }
    } else {
        println!("{}", message);
    }
    println!("{}", localized.join(" "));
}

pub fn localize(message: &str, locale: &Locale) -> String {
    let mut localized: Vec<String> = Vec::new();
    let split = message.split(" ");
    let locstrs: Vec<&str> = split.collect();
    if !locale.get_message_str(locstrs[0]).is_empty() {
        for l in locstrs {
            localized.push(locale.get_message_str(l));
        }
    } else {
        return message.to_owned();
    }
    localized.join(" ")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
