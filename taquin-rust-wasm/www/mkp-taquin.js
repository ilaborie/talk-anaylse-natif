import {Taquin, Move} from "taquin_rust_wasm";

// Web component
export class MkpTaquinElt extends HTMLElement {

    constructor() {
        super();
        this.taquin = null;
        this.score = 0;
        this.actions = {
            newTaquin: () => this.new(),
            up: () => this.move(Move.Up),
            right: () => this.move(Move.Right),
            down: () => this.move(Move.Down),
            left: () => this.move(Move.Left)
        };
    }

    connectedCallback() {
        this.init();
    }

    adoptedCallback() {
        this.init();
    }

    static get observedAttributes() {
        return ['size'];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === 'size' && oldValue !== newValue) {
            this.new();
        }
    }

    init() {
        if (!this.isConnected) return;

        const shadow = this.attachShadow({mode: "open"});
        const styleElt = document.createElement("style");
        this.taquinElt = document.createElement("pre");
        const asideElt = document.createElement("aside");

        shadow.appendChild(styleElt);
        shadow.appendChild(this.taquinElt);
        shadow.appendChild(asideElt);

        asideElt.innerHTML = `
<button type="button" class="newTaquin">➕ New</button>
<output class="score"></output>
<div class="pad">
    <button type="button" class="up">⬆️</button>
    <button type="button" class="left">⬅️</button>
    <button type="button" class="hole" disabled>⚫️</button>
    <button type="button" class="right">➡️</button>
    <button type="button" class="down">⬇️</button>
</div>`;
        this.scoreElt = asideElt.querySelector(".score");

        // Event Bindings
        asideElt.querySelectorAll("button").forEach((btn) => {
            const className = btn.classList.item(0);
            const action = this.actions[className];
            if (action) {
                btn.addEventListener('click', action);
            }
        });

        // Load Style
        fetch('./taquin.css')
            .then((response) => response.text())
            .then(content => {
                styleElt.innerHTML = content
            });
    }

    new() {
        const size = this.getAttribute("size") || 3;
        const taquin = Taquin.new(size);
        this.state = {taquin, score: 0};
        this.render();
    }

    render() {
        const {taquin, score} = this.state;
        if (!taquin) return;

        if (taquin.is_solved()) {
            this.taquinElt.classList.add("solved");
        }
        this.taquinElt.innerHTML = taquin.render();
        this.scoreElt.innerHTML = score;
    }

    move(move) {
        const {taquin, score} = this.state;
        if (!taquin || taquin.is_solved()) return;

        let moved = taquin.move_hole(move);

        this.state = {taquin, score: score + (moved ? 1 : 0)};

        this.render();
    }
}