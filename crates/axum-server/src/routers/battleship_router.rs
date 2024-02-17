use axum::{response::Html, routing::get, Form};
use chrono::Local;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use rand::Rng;

#[derive(Deserialize)]
struct GameForm {
    name: Option<String>,
    r#move: Option<String>,
    reset: Option<String>,
}

pub fn new_battleship_router() -> axum::Router {
    axum::Router::new()
        .route("/", get(get_form_handler).post(post_form_handler))
}

async fn get_form_handler() -> Html<&'static str> {
    Html(
        r#"
<!DOCTYPE html>
<html>
<head>
    <link rel="stylesheet" href="/battleship.css"/>
<head>
<body>
    <form action="battleship.php" method="post">
        <label for="name">Name:</label>
        <input type="text" id="name" name="name"/>
        <input type="submit" value="Submit"/>
    </form>
</body>
</html>
"#,
    )
}

const NAME_KEY: &str = "name";
const MOVES_KEY: &str = "moves_left";
const BOARD_KEY: &str = "board";

async fn post_form_handler(session: Session, Form(form): Form<GameForm>) -> Html<String> {
    //if let Form(form) = Form::<StartGameForm>::from_request(req).await
    let time_formatted = Local::now().format("%Y-%m-%d");

    // process name
    if let Some(name) = form.name {
        session.insert(NAME_KEY, name).await.unwrap();
    } else if form.reset.is_some() {
        // must reset move and board before loading it for rendering
        let _: Option<i32> = session.remove(MOVES_KEY).await.unwrap();
        let _: Option<Vec<Vec<Tile>>> = session.remove(BOARD_KEY).await.unwrap();
    }

    const ROWS: usize = 5;
    const COLS: usize = 7;
    const SHIPS: [usize; 3] = [2, 3, 4];
    let max_hits: usize = SHIPS.iter().sum();

    let mut moves_left = session
        .get(MOVES_KEY)
        .await
        .unwrap()
        .unwrap_or(((COLS as f64) * (ROWS as f64) * 0.60).ceil() as i32);

    let mut board = session
        .get(BOARD_KEY)
        .await
        .unwrap()
        .unwrap_or(create_battleship_game(ROWS, COLS, &SHIPS));

    if let Some(move_str) = form.r#move {
        let mut it = move_str.split(',');
        let (i, j): (usize, usize) = (
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
        );
        match board[i][j] {
            Ship => board[i][j] = Hit,
            Untried => board[i][j] = Miss,
            _ => {}
        }
        moves_left -= 1;
    }

    session.insert(MOVES_KEY, moves_left).await.unwrap();

    let name = session
        .get(NAME_KEY)
        .await
        .unwrap()
        .unwrap_or("".to_owned());

    let total_hits: usize = board
        .iter()
        .map(|row| row.iter().filter(|&&tile| tile == Hit).count())
        .sum();

    let mut table_rows = "".to_owned();
    board.iter().enumerate().for_each(|(i, row)| {
        table_rows += "<tr>";
        row.iter().enumerate().for_each(|(j, tile)| {
            table_rows += "<td>";
            match tile {
                Hit => {
                    table_rows += "X";
                }
                Miss => table_rows += "O",
                Untried | Ship => {
                    if moves_left > 0 && total_hits < max_hits {
                        table_rows +=
                            format!(r#"<button type="submit" name="move" value={i},{j}>?</button>"#)
                                .as_str()
                    }
                }
            };
            table_rows += "</td>";
        });
        table_rows += "</tr>";
    });

    session.insert(BOARD_KEY, board).await.unwrap();

    let table = format!(
        r#"
    <table>
    <tbody>
    {table_rows}
    </tbody>
    </table>
"#
    );

    let play_again = if total_hits == max_hits {
        r#"You win!
<form method="post">
    <button type="submit" name="reset">Play again</button>
</form>"#
    } else if moves_left == 0 {
        r#"You lose!
<form method="post">
    <button type="submit" name="reset">Play again</button>
</form>"#
    } else {
        ""
    };

    Html(format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <link rel="stylesheet" href="/battleship.css"/>
<head>
<body>
    Hello {name}, {time_formatted}<br/>
    Moves left: {moves_left}
    <form method="post">
    {table}
    </form>
    {play_again}
</body>
</html>
"#,
    ))
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum Tile {
    Hit,
    Miss,
    Untried,
    Ship,
}
use Tile::*;

fn create_battleship_game(rows: usize, cols: usize, ships: &[usize]) -> Vec<Vec<Tile>> {
    let mut res = vec![vec![Untried; cols]; rows];
    let mut rng = rand::thread_rng();
    const MAX_ATTEMPTS: usize = 100;
    for length in ships {
        match rng.gen_range(0..2) {
            0 => {
                // horizontal
                'place: for _ in 0..MAX_ATTEMPTS {
                    let r = rng.gen_range(0..rows);
                    let c = rng.gen_range(0..(cols - length));
                    for j in c..(c + length) {
                        if res[r][j] != Untried {
                            continue 'place;
                        }
                    }
                    for j in c..(c + length) {
                        res[r][j] = Ship;
                    }
                    break;
                }
            }
            1 => {
                // vertical
                'place: for _ in 0..MAX_ATTEMPTS {
                    let r = rng.gen_range(0..(rows - length));
                    let c = rng.gen_range(0..cols);
                    for row in &res[r..r + length] {
                        if row[c] != Untried {
                            continue 'place;
                        }
                    }
                    for row in &mut res[r..r + length] {
                        row[c] = Ship;
                    }
                    break;
                }
            }
            _ => unreachable!(),
        }
    }
    res
}
