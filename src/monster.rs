trait Fight {}

struct Mensch {}
struct Ork {}

pub struct Arena {
    fighters: /* Datentyp? */,
}

impl Arena {
    pub fn new() -> Self {
        // neue Arena anlegen
    }

    fn add_fighter(&mut self, fighter: /* Welcher Datentyp muss hier verwendet werden? */) {
        // Hier soll ein neuer Kämpfer der Arena hinzugefügt werden
    }

    pub fn print_fighters(&self) {
        for f in &self.fighters {
            println!("{}", f.to_string());
        }
    }

    pub fn add_mensch(&mut self) {
        self.add_fighter(Box::new(Mensch{}));
    }

    pub fn add_ork(&mut self) {
        self.add_fighter(Box::new(Ork{}));
    }
}
