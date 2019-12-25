package io.monkeypatch.taquin.kt

import io.monkeypatch.taquin.kt.Move.DOWN
import io.monkeypatch.taquin.kt.Move.LEFT
import io.monkeypatch.taquin.kt.Move.RIGHT
import io.monkeypatch.taquin.kt.Move.UP

data class TaquinList private constructor(private val size: Int, private val values: List<Int>) : Taquin {

    private val maxIndex = size * size - 1

    override fun isSolved(): Boolean =
        values.mapIndexed { index, i ->
            val goodNormalValue = i == (index + 1)
            val goodHolePosition = index == maxIndex && i == 0
            goodNormalValue || goodHolePosition
        }.all { it }

    internal val countInversion: Int by lazy {
        (0..maxIndex)
            .flatMap { i -> ((i + 1)..maxIndex).map { j -> i to j } }
            .map { (i, j) -> values[i] to values[j] }
            .filterNot { (i, j) -> i == 0 || j == 0 }
            .count { (i, j) -> i > j }
    }

    internal val holeIndex: Int =
        values.indexOf(0)

    internal val holePosition: Position =
        Position.fromIndex(holeIndex, size)

    override fun check(): Boolean =
        countInversion % 2 == 0


    override fun apply(move: Move): Taquin {
        require(move in availableMoves()) {
            "Illegal move $move, only ${availableMoves().joinToString(", ")} allowed"
        }

        val newPosition = holePosition.move(move)
        val newHole = newPosition.toIndex(size)

        return TaquinList(size, values.toMutableList().swap(holeIndex, newHole).toList())
    }

    override fun availableMoves(): Set<Move> {
        val (x, y) = holePosition

        val result = mutableSetOf<Move>()
        if (x != 0) result.add(RIGHT)
        if (x != (size - 1)) result.add(LEFT)
        if (y != 0) result.add(DOWN)
        if (y != (size - 1)) result.add(UP)
        return result
    }

    override fun displayString(): String {
        val len = maxIndex.toString().length
        return values.mapIndexed { index, i ->
            val prefix = (if (i == 0) "·" else i.toString()).padStart(len)
            val suffix = when {
                index == maxIndex       -> ""
                (index + 1) % size == 0 -> "\n"
                else                    -> " "
            }
            prefix + suffix
        }.joinToString("")
    }

    override fun toString(): String =
        displayString()

    companion object {


        fun fromString(size: Int, s: String): Taquin {
            val arrayLen = size * size

            val (lst, counts) = s.split(",")
                .map { it.trim() }
                .map { it.toInt() }
                .fold(emptyList<Int>() to IntArray(arrayLen)) { (lst, counts), i ->
                    // check size (crash if not OK)
                    counts[i] += 1
                    (lst + i) to counts
                }

            // Check has hole
            counts.forEachIndexed { index, count ->
                if (count != 1) throw IllegalStateException(
                    "${Position.fromIndex(index, size)} should have one entry, got $count"
                )
            }

            return TaquinList(size, lst)
        }
    }

}
