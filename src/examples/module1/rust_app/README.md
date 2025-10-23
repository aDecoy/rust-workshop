# Oppgave 1 : Få rust til å kjøre

# Nedlastning av IDE (hvis du ikke vil bruke din egen)

## Rustrover

https://www.jetbrains.com/rust/download/?section=linux

### Linux

sudo snap install rustrover --classic

### Mac
https://www.petergirnus.com/blog/rust-macos-how-to-install#:~:text=In%20this%20tutorial%2C%20I%20will%20show%20you%20how,configure%20the%20entire%20Rust%20toolchain%20and%20its%20dependencies. 

# Rustc

## Alternativ 1: via IDE

Kan lastes inn via rustrover. (Kommer pop-up om "toolchain" når går til en rust main.rs fil. Trykk på den og på download rustc)
Du burde nå kunne høyreklikke og trykke kjør.

## Alternativ 2: Følg guide

https://doc.rust-lang.org/cargo/getting-started/installation.html

# Kjøre main.rs

Man kan få attach "attach cargo.toml" pop up i Rustrover, og kjøre via IDE sin run.
Av erfaring så er det ofte lettere å navigere til main.rs i terminalen og kjøre `cargo run`
