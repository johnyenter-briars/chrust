class ChessSync {
    constructor(chessGame, board) {
        this.chessGame = chessGame;
        this.chessBoard = board;
    }

    async test() {
        var response = await fetch("http://localhost:8000/api/test");
        console.log(response);
        console.log(await response.text());
    }

    async print_fen() {
        console.log(this.chessGame.fen());
    }

    // sends the fen of the current state to the API
    // API will return a fen with it's move in resposne
    // if the API says "i can't make that move for whatever reason"..... uhh idk - ill figure that one out
    send_fen() {
        var fen = game.fen();
        debugger;
        var idk = fetch("http://localhost:8000/api/process",
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
            // var responseFen = JSON.parse(text);
            try {
                this.chessBoard.position(text, true);
                
            }catch(err) {
                console.log(err);
            }
        })
        .catch((err) => {
            debugger;
            console.log(err);
        });
        // var response = await fetch("http://localhost:8000/api/process" + fen, {
        //     method: 'POST', // *GET, POST, PUT, DELETE, etc.
        //     mode: 'cors', // no-cors, *cors, same-origin
        //     cache: 'no-cache', // *default, no-cache, reload, force-cache, only-if-cached
        //     credentials: 'same-origin', // include, *same-origin, omit
        //     headers: {
        //         'Content-Type': 'application/json'
        //         // 'Content-Type': 'application/x-www-form-urlencoded',
        //     }
        // });

        // if (response.status_code == 200) {
        //     var responseFen = JSON.parse(await response.text());
        //     this.chessGame.position(responseFen, true);
        // }

    }
}

var board,
    game = new Chess();


// chessSync.test();

/*The "AI" part starts here */

var calculateBestMove = function (game) {
    var newGameMoves = game.ugly_moves();

    return newGameMoves[Math.floor(Math.random() * newGameMoves.length)];
};

/* board visualization and games state handling starts here*/

var onDragStart = function (source, piece, position, orientation) {
    if (game.in_checkmate() === true || game.in_draw() === true ||
        piece.search(/^b/) !== -1) {
        return false;
    }
};

var makeBestMove = function () {
    var bestMove = getBestMove(game);
    game.ugly_move(bestMove);
    board.position(game.fen());
    renderMoveHistory(game.history());
    if (game.game_over()) {
        alert('Game over');
    }
};

var getBestMove = function (game) {
    if (game.game_over()) {
        alert('Game over');
    }
    var bestMove = calculateBestMove(game);
    return bestMove;
};

var renderMoveHistory = function (moves) {
    var historyElement = $('#move-history').empty();
    historyElement.empty();
    for (var i = 0; i < moves.length; i = i + 2) {
        historyElement.append('<span>' + moves[i] + ' ' + (moves[i + 1] ? moves[i + 1] : ' ') + '</span><br>')
    }
    historyElement.scrollTop(historyElement[0].scrollHeight);

};

var getMoveAPI = () => {
    chessSync.send_fen();
}

var onDrop = function (source, target) {
    var move = game.move({
        from: source,
        to: target,
        promotion: 'q'
    });

    removeGreySquares();
    if (move === null) {
        return 'snapback';
    }

    renderMoveHistory(game.history());
    chessSync.print_fen();
    window.setTimeout(getMoveAPI, 250);
};

var onSnapEnd = function () {
    board.position(game.fen());
};

var onMouseoverSquare = function (square, piece) {
    var moves = game.moves({
        square: square,
        verbose: true
    });

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

var cfg = {
    draggable: true,
    position: 'start',
    onDragStart: onDragStart,
    onDrop: onDrop,
    onMouseoutSquare: onMouseoutSquare,
    onMouseoverSquare: onMouseoverSquare,
    onSnapEnd: onSnapEnd
};
board = ChessBoard('board1', cfg);
var chessSync = new ChessSync(game, board);