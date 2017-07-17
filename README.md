### litelocale
>:es: Lightweight localization library for Rust.

[![Build Status](https://travis-ci.org/stpettersens/litelocale.png?branch=master)](https://travis-ci.org/stpettersens/litelocale)
[![Build status](https://ci.appveyor.com/api/projects/status/x937wesh830othcx?svg=true)](https://ci.appveyor.com/project/stpettersens/litelocale)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/stpettersens/litelocale/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/litelocale.svg)](https://crates.io/crates/litelocale)

```toml
# Add to your Cargo.toml file dependencies:
litelocale = "0.1.0" 
# or: litelocale = { git = "https://github.com/stpettersens/litelocale.git" }
```

You can use `litelocale` to provide simple localization for your Rust program.

Below is a simple program that can display numbers from 1 to 5 in either English or Spanish depending on the selected locale:

```rust
extern crate litelocale;
use litelocale::*;

fn main() {
    // Declare a vector of numbers in English.
    let nums = vec!["one", "two", "three", "four", "five"];
    // Declare a new locale, Spanish (Español, es).
    let mut es = Locale::new();
    // Select es as the locale.
    let sel = "es";
    // Load Spanish (es) locale when English (en) is not selected.
    if sel != "en" {
        // Add each number to locale.
        // English-Spanish-Phonetic Spanish (phonetics are optional).
        es.add_message(LocaleMessage::new("one", "uno", "oono"));
        es.add_message(LocaleMessage::new("two", "dos", "dos"));
        es.add_message(LocaleMessage::new("three", "tres", "trez"));
        es.add_message(LocaleMessage::new("four", "cuatro", "katro"));
        es.add_message(LocaleMessage::new("five", "cinco", "finco"));

        /* It is recommended in more substantial programs that you instead
           deserialize LocaleMessages from an external structure.
           E.g. from JSON using serde_json:

           let mut locale = Locale::new();
           if sel != "en {
               let file = File::open("my_locale.json");
               for line in BufReader::new(file).lines() {
                   locale.add_message(
                   serde_json::from_str(&line.unwrap())
                   .unwrap()); 
                   // <- {"locstr":"one","message":"uno","phonetic":"oono"}
                   // , etc.
               }
           }
        */
    }
    // Print each number in Spanish as sel == "es".
    // localize returns the localization for each English word.
    for n in nums {
        println!("-> {}", localize(n, &es)); // -> uno, -> dos, -> tres, etc.
        // If sel == "en"; output is as-is in original vector:
        // -> one, -> two, -> three, etc.
    }
}
```

[Documentation](https://docs.rs/litelocale)
