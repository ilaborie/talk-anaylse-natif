import {Move} from "taquin_rust_wasm";
import {MkpTaquinElt} from "./mkp-taquin";

// Register WebComponent
customElements.define('mkp-taquin', MkpTaquinElt);

// Actions
const keyActions = {
    ArrowUp: (taquin) => taquin.move(Move.Up),
    ArrowRight: (taquin) => taquin.move(Move.Right),
    ArrowDown: (taquin) => taquin.move(Move.Down),
    ArrowLeft: (taquin) => taquin.move(Move.Left),
    '3': (taquin) => {
        if (taquin.size !== 3) {
            taquin.setAttribute('size', '3');
        }
    },
    '4': (taquin) => {
        if (taquin.size !== 4) {
            taquin.setAttribute('size', '4');
        }
    }
};

// Event Bindings
document.body.addEventListener('keyup', (event) => {
    const action = keyActions[event.key];
    if (action) {
        document.body.querySelectorAll('mkp-taquin')
            .forEach(action);
    }
});
