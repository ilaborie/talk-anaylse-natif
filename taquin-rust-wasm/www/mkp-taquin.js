import {Taquin, Move} from "taquin_rust_wasm";
import {memory} from "taquin_rust_wasm/taquin_rust_wasm_bg";

const states = ['init', 'game', 'win'];

// Web component
export class MkpTaquinElt extends HTMLElement {

    get size() {
        return this.getAttribute("size") || 3;
    }

    constructor() {
        super();
        this.taquin = null;
        this.score = 0;
        this.state = 'init'; // game success
        this.actions = {
            newTaquin: () => this.new(),
            up: () => this.move(Move.Up),
            right: () => this.move(Move.Right),
            down: () => this.move(Move.Down),
            left: () => this.move(Move.Left)
        };
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

        const styleElt = document.createElement("style");

        this.taquinElt = document.createElement("div");
        this.taquinElt.classList.add("grid", "grid-" + this.size);
        const lastSize = this.size ** 2;
        for (let i = 1; i < lastSize; i++) {
            const tile = document.createElement("div");
            tile.classList.add("tile")
            tile.setAttribute('data-value', '' + i);
            const span = document.createElement("span");
            tile.appendChild(span);
            this.taquinElt.appendChild(tile);
        }

        const asideElt = document.createElement("aside");
        asideElt.innerHTML = `
<button type="button" class="newTaquin">➕ New</button>
<div class="pad">
    <button type="button" class="up">⬆️</button>
    <button type="button" class="left">⬅️</button>
    <button type="button" class="hole" disabled>⚫️</button>
    <button type="button" class="right">➡️</button>
    <button type="button" class="down">⬇️</button>
</div>
<output class="score"></output>`;
        this.scoreElt = asideElt.querySelector(".score");
        this.padElt = asideElt.querySelector(".pad");

        // Add all
        const shadow = this.shadowRoot || this.attachShadow({mode: "open"});
        shadow.innerHTML = '';
        shadow.appendChild(styleElt);
        shadow.appendChild(this.taquinElt);
        shadow.appendChild(asideElt);

        // Event Bindings
        asideElt.querySelectorAll("button")
            .forEach((btn) => {
                const className = btn.classList.item(0);
                const action = this.actions[className];
                if (action) {
                    this.buttons[className] = btn;
                    btn.addEventListener('click', action);
                }
            });
        this.taquinElt.querySelectorAll('span')
            .forEach((elt) => {
                elt.addEventListener('click', () => {
                    if (elt.hasAttribute('data-move')) {
                        const move = +elt.getAttribute('data-move');
                        this.move(move);
                    }
                });
            });

        // Load Style
        fetch('./taquin.css')
            .then((response) => response.text())
            .then(content => {
                styleElt.innerHTML = content
            });

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
        // State
        if (this.shadowRoot && this.state) {
            const toRemove = states.filter((it) => it === this.state);
            this.shadowRoot.host.classList.remove(toRemove);
            this.shadowRoot.host.classList.add(this.state);
        }

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
            } else {
                this.taquinElt.style.visibility = "hidden";
            }
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
                Object.entries(this.buttons).forEach(([key, btn]) => {
                    const hole = this.taquin.find_hole();
                    switch (key) {
                        case "up":
                            btn.disabled = !this.taquin.is_valid(Move.Up, hole);
                            break;
                        case "right":
                            btn.disabled = !this.taquin.is_valid(Move.Right, hole);
                            break;
                        case "down":
                            btn.disabled = !this.taquin.is_valid(Move.Down, hole);
                            break;
                        case "left":
                            btn.disabled = !this.taquin.is_valid(Move.Left, hole);
                            break;
                    }
                });
            }
        }
    }

    move(move) {
        if (!this.taquin) return;

        const {moved, win, from, to} = this.taquin.move_hole(move);

        this.state = win ? 'win' : 'game';
        this.score += moved ? 1 : 0;

        this.render();
    }
}