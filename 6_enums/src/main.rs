// ENUMS
#![allow(warnings)]         // Help to ignore warnings

enum MieAyam {              // Enums represents one of multiple possible variants 
    Jawa,                   
    Cina,
    Bangka,
}

enum Bakmi {                // Each variants can contain have their own data
    Jawa(String),
    Cina(String),
    Bangka(String),
}

enum Message {              // Each variants also can have their own data type, in the same Enum
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {              // Enums also can be implemented with
    fn call(&self) {        // Methods

    }
}

fn main() {
    let _mieJaka = MieAyam::Jawa;

    let mieTerEnak = Bakmi::Cina(String::from("Roxy"));

    let SuratCinta = Message::Write(String::from("Aku cinta kamu"));
}

