// code for tic-tac-toe game
#![allow(clippy::needless_range_loop)]
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

    fn get_state(&self) -> &'static str {
        match self.has_win() {
            Some("X") => "You won!",
            Some("O") => "I won!",
            Some(_) => panic!("Invalid state"),
            None => if self.is_full() { "WINNER: NONE.  A STRANGE GAME.  THE ONLY WINNING MOVE IS NOT TO PLAY." } else { "" }
        } 
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
    name: String,
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
                method: "GET",
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
    let is_end = cx.props.board.has_win() != None || cx.props.board.is_full();
    cx.render(rsx! {
        table {
            tbody {
                for (i, row) in states.iter().enumerate() {
                    rsx! {
                        tr {
                            for (j, state) in row.iter().enumerate() {
                                rsx! {
                                    if cx.props.board.chips[i][j].as_str().is_empty() && !is_end{
                                        rsx! {
                                            td {
                                                width: "50px",
                                                height: "50px",
                                                border: "1px solid black",
                                                a {
                                                    href: "/ttt.php?name={cx.props.name}&board={state}",

                                                    style: "width: 100%; height: 100%; display: flex; justify-content: center; align-items: center;",
                                                    " "
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
    let date = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(); 
    let board = match cx.props.encoding.as_str() {
        "" => Board::new(),
        "        " => Board::new(),
        encoding => {
            let mut board = Board::from(encoding);
            if board.get_state().is_empty() {
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
            }
            board
        }
    };
    let state = board.get_state(); 

    cx.render(rsx! {
        p { "Hello {name}, {date}" }
        if !state.is_empty() {
            rsx! { p { "{state}" } }
        }
        Game { name: name, board: board }
        if !state.is_empty() && state != "WINNER: NONE.  A STRANGE GAME.  THE ONLY WINNING MOVE IS NOT TO PLAY." {
            rsx! {
                a { href: "/ttt.php?name={cx.props.name}", "Play Again" }
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
