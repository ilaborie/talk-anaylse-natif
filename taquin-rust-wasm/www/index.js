import * as wasm from "taquin_rust_wasm";

const parentElt = document.querySelector("main");

const btnNewTaquin = parentElt.querySelector("aside button.new");
const taquinElt = parentElt.querySelector("pre");
const scoreElt = parentElt.querySelector("aside .score");
const btnUp = parentElt.querySelector("aside button.up");
const btnRight = parentElt.querySelector("aside button.right");
const btnDown = parentElt.querySelector("aside button.down");
const btnLeft = parentElt.querySelector("aside button.left");

let state = {};

function render(s) {
    const {taquin, score} = s;
    if (!taquin) return;

    if (taquin.is_solved()) {
        taquinElt.classList.add("solved");
    }
    taquinElt.innerHTML = taquin.render();
    scoreElt.innerHTML = score;
}

function move(s, move) {
    const {taquin, score} = s;
    if (!taquin) return;

    let moved = taquin.move_hole(move);
    state = {taquin, score: score + (moved ? 1 : 0)};
    render(state);
}

btnNewTaquin.addEventListener('click', function () {
    console.log('New');
    const taquin = wasm.Taquin.new(3);
    state = {taquin, score: 0};
    render(state);
});


document.body.addEventListener('keyup', (event)  => {
    let {taquin} = state;
    if (!taquin|| taquin.is_solved()) return;
    switch (event.key) {
        case "ArrowUp":
            move(state, wasm.Move.Down);
            return;
        case "ArrowRight":
            move(state, wasm.Move.Left);
            return;
        case "ArrowDown":
            move(state, wasm.Move.Up);
            return;
        case "ArrowLeft":
            move(state, wasm.Move.Right);
            return;
    }
});
btnUp.addEventListener('click', () => {
    let {taquin} = state;
    if (!taquin|| taquin.is_solved()) return;
    move(state, wasm.Move.Down);
});
btnRight.addEventListener('click', () => {
    let {taquin} = state;
    if (!taquin|| taquin.is_solved()) return;
    move(state, wasm.Move.Left);
});
btnDown.addEventListener('click', () => {
    let {taquin} = state;
    if (!taquin|| taquin.is_solved()) return;
    move(state, wasm.Move.Up);
});
btnLeft.addEventListener('click', () => {
    let {taquin} = state;
    if (!taquin|| taquin.is_solved()) return;
    move(state, wasm.Move.Right);
});
