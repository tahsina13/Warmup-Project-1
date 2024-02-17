// code for connect4 game
use dioxus::prelude::*;
use rand::prelude::*;

#[derive(Debug, Clone, PartialEq)]
struct Board {
    chips: [[String; 7]; 5]
}

impl Board {
    fn new() -> Self {
        Board { chips: Default::default() }
    }

    fn from(encoding: &str) -> Self {
        let mut chips: [[String; 7]; 5] = Default::default();
        let rows: Vec<&str> = encoding.split('.').collect();
        for (i, row) in rows.iter().enumerate() {
            let cols: Vec<&str> = row.split(' ').collect(); 
            for (j, col) in cols.iter().enumerate() {
                if !col.is_empty() {
                    chips[i][j] = col.to_string(); 
                }
            }
        }
        Board { chips } 
    }

    fn to_string(&self) -> String {
        let mut rows: [String; 5] = Default::default(); 
        for (i, row) in self.chips.iter().enumerate() {
            rows[i] = row.join(" ");
        }
        rows.join(".") 
    }

    fn make_move(&mut self, col: usize, player: &str) -> Result<(), String> {
        for i in (0..5).rev() {
            if self.chips[i][col].is_empty() {
                self.chips[i][col] = player.to_string();
                return Ok(());
            }
        }
        Err("Column is full".to_string())
    }

    fn is_full(&self) -> bool {
        for i in 0..5 {
            if self.chips[i][0].is_empty() {
                return false;
            }
        }
        true
    }

    fn has_win(&self) -> Option<&str> {
        // across, down, diagonal
        const DX: [i32; 3] = [0, -1, -1]; 
        const DY: [i32; 3] = [-1, 0, -1]; 
        for i in 0..5 {
            for j in 0..7 {
                if self.chips[i][j].is_empty() {
                    continue;
                }
                for k in 0..DX.len() {
                    let mut count = 0; 
                    for l in 0..4 {
                        let (nx, ny) = ((i as i32)+DX[k]*l, (j as i32)+DY[k]*l); 
                        let in_bounds = 0 <= nx && nx < 5 && 0 <= ny && ny < 7;
                        if !in_bounds {
                            break;
                        }
                        if self.chips[nx as usize][ny as usize] == self.chips[i][j] {
                            count += 1;
                        }
                    }
                    if count == 4 {
                        return Some(&self.chips[i][j]);
                    }
                }
            }
        } 
        None
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
            link { rel: "stylesheet", href: "/connect.css" }
        }
        body {
            form {
                action: "/connect.php",
                method: "POST",
                label { r#for: "name", "Name:"}
                input { id: "name", name: "name", r#type: "text", required: true }
                input { r#type: "submit", value: "Submit" }
            }
        }
    })
}

#[component]
fn Game(cx: Scope<GameProps>) -> Element {
    let mut states: [String; 7] = Default::default();
    for i in 0..7 {
        let mut board = cx.props.board.clone();
        match board.make_move(i, "X") {
            Ok(()) => states[i] = board.to_string(),
            Err(_) => continue
        } 
    }
    
    cx.render(rsx! {
        form {
            id: "game-form",
            action: "/connect.php",
            method: "POST",
            input { r#type: "hidden", name: "name", value: "{cx.props.name}" }
        }
        table {
            thead {
                tr {
                    states.iter().map(|state| {
                        rsx! {
                            th {
                                if !state.is_empty() {
                                    rsx! {
                                        button { 
                                            form: "game-form",
                                            r#type: "submit", 
                                            name: "board", 
                                            value: "{state}",
                                            "X"
                                        }
                                    }
                                }
                            }
                        }
                    })
                }
            }
            tbody {
                (0..5).map(|i| {
                    rsx! {
                        tr {
                            (0..7).map(|j| {
                                rsx! {
                                    td { 
                                        width: "50px",
                                        height: "50px",
                                        border: "1px solid black", 
                                        text_align: "center",
                                        cx.props.board.chips[i][j].as_str() 
                                    }
                                }
                            })
                        }
                    }
                })
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
        encoding => {
            let mut board = Board::from(encoding); 
            let mut rng = rand::thread_rng();
            let mut cols = vec![]; 
            for i in 0..7 {
                if board.chips[0][i].is_empty() {
                    cols.push(i);
                }
            }
            let col = cols.choose(&mut rng).unwrap();
            board.make_move(*col, "O").expect("Invalid move");  
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
        },
        None => {
            if board.is_full() {
                return cx.render(rsx! { PlayAgain { state: "Draw" } });
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
            form {
                action: "/connect.php",
                method: "POST",
                input { r#type: "submit", value: "Play again" }
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
    let mut app = VirtualDom::new_with_props(
        Play, PlayProps { name, encoding }
    );
    let _ = app.rebuild();      
    format!(
        "<!DOCTYPE html><html lang='en'>{}</html", 
        dioxus_ssr::render(&app)
    )
}
