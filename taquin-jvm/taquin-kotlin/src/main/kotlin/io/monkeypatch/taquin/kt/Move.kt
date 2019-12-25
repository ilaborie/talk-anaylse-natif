package io.monkeypatch.taquin.kt


enum class Move {
    UP,
    RIGHT,
    DOWN,
    LEFT;

    val inverse: Move by lazy {
        when (this) {
            UP    -> DOWN
            RIGHT -> LEFT
            DOWN  -> UP
            LEFT  -> RIGHT
        }
    }
}
