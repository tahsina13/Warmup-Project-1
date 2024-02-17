// code for connect4 game
use dioxus::prelude::*;

#[derive(Debug, Clone)]
struct Board {
    chips: [[String; 7]; 5]
}

impl Board {
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
    board: String
}

#[derive(Debug, Clone, PartialEq, Props)]
struct PlayProps {
    name: String,
}

#[component]
fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        head {
            link { rel: "stylesheet", href: "/connect.css" }
        }
        div {
            display: "flex",
            flex_direction: "column",
            h1 {
                class: "my-title",
                "Connect 4"
            }
            form {
                action: "/connect.php",
                method: "POST",
                label { r#for: "name", "Name: "}
                input { id: "name", name: "name", r#type: "text", required: true }
                input { r#type: "submit", value: "Start" }
            }
        }
    })
}

#[component]
fn Play(cx: Scope<PlayProps>) -> Element {
    let name = &cx.props.name;
    let now = chrono::Utc::now().to_rfc2822();
    cx.render(rsx! {
        p { "Hello {name}, {now}" }
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

pub fn accept_from_html(name: String) -> String {
    let mut app = VirtualDom::new_with_props(Play, PlayProps { name });
    let _ = app.rebuild();      
    format!(
        "<!DOCTYPE html><html lang='en'>{}</html", 
        dioxus_ssr::render(&app)
    )
}
