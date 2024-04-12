struct Coordinates {
    x: u8,
    y: u8,
}

struct GameState {
    time: u128,
    score: i32,
    length: i16,
    parts: Vec<Coordinates>,
}

pub fn run() {
    let mut gamestate: GameState = GameState {
        time: 0,
        score: 0,
        length: 0,
        parts: vec![],
    };
    hello_world(&gamestate);
    gamestate.parts.push(Coordinates { x: 2, y: 2 });
    hello_world(&gamestate)
}

fn hello_world(state: &GameState) {
    println!("Hello, world {}!", state.parts.len());
    if state.parts.len() > 0 {
        println!("First part at: {}, {}", state.parts[0].x, state.parts[0].y)
    }
}
