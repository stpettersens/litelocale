/*
    litelocale.
    Lightweight localization library for Rust.

    Copyright (c) 2017 Sam Saint-Pettersen.

    Released under the MIT License.
*/

// ! Lightweight localization library for Rust.

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)]
pub struct LocaleMessage {
    locstr: String,
    message: String,
    phonetic: String,
}

impl LocaleMessage {
    ///
    /// Create a new LocaleMessage, a unit of localization.
    ///
    /// * `locstr` - Identifying string (i.e. in English).
    /// * `message` - Message string in localized language.
    /// * `phonetic` - Message string in localized language as phonetic (optional).
    ///
    /// # Return value:
    /// A LocaleMessage.
    pub fn new(locstr: &str, message: &str, phonetic: &str)
    -> LocaleMessage {
        LocaleMessage {
            locstr: locstr.to_owned(),
            message: message.to_owned(),
            phonetic: phonetic.to_owned(),
        }
    }
    fn get_str(&self) -> String {
        self.locstr.clone()
    }
    fn get_message(&self) -> String {
        self.message.clone()
    }
    fn get_phonetic(&self) -> String {
        self.phonetic.clone()
    }
}

pub struct Locale {
    messages: Vec<LocaleMessage>,
}

impl Locale {
    ///
    /// Create a new Locale, a collection of LocaleMessages.
    ///
    /// # Return value:
    /// A Locale.
    pub fn new() -> Locale {
        Locale {
            messages: Vec::new(),
        }
    }
    ///
    /// Add a LocaleMessage to the Locale.
    /// 
    /// `message` - LocalMessage to add to this Locale.
    ///
    pub fn add_message(&mut self, message: LocaleMessage) {
        self.messages.push(message);
    }
    fn get_message_str(&self, locstr: &str) -> String {
        let mut message_str = String::new();
        for message in &self.messages {
            if message.get_str() == locstr {
                message_str = message.get_message();
            }
        }
        message_str
    }
    fn get_phonetic_str(&self, locstr: &str) -> String {
        let mut phonetic_str = String::new();
        for message in &self.messages {
            if message.get_str() == locstr {
                phonetic_str = message.get_phonetic();
            }
        }
        phonetic_str
    }
}

///
/// Localize each word (e.g. hello -> hola) in the provided 
/// message string using the provided Locale.
///
/// `message` - Message string to localize (i.e. in English).
/// `locale` - &Locale to use for localization.
///
/// # Return value:
/// A localized message string or original message as-is provided.
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

///
/// Localize each word phonetically (e.g. hello -> ola) 
/// in provided message string using the provided Locale
/// where phonetic strings have been provided.
///
/// `message` - Message string to localize (i.e. in English).
/// `locale` - &Locale to use for localization.
///
/// # Return value:
/// A localized phonetic message string or original message as-is provided.
pub fn phoneticize(message: &str, locale: &Locale) -> String {
    let mut phoneticized: Vec<String> = Vec::new();
    let split = message.split(" ");
    let locstrs: Vec<&str> = split.collect();
    if !locale.get_message_str(locstrs[0]).is_empty() {
        for l in locstrs {
            phoneticized.push(locale.get_phonetic_str(l));
        }
    } else {
        return message.to_owned();
    }
    phoneticized.join(" ")
}

#[cfg(test)]
fn load_spanish_locale() -> Locale {
    let mut es = Locale::new();
    es.add_message(LocaleMessage::new("hello", "hola", "ola"));
    es.add_message(LocaleMessage::new("and", "y", "ee"));
    es.add_message(LocaleMessage::new("goodbye", "adiós", "adeeos"));
    es
}
#[test]
fn localize_each_word() {
    let es = load_spanish_locale();
    assert_eq!("hola", localize("hello", &es));
    assert_eq!("y", localize("and", &es));
    assert_eq!("adiós", localize("goodbye", &es));
}
#[test]
fn phoneticize_each_word() {
    let es = load_spanish_locale();
    assert_eq!("ola", phoneticize("hello", &es));
    assert_eq!("ee", phoneticize("and", &es));
    assert_eq!("adeeos", phoneticize("goodbye", &es));
}
#[test]
fn localize_message() {
    let es = load_spanish_locale();
    assert_eq!("hola y adiós", localize("hello and goodbye", &es));
}
#[test]
fn phoneticize_message() {
    let es = load_spanish_locale();
    assert_eq!("ola ee adeeos", phoneticize("hello and goodbye", &es));
}
