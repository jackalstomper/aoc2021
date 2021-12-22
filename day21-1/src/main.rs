const MIN_POS: u32 = 1;
const MAX_POS: u32 = 10;
const MIN_DIE: u32 = 1;
const MAX_DIE: u32 = 100;

struct Player {
    pos: u32,
    score: u64,
}

fn main() {
    let mut p1 = Player { pos: 8, score: 0 };
    let mut p2 = Player { pos: 5, score: 0 };

    let mut die = 1;
    let mut die_rolls = 0;
    let mut p1_turn = true;
    loop {
        let player = if p1_turn { &mut p1 } else { &mut p2 };
        let roll = die + (die + 1) + (die + 2);
        die_rolls += 3;
        die = wrap(die + 3, MIN_DIE, MAX_DIE);
        player.pos = wrap(player.pos + roll, MIN_POS, MAX_POS);
        player.score += player.pos as u64;
        if player.score >= 1000 {
            break;
        }

        p1_turn = !p1_turn;
    }

    println!(
        "Die Rolls: {}. p2 score: {}, answer: {}",
        die_rolls,
        p2.score,
        die_rolls * p2.score
    );
}

fn wrap(num: u32, min: u32, max: u32) -> u32 {
    ((num - min) % (max + 1 - min)) + min
}
