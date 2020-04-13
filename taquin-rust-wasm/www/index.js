import {Move} from "taquin_rust_wasm";
import {MkpTaquinElt} from "./mkp-taquin";


// Register WebComponent
customElements.define('mkp-taquin', MkpTaquinElt);


// Actions
const keyActions = {
    ArrowUp: (elt) => elt.move(Move.Up),
    ArrowRight: (elt) => elt.move(Move.Right),
    ArrowDown: (elt) => elt.move(Move.Down),
    ArrowLeft: (elt) => elt.move(Move.Left)
};

// Event Bindings
document.body.addEventListener('keyup', (event) => {
    const action = keyActions[event.key];
    if (action) {
        document.body.querySelectorAll('mkp-taquin')
            .forEach(elt => action(elt));
    }
});
