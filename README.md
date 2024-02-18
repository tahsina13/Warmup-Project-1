# Warmup-Project-1

## Instructions

Description
1. Create a front page at http://yourserver/ttt.php – the page must include at
least one CSS file which changes the appearance of something on the page and a
GET form that requests and submits a field called 'name' (the form action should
point back to itself at /ttt.php).

2. If the page receives a GET parameter called 'name', it should instead display
'Hello $name, $date' with the name and date filled in dynamically on an HTML page.
(do not use client-side JavaScript)

3. When loaded with the name, the /ttt.php page should also, below the "Hello..."
line, output a Tic-Tac-Toe board, where each cell where a legal move can be made
contains a link with a URL that performs that move, passing two GET parameters
to the /ttt.php script, the "name" parameter and a "board" parameter which is a
space-separated string of X and O values (when the board is empty, the board
parameter is a string comprising 8 spaces, corresponding to 9 empty cells).
When only the name is passed, the board should be empty and the user of the web
page should be making the first X move. The cells must literally contain the
string X or O in the HTML code.  (do not use client-side JavaScript)

4. When /ttt.php receives a request with both a name and a board parameter, there
are three possibilities.  If the X has won, the generated response page should
include the string "You won!".  If X has not won, the system should respond with
a board with an automated O move.  If the O move wins the game, the response page
should include the string "I won!".  If the response indicates a game that was won
or lost (but not tied), include a "Play again" link, which will load /ttt.php with
only the name parameter.  If the game was tied, the response page should include
the string "WINNER: NONE.  A STRANGE GAME.  THE ONLY WINNING MOVE IS NOT TO PLAY.".

5. Similar to the above, create a /connect.php page for a Connect-4 game which
also asks for a name before allowing to play the game (and shows a greeting with
the time once a name is provided).  The game board should be 5 rows by 7 columns.
Instead of using GET parameters, all Connect-4 actions (including the name
submission) should be sent as POST requests using a FORM element.  Above each
column, there should be a <button type="submit" name="board" ...> element which
submits the new game "board" parameter that includes the move being made in that
column.  If a column is full, do not show the button to make a move in that column.
The format of the "board" parameter should be a string which joins the data from
all rows using dots and joins the data for all columns within a row with spaces
(e.g., "X      .O      .X      .O      .O O X X X  " for a board with the bottom
row containing O O X X X and the last two cells being empty).  When a game is
over, the page should include the text "You won!", "I won!", or "Draw", and a
button to "Play again".

6. Similar to the above, create a /battleship.php page for a one-sided Battleship
game which also asks for a name before allowing to play the game (and shows a
greeting with the time once a name is provided).  The board should be 5 rows by 7
columns.  Instead of maintaining all state in the client and submitting it to the
server, maintain all state in a server-side session.  At the start of a game, the
server should select secret locations of three ships, 2x1, 3x1, and 4x1, in
non-overlapping horizontal or vertical directions.  The top of the page should
also include the string: "Moves left: (integer)" where the integer is the number
of remaining moves.  The number of moves should start out as being
ceil($columns * $rows * 0.60).  During gameplay, cells where moves are made
should reflect the result: 'X' for a hit, 'O' for a miss, and '?' for an untried
cell (only visible when the game is not over).  For making moves, instead of a
"board" parameter, submit a "move" parameter whose value is 0-indexed $row,$col
of the move being made.  The game is over if (a) the player has run out of moves
(indicate this by "You lose!") or (b) all of the ships are sunk (indicate this
by "You win!").  When the game is over, a "Play again" button should be shown.

Note: All responses must contain the header field X-CSE356 with the value containing the submission ID  (click on 'Copy ID' below to get it).

## Config

Example config.toml:

```TOML
ip = [127, 0, 0, 1]
submission_id = "foobarbooblaz1234"
http_port = 80
```

## To Run

From root:

```Shell
cargo build
cargo install cargo-watch
cargo watch -x run
```

```Shell
# allow binary to bind to low-number ports
sudo setcap CAP_NET_BIND_SERVICE=+eip target/debug/axum-server
./target/debug/axum-server
```
