import {Taquin, Move} from "taquin_rust_wasm";
import {memory} from "taquin_rust_wasm/taquin_rust_wasm_bg";
import css from './taquin.css';

const allMoves = [];
allMoves[Move.Up] = Move.Up;
allMoves[Move.Right] = Move.Right;
allMoves[Move.Down] = Move.Down;
allMoves[Move.Left] = Move.Left;

const moveLabels = [];
moveLabels[Move.Up] = '⬆️';
moveLabels[Move.Right] = '➡️';
moveLabels[Move.Down] = '⬇️';
moveLabels[Move.Left] = '⬅️';

const moveClassName = [];
moveClassName[Move.Up] = 'up';
moveClassName[Move.Right] = 'right';
moveClassName[Move.Down] = 'down';
moveClassName[Move.Left] = 'left';

const delay = (delayInMs, block) => new Promise(resolve => {
    setTimeout(function () {
        resolve(block && block());
    }, delayInMs);
});

function createButtonElement(label, className, clickEvent) {
    const btn = document.createElement("button");
    btn.type = 'button';
    if (label) {
        btn.textContent = label;
    }

    if (className) {
        btn.classList.add(className);
    }

    if (clickEvent) {
        btn.addEventListener('click', clickEvent);
    }

    return btn;
}

// Web component
export class MkpTaquinElt extends HTMLElement {

    get size() {
        return this.getAttribute("size") || 3;
    }

    constructor() {
        super();
        this.taquin = null;
        this.score = 0;
        this.state = 'init';
        this.buttons = {};
    }

    connectedCallback() {
        console.log('connectedCallback');
        this.init();
    }

    adoptedCallback() {
        console.log('adoptedCallback');
        this.init();
    }

    static get observedAttributes() {
        return ['size'];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        console.log('attributeChangedCallback');
        if (name === 'size' && oldValue !== newValue) {
            this.new();
            this.init();
        }
    }

    init() {
        if (!this.isConnected) return;

        // Style
        const styleElt = document.createElement("style");
        styleElt.textContent = css;

        // Taquin
        this.taquinElt = document.createElement("div");
        this.taquinElt.classList.add("grid");
        this.taquinElt.style.setProperty("--size", this.size);
        const lastSize = this.size ** 2;
        for (let i = 1; i < lastSize; i++) {
            const tile = document.createElement("div");
            tile.classList.add("tile")
            tile.setAttribute('data-value', '' + i);
            const span = document.createElement("span");
            tile.appendChild(span);
            this.taquinElt.appendChild(tile);

            span.addEventListener('click', () => {
                if (this.state === 'game' && span.hasAttribute('data-move')) {
                    const move = +span.getAttribute('data-move');
                    this.move(move);
                }
            });
        }

        // Left block
        const asideElt = document.createElement("aside");
        this.btnNewTaquin = createButtonElement('➕ New', 'newTaquin', () => this.new());
        asideElt.appendChild(this.btnNewTaquin);
        this.btnCheat = createButtonElement('🤖 Cheat', 'cheat', () => this.cheat());
        asideElt.appendChild(this.btnCheat);
        this.padElt = document.createElement('div');
        asideElt.appendChild(this.padElt);
        this.padElt.classList.add('pad');
        const btnHole = createButtonElement('⚫️', 'hole');
        btnHole.disabled = true;
        this.padElt.appendChild(btnHole);
        allMoves.forEach(move => {
            const className = moveClassName[move];
            const btn = createButtonElement(moveLabels[move], className, () => {
                if (this.state === 'game') {
                    this.move(move);
                }
            });
            this.buttons[className] = btn;
            this.padElt.appendChild(btn);
        });
        asideElt.appendChild(this.padElt);
        this.scoreElt = document.createElement('output');
        this.scoreElt.classList.add('score');
        asideElt.appendChild(this.scoreElt);

        // Add all
        const shadow = this.shadowRoot || this.attachShadow({mode: "open"});
        shadow.innerHTML = '';
        shadow.appendChild(styleElt);
        shadow.appendChild(this.taquinElt);
        shadow.appendChild(asideElt);

        this.render();
    }

    new() {
        const size = this.size;
        console.log(`New taquin ${size}x${size}`);
        this.taquin = Taquin.new(size);
        this.taquin.shuffle(size ** 4 * 2);
        this.score = 0;
        this.state = 'game';
        this.render();
    }

    render() {
        // Grid
        if (this.taquinElt) {
            if (this.taquin) {
                this.taquinElt.style.visibility = "visible";

                let tilesPtr = this.taquin.tiles();
                const tiles = new Uint8Array(memory.buffer, tilesPtr, this.size * this.size);
                for (let i = 0; i < tiles.length; i++) {
                    const value = tiles[i];
                    if (value !== 0) {
                        const elt = this.taquinElt.childNodes.item(value - 1);
                        if (elt) {
                            const position = this.taquin.get_position(i);
                            const {row, column} = position;
                            elt.style.setProperty('--column', '' + column);
                            elt.style.setProperty('--row', '' + row);
                            const move = this.taquin.move_from_position(position);
                            if (typeof move === 'undefined') {
                                elt.removeAttribute("data-move");
                            } else {
                                elt.setAttribute("data-move", "" + move);
                            }
                        }
                    }
                }
            } else {
                this.taquinElt.style.visibility = "hidden";
            }
        }

        // New Taquin
        if (this.btnNewTaquin) {
            this.btnNewTaquin.style.visibility = (this.state === 'win' || this.state === 'game') ? 'visible' : 'hidden'
        }

        // Cheat
        if (this.btnCheat) {
            this.btnCheat.style.visibility = (this.size === 4 || this.state !== 'game') ? 'hidden' : 'visible';
        }

        // Score
        if (this.scoreElt && typeof this.score === 'number') {
            if (this.state == 'win') {
                this.scoreElt.innerHTML = '🎉 ' + this.score;
            } else {
                switch (this.score) {
                    case 0:
                        this.scoreElt.innerHTML = '';
                        break;
                    case 1 :
                        this.scoreElt.innerHTML = 'Move :' + this.score;
                        break;
                    default:
                        this.scoreElt.innerHTML = 'Moves:' + this.score;
                }
            }
        }

        // Pad
        if (this.padElt) {
            this.padElt.style.visibility = this.state == "game" ? "visible" : "hidden";
            if (this.taquin) {
                Object.entries(this.buttons)
                    .forEach(([key, btn]) => {
                        const hole = this.taquin.find_hole();
                        switch (key) {
                            case "up":
                                btn.disabled = this.state !== 'game' || !this.taquin.is_valid(Move.Up, hole);
                                break;
                            case "right":
                                btn.disabled = this.state !== 'game' || !this.taquin.is_valid(Move.Right, hole);
                                break;
                            case "down":
                                btn.disabled = this.state !== 'game' || !this.taquin.is_valid(Move.Down, hole);
                                break;
                            case "left":
                                btn.disabled = this.state !== 'game' || !this.taquin.is_valid(Move.Left, hole);
                                break;
                        }
                    });
            }
        }
    }

    move(move) {
        if (!this.taquin || this.state !== 'game') return;
        this.innerMove(move);
    }

    innerMove(move) {
        if (this.taquin) {
            const moved = this.taquin.move_hole(move);

            this.state = this.taquin.is_solved() ? 'win' : this.state;
            this.score += moved ? 1 : 0;

            this.render();
        }
    }

    cheat() {
        console.log('Cheat');
        const solution = this.taquin.solve();
        let count = solution.size();
        console.log('Solved in', count, 'moves');
        const moves = new Uint8Array(memory.buffer, solution.moves(), count);

        this.state = 'cheat';
        let p = Promise.resolve();
        for (let move of moves) {
            p = p.then(() => delay(300, () => this.innerMove(move)));
        }
    }
}