class ChrustAPI {
    constructor(board) {
        this.chessBoard = board;
        this.hostname = window.location.hostname;
        this.apiUrl = `http://${this.hostname}:8000`;
    }

    // sends the fen of the current state to the API
    // API will return a fen with it's move in resposne
    // if the API says "i can't make that move for whatever reason"..... uhh idk - ill figure that one out
    send_fen() {
        var fen = this.chessBoard.fen();
        var url = `${this.apiUrl}/api/process/${encodeURIComponent(fen)}`;
        fetch(url,
            {
                method: 'POST', 
                mode: 'cors',
                cache: 'no-cache', 
                credentials: 'same-origin',
                headers: {
                    'Content-Type': 'application/json'
                }
            }

        ).then((response) => { return response.text() }
        ).then((fen) => {
            this.chessBoard.position(fen, true);
        })
            .catch((err) => {
                console.log(err);
                return false;
            });
    }

    //location of form "h2"
    validMoves(location) {
        var fen = this.chessBoard.fen();
        var url = `${this.apiUrl}/api/possible/${encodeURIComponent(fen)}/${location}`;
        return fetch(url,
            {
                method: 'GET',
                mode: 'cors',
                cache: 'no-cache',
                credentials: 'same-origin',
                headers: {
                    'Content-Type': 'application/json'
                }
            }

        ).then((response) => { return response.text() }
        ).then((text) => {
            console.log(text);
            var foo = JSON.parse(text);
            return foo.options;
        })
            .catch((err) => {
                console.log(err);
                return false;
            });
    }

    isValid(currentLocation, possibleLocation, resolve) {
        var fen = this.chessBoard.fen();
        var url = `${this.apiUrl}/api/validate/${encodeURIComponent(fen)}/${currentLocation}/${possibleLocation}`;
        var request = new XMLHttpRequest();
        request.open('GET', url, false);
        request.send(null);

        if (request.status === 200) {
            console.log(request.responseText);
            var foo = JSON.parse(request.responseText);
            return foo.is_valid;
        }
    }
}

var board;

var squareCache = {};

var onDragStart = function (source, piece, position, orientation) {
    if (game.in_checkmate() === true || game.in_draw() === true ||
        piece.search(/^b/) !== -1) {
        return false;
    }
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
    chrustAPI.send_fen();
}

var onSnapEnd = function () {
    // board.position(game.fen());
};

var onMouseoverSquare = async (square, piece) => {
    if (!piece) return;
    //moves needs to be in format: ["h3", "h4"]
    var moves = squareCache[[square, chrustAPI.chessBoard.fen()]];

    if (!moves) {
        moves = await chrustAPI.validMoves(square);
    }

    if (moves.length === 0) return;

    squareCache[[square, chrustAPI.chessBoard.fen()]] = moves;

    greySquare(square);

    for (var i = 0; i < moves.length; i++) {
        greySquare(moves[i].x + moves[i].y);
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

var onDrop = (source, target, piece, newPos, oldPos, orientation) => {
    if (source === target) {
        return "snapback";
    }

    if (!chrustAPI.isValid(source, target)) {
        return "snapback";
    }

    window.setTimeout(getMoveAPI, 500);
}

var config = {
    draggable: true,
    position: 'start',
    onDrop: onDrop,
    onMouseoverSquare: onMouseoverSquare,
    onMouseoutSquare: onMouseoutSquare,
}
var board = Chessboard('board', config)

var chrustAPI = new ChrustAPI(board);

function resetBoard() {
    board.start(true);
}