class ChrustAPI {
    constructor(board) {
        this.chessBoard = board;
    }

    // sends the fen of the current state to the API
    // API will return a fen with it's move in resposne
    // if the API says "i can't make that move for whatever reason"..... uhh idk - ill figure that one out
    send_fen() {
        var fen = this.chessBoard.fen();
        var url = "http://localhost:8000/api/process/" + encodeURIComponent(fen);
        fetch(url,
            {
                method: 'POST', // *GET, POST, PUT, DELETE, etc.
                mode: 'cors', // no-cors, *cors, same-origin
                cache: 'no-cache', // *default, no-cache, reload, force-cache, only-if-cached
                credentials: 'same-origin', // include, *same-origin, omit
                headers: {
                    'Content-Type': 'application/json'
                    // 'Content-Type': 'application/x-www-form-urlencoded',
                }
            }

        ).then((response) => { return response.text() }
        ).then((fen) => {
            console.log(fen);
            this.chessBoard.position(fen, true);
        })
        .catch((err) => {
            console.log(err);
            return false;
        });
    }

    //location of form "h2"
    validMoves(location) {
        debugger;
        var fen = this.chessBoard.fen();
        var url = "http://localhost:8000/api/validate/" + encodeURIComponent(fen) + "/" + location;
        return fetch(url,
            {
                method: 'GET', // *GET, POST, PUT, DELETE, etc.
                mode: 'cors', // no-cors, *cors, same-origin
                cache: 'no-cache', // *default, no-cache, reload, force-cache, only-if-cached
                credentials: 'same-origin', // include, *same-origin, omit
                headers: {
                    'Content-Type': 'application/json'
                    // 'Content-Type': 'application/x-www-form-urlencoded',
                }
            }

        ).then((response) => { return response.text() }
        ).then((text) => {
            debugger;
            console.log(text);
            var foo = JSON.parse(text);
            return foo.options;
        })
        .catch((err) => {
            console.log(err);
            return false;
        });
    }
}

var board,
    game = new Chess();



/*The "AI" part starts here */

// var calculateBestMove = function (game) {
//     var newGameMoves = game.ugly_moves();

//     return newGameMoves[Math.floor(Math.random() * newGameMoves.length)];
// };

/* board visualization and games state handling starts here*/

var onDragStart = function (source, piece, position, orientation) {
    if (game.in_checkmate() === true || game.in_draw() === true ||
        piece.search(/^b/) !== -1) {
        return false;
    }
};

// var makeBestMove = function () {
//     var bestMove = getBestMove(game);
//     game.ugly_move(bestMove);
//     board.position(game.fen());
//     renderMoveHistory(game.history());
//     if (game.game_over()) {
//         alert('Game over');
//     }
// };

// var getBestMove = function (game) {
//     if (game.game_over()) {
//         alert('Game over');
//     }
//     var bestMove = calculateBestMove(game);
//     return bestMove;
// };

var renderMoveHistory = function (moves) {
    var historyElement = $('#move-history').empty();
    historyElement.empty();
    for (var i = 0; i < moves.length; i = i + 2) {
        historyElement.append('<span>' + moves[i] + ' ' + (moves[i + 1] ? moves[i + 1] : ' ') + '</span><br>')
    }
    historyElement.scrollTop(historyElement[0].scrollHeight);

};

var getMoveAPI = () => {
    chrustAPI.send_fen();
}

var onSnapEnd = function () {
    // board.position(game.fen());
};

var onMouseoverSquare = (square, piece) => {
    console.log("test");
    debugger;
    // var game = Chess();
    // var moves = game.moves({
    //     square: square,
    //     verbose: true
    // });

    //moves needs to be in format: ["h3", "h4"]
    return;
    var moves = chrustAPI.validMoves(square);

    debugger;
    if (moves.length === 0) return;

    greySquare(square);

    for (var i = 0; i < moves.length; i++) {
        greySquare(moves[i].to);
    }
};

var onMouseoutSquare = function (square, piece) {
    removeGreySquares();
};

var removeGreySquares = function () {
    $('#board .square-55d63').css('background', '');
};

var greySquare = function (square) {
    var squareEl = $('#board .square-' + square);

    var background = '#a9a9a9';
    if (squareEl.hasClass('black-3c85d') === true) {
        background = '#696969';
    }

    squareEl.css('background', background);
};

function onDrop(source, target, piece, newPos, oldPos, orientation) {
    if (source === target) {
        return "snapback";
    }

    window.setTimeout(getMoveAPI, 500);
}

var config = {
    draggable: true,
    position: 'start',
    onDrop: onDrop,
    onMouseoverSquare: onMouseoverSquare,
}
var board = Chessboard('board1', config)

var chrustAPI = new ChrustAPI(board);

function resetBoard() {
    board.start(true);
}