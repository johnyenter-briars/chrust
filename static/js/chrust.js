class ChrustAPI {
    constructor(board) {
        this.chessBoard = board;
        this.hostname = window.location.hostname;
        this.apiUrl = this.hostname === '127.0.0.1' || this.hostname === '0.0.0.0' || this.hostname === '192.168.0.4'
            ? `http://${this.hostname}:8083/chrust` :
            `http://${this.hostname}/chrust`;

        this.currenlyInWebRequest = false;
        this.rightHalfFen = "w KQkq - 0 1"; //starging position
    }

    // sends the fen of the current state to the API
    // API will return a fen with it's move in resposne
    // if the API says "i can't make that move for whatever reason"..... uhh idk - ill figure that one out
    processFen(enPassantTarget) {
        if(enPassantTarget) {
            this.rightHalfFen = this.rightHalfFen.replace('-', enPassantTarget);
        }
        var updatedFen = setCharAt(this.rightHalfFen, 0, 'b');
        const fullFen = `${this.chessBoard.fen()} ${updatedFen}`;
        console.log("sending fen", fullFen);
        const url = `${this.apiUrl}/api/process/${encodeURIComponent(fullFen)}`;
        notifyMove(true);
        this.currenlyInWebRequest = true;
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

        ).then(async (response) => {
            const responseFen = await response.text()
            console.log("response fen", responseFen);
            const fenElements = responseFen.split(' ');
            const boardSectionFen = fenElements[0];
            const rightHalfOfFen = fenElements.splice(1, 5).join(" ");
            this.rightHalfFen = rightHalfOfFen;
            return responseFen;
        }
        ).then((fen) => {
            this.chessBoard.position(fen, true);
            notifyMove(false);
            this.currenlyInWebRequest = false;
        })
            .catch((err) => {
                return false;
            });
    }

    //location of form "h2"
    validMoves(location) {
        if (!isValidLocationString(location)) return;
        const fullFen = `${this.chessBoard.fen()} ${this.rightHalfFen}`;
        const url = `${this.apiUrl}/api/possible/${encodeURIComponent(fullFen)}/${location}`;
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
            const validMoves = JSON.parse(text);
            return validMoves.options;
        })
            .catch((err) => {
                return false;
            });
    }

    getCurrentSettings() {
        const url = `${this.apiUrl}/api/settings`;
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
            const foo = JSON.parse(text);
            return foo.program_state;
        })
            .catch((err) => {
                return false;
            });
    }

    isValid(currentLocation, possibleLocation, resolve) {
        if (!isValidLocationString(currentLocation) || !isValidLocationString(possibleLocation)) return;
        const fullFen = `${this.chessBoard.fen()} ${this.rightHalfFen}`;
        const url = `${this.apiUrl}/api/validate/${encodeURIComponent(fullFen)}/${currentLocation}/${possibleLocation}`;
        this.currenlyInWebRequest = true;
        const request = new XMLHttpRequest();
        request.open('GET', url, false);
        request.send(null);
        this.currenlyInWebRequest = false;

        if (request.status === 200) {
            const foo = JSON.parse(request.responseText);
            return foo.is_valid;
        }
    }
}

const squareCache = {};

const onSnapEnd = function () {
    // board.position(game.fen());
};

function notifyMove(aiMove) {
    document.getElementById("current-mover").textContent = aiMove ? "Rusty" : "You";
}

const onMouseoverSquare = async (square, piece) => {
    if (chrustAPI.currenlyInWebRequest) return;
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

const onMouseoutSquare = function (square, piece) {
    removeGreySquares();
};

const removeGreySquares = function () {
    $('#board .square-55d63').css('background', '');
};

const greySquare = function (square) {
    const squareEl = $('#board .square-' + square);

    var background = '#a9a9a9';
    if (squareEl.hasClass('black-3c85d') === true) {
        background = '#696969';
    }

    squareEl.css('background', background);
};

const onDrop = (source, target, piece, newPos, oldPos, orientation) => {
    if (chrustAPI.currenlyInWebRequest) return;
    if (source === target) {
        return "snapback";
    }

    var enPassant = null;
    //check for en passant
    if (piece == "wP") {
        for (var i of ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']) {
            if (newPos[`${i}2`] === undefined && oldPos[`${i}2`] !== undefined &&
                newPos[`${i}4`] !== undefined && oldPos[`${i}4`] === undefined
            ) {
                enPassant = `${i}3`;
            }
        }
    }

    if (!chrustAPI.isValid(source, target)) {
        return "snapback";
    }

    window.setTimeout(() => chrustAPI.processFen(enPassant), 500);
}

//https://stackoverflow.com/questions/1431094/how-do-i-replace-a-character-at-a-particular-index-in-javascript
const setCharAt = (str, index, chr) => {
    if (index > str.length - 1) return str;
    return str.substring(0, index) + chr + str.substring(index + 1);
}

const isValidLocationString = (location) => {
    return location.length === 2;
}

const resetBoard = () => {
    chrustAPI.chessBoard.start(true);
}

const config = {
    draggable: true,
    position: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
    onDrop: onDrop,
    onMouseoverSquare: onMouseoverSquare,
    onMouseoutSquare: onMouseoutSquare,
}
const board = Chessboard('board', config)

const chrustAPI = new ChrustAPI(board);

const refreshSettings = async () => {
    const result = await chrustAPI.getCurrentSettings();
    for (key in result) {
        document.getElementById(key).textContent = result[key];
    }
}

refreshSettings();