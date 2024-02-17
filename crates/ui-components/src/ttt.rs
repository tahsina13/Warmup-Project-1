#![allow(clippy::needless_range_loop)]

// code for tic-tac-toe game
use dioxus::prelude::*;
use rand::Rng;
use std::fmt;


#[derive(Debug, Clone, PartialEq)]
struct Board {
    chips: [[String; 3]; 3],
}

impl Board {
    fn new() -> Self {
        Board {
            chips: Default::default(),
        }
    }
    fn from(encoding: &str) -> Self {
        let mut chips: [[String; 3]; 3] = Default::default();
        let chars = encoding.chars().collect::<Vec<char>>();
        let mut idx = 0;
        for &value in chars.iter() {
            if value == ' ' {
                idx += 1;
            } else {
                chips[idx / 3][idx % 3] = value.to_string();
            }
        }
        Board { chips }
    }

    fn is_full(&self) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if self.chips[i][j].is_empty() {
                    return false;
                }
            }
        }
        true
    }

    fn make_move(&mut self, row: usize, col: usize, player: &str) -> Result<(), String> {
        if self.chips[row][col].is_empty() {
            self.chips[row][col] = player.to_string();
            return Ok(());
        }
        Err("Cannot place player at location".to_string())
    }

    fn has_win(&self) -> Option<&str> {
        // across, down, diagonal
        const DX: [i32; 4] = [0, -1, -1, -1];
        const DY: [i32; 4] = [-1, 0, 1, -1];
        for i in 0..3 {
            for j in 0..3 {
                if self.chips[i][j].is_empty() {
                    continue;
                }
                for k in 0..DX.len() {
                    let mut count = 0;
                    for l in 0..3 {
                        let (nx, ny) = ((i as i32) + DX[k] * l, (j as i32) + DY[k] * l);
                        let in_bounds = (0..3).contains(&nx) && (0..3).contains(&ny);
                        if !in_bounds {
                            break;
                        }
                        if self.chips[nx as usize][ny as usize] == self.chips[i][j] {
                            count += 1;
                        }
                    }
                    if count == 3 {
                        return Some(&self.chips[i][j]);
                    }
                }
            }
        }
        None
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut encoded: String = String::new();
        for i in 0..3 {
            for j in 0..3 {
                encoded.push_str(&self.chips[i][j]);
                encoded.push(' ');
            }
        }
        encoded.pop();
        write!(f, "{}", encoded)
    }
}

#[derive(Debug, Clone, PartialEq, Props)]
struct GameProps {
    name: String,
    board: Board,
}

#[derive(Debug, Clone, PartialEq, Props)]
struct PlayProps {
    name: String,
    encoding: String,
}

#[derive(Debug, Clone, PartialEq, Props)]
struct PlayAgainProps<'a> {
    state: &'a str,
}

#[component]
fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        head {
            link { rel: "stylesheet", href: "/ttt.css" }
        }
        body {
            form {
                action: "/ttt.php",
                method: "POST",
                label { r#for: "name", "Name: "}
                input { id: "name", name: "name", r#type: "text", required: true }
                input { r#type: "submit", value: "Submit" }
            }
        } 
    })
}

#[component]
fn Game(cx: Scope<GameProps>) -> Element {
    let mut states: [[String; 3]; 3] = Default::default();
    for i in 0..3 {
        for j in 0..3 {
            let mut board = cx.props.board.clone();
            match board.make_move(i, j, "X") {
                Ok(()) => states[i][j] = board.to_string(),
                Err(_) => continue,
            }
        }
    }

    cx.render(rsx! {
        form {
            id: "game-form",
            action: "/ttt.php",
            method: "POST",
            input { r#type: "hidden", name: "name", value: "{cx.props.name}" }
        }
        table {
            tbody {
                for (i, row) in states.iter().enumerate() {
                    rsx! {
                        tr {
                            for (j, state) in row.iter().enumerate() {
                                rsx! {
                                    if cx.props.board.chips[i][j].is_empty() {
                                        rsx! {
                                            td {
                                                width: "50px",
                                                height: "50px",
                                                border: "1px solid black",
                                                button {
                                                    form: "game-form",
                                                    r#type: "submit",
                                                    name: "board",
                                                    style: "width: 100%; height: 100%",
                                                    value: "{state}",
                                                    "X"
                                                }
                                            }
                                        }
                                    }else{
                                        rsx! {
                                            td {
                                                width: "50px",
                                                height: "50px",
                                                border: "1px solid black",
                                                text_align: "center",
                                                cx.props.board.chips[i][j].as_str()
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

#[component]
fn Play(cx: Scope<PlayProps>) -> Element {
    let name = cx.props.name.to_string();
    let date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let board = match cx.props.encoding.as_str() {
        "" => Board::new(),
        "        " => Board::new(),
        encoding => {
            let mut board = Board::from(encoding);
            let mut rng = rand::thread_rng();
            let mut positions = Vec::new();
            for i in 0..3 {
                for j in 0..3 {
                    if board.chips[i][j].is_empty() {
                        positions.push((i, j));
                    }
                }
            }
            // make a random move
            if !positions.is_empty() {
                let (i, j) = positions[rng.gen_range(0..positions.len())];
                let _ = board.make_move(i, j, "O");
            }
            board
        }
    };

    match board.has_win() {
        Some(win) => {
            if win == "X" {
                return cx.render(rsx! { PlayAgain { state: "You won!" } });
            } else {
                return cx.render(rsx! { PlayAgain { state: "I won!" } });
            }
        }
        None => {
            if board.is_full() {
                return cx.render(rsx! { PlayAgain { state: "WINNER: NONE.  A STRANGE GAME.  THE ONLY WINNING MOVE IS NOT TO PLAY." } });
            }
        }
    }

    cx.render(rsx! {
        p { "Hello {name}, {date}" }
        Game { name: name, board: board }
    })
}

#[component]
fn PlayAgain<'a>(cx: Scope<'a, PlayAgainProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            p { "{cx.props.state}" }
            if cx.props.state != "WINNER: NONE.  A STRANGE GAME.  THE ONLY WINNING MOVE IS NOT TO PLAY." {
                rsx!{
                    form {
                        action: "/ttt.php",
                        method: "POST",
                        input { r#type: "submit", value: "Play again" }
                    }
                }
            }
        }
    })
}

pub fn get_form_html() -> String {
    let mut app = VirtualDom::new(Home);
    let _ = app.rebuild();
    format!(
        "<!DOCTYPE html><html lang='en'>{}</html",
        dioxus_ssr::render(&app)
    )
}

pub fn accept_from_html(name: String, encoding: String) -> String {
    let mut app = VirtualDom::new_with_props(Play, PlayProps { name, encoding });
    let _ = app.rebuild();
    format!(
        "<!DOCTYPE html><html lang='en'>{}</html",
        dioxus_ssr::render(&app)
    )
}
