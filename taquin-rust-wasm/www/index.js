import {Move} from "taquin_rust_wasm";
import {MkpTaquinElt} from "./mkp-taquin";

// Register WebComponent
customElements.define('mkp-taquin', MkpTaquinElt);

// Actions
const keyActions = {
    ArrowUp: (taquin) => taquin.move(Move.Up),
    ArrowRight: (taquin) => taquin.move(Move.Right),
    ArrowDown: (taquin) => taquin.move(Move.Down),
    ArrowLeft: (taquin) => taquin.move(Move.Left)
};

// Event Bindings
document.body.addEventListener('keyup', (event) => {
    const action = keyActions[event.key];
    if (action) {
        document.body.querySelectorAll('mkp-taquin')
            .forEach(action);
    }
});
