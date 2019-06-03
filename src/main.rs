mod monster;

use monster::Arena;

fn main() {
    let mut arena = Arena::new();

    arena.add_mensch();
    arena.add_ork();

    arena.print_fighters();
}
