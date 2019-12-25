package io.monkeypatch.taquin.kt

import io.monkeypatch.taquin.kt.Move.DOWN
import io.monkeypatch.taquin.kt.Move.LEFT
import io.monkeypatch.taquin.kt.Move.RIGHT
import io.monkeypatch.taquin.kt.Move.UP

data class Position(val x: Int, val y: Int) {

    fun toIndex(size: Int): Int =
        x + (size * y)

    fun move(direction: Move): Position =
        when (direction) {
            UP    -> copy(y = y + 1)
            RIGHT -> copy(x = x - 1)
            DOWN  -> copy(y = y - 1)
            LEFT  -> copy(x = x + 1)
        }


    companion object {

        fun fromIndex(index: Int, size: Int): Position =
            Position(
                x = index % size,
                y = index / size
            )
    }
}
