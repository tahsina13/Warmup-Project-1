// code for battleship game

use chrono::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};

pub const BATTLESHIP_GET_PAGE: &str = r#"
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
"#;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tile {
    Hit,
    Miss,
    Untried,
    Ship,
}
pub use Tile::*;

pub fn create_battleship_game(rows: usize, cols: usize, ships: &[usize]) -> Vec<Vec<Tile>> {
    let mut board = vec![vec![Untried; cols]; rows];
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
                        if board[r][j] != Untried {
                            continue 'place;
                        }
                    }
                    for j in c..(c + length) {
                        board[r][j] = Ship;
                    }
                    break;
                }
            }
            1 => {
                // vertical
                'place: for _ in 0..MAX_ATTEMPTS {
                    let r = rng.gen_range(0..(rows - length));
                    let c = rng.gen_range(0..cols);
                    for row in &board[r..r + length] {
                        if row[c] != Untried {
                            continue 'place;
                        }
                    }
                    for row in &mut board[r..r + length] {
                        row[c] = Ship;
                    }
                    break;
                }
            }
            _ => unreachable!(),
        }
    }
    board
}

pub const ROWS: usize = 5;
pub const COLS: usize = 7;
pub const SHIPS: [usize; 3] = [2, 3, 4];

pub fn make_board_page(name: String, board: Vec<Vec<Tile>>, moves_left: i32) -> String {
    let time_formatted = Utc::now().format("%Y-%m-%d");

    let max_hits: usize = SHIPS.iter().sum();
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
<form action="/battleship.php" method="POST">
    <input type="submit" name="play_again" value="Play again">
</form>"#
    } else if moves_left == 0 {
        r#"You lose!
<form action="/battleship.php" method="POST">
    <input type="submit" name="play_again" value="Play again">
</form>"#
    } else {
        ""
    };

    format!(
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
    )
}
