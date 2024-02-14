// code for connect4 game
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
struct GameProps {
    name: String,
}

#[component]
fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            h1 { "Connect 4" }
            form {
                action: "/connect",
                method: "POST",
                label { r#for: "name", "Name: "}
                input { id: "name", name: "name", r#type: "text", required: true } 
                input { r#type: "submit", value: "Start" }
            }
        }
    })
}

#[component]
fn Game(cx: Scope<GameProps>) -> Element {
    let name = &cx.props.name;
    let now = chrono::Utc::now().to_rfc2822();
    cx.render(rsx! {
        p { "Hello {name}, {now}" }
    })
}

pub fn get_form() -> String {
    let mut app = VirtualDom::new(Home);
    let _ = app.rebuild();     
    format!("<!DOCTYPE html><html lang='en'>{}</html", dioxus_ssr::render(&app))
}

pub fn accept_from(name: String) -> String {
    let mut app = VirtualDom::new_with_props(
        Game, 
        GameProps { name }
    );
    let _ = app.rebuild();      
    format!("<!DOCTYPE html><html lang='en'>{}</html", dioxus_ssr::render(&app))
}