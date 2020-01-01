package io.monkeypatch.taquin.kt

import io.monkeypatch.taquin.kt.Move.DOWN
import io.monkeypatch.taquin.kt.Move.LEFT
import io.monkeypatch.taquin.kt.Move.RIGHT
import io.monkeypatch.taquin.kt.Move.UP

data class Position(val x: Byte, val y: Byte) {

    fun toIndex(size: Byte): Byte =
        (x + (size * y)).toByte()

    fun move(direction: Move): Position =
        when (direction) {
            UP    -> copy(y = (y + 1).toByte())
            RIGHT -> copy(x = (x - 1).toByte())
            DOWN  -> copy(y = (y - 1).toByte())
            LEFT  -> copy(x = (x + 1).toByte())
        }

    companion object {

        fun fromIndex(index: Byte, size: Byte): Position =
            Position(
                x = (index % size).toByte(),
                y = (index / size).toByte()
            )
    }
}
