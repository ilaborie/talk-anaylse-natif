package io.monkeypatch.taquin.kt


class TaquinImpl private constructor(private val size: Int, private val values: IntArray) : Taquin {

    private val maxIndex = size * size - 1

    override fun isSolved(): Boolean =
        values.mapIndexed { index, i -> (i == (index + 1)) || (index == maxIndex && i == 0) }
            .all { it }

    internal val countInversion: Int by lazy {
        (0 until maxIndex)
            .flatMap { i ->
                ((i + 1) until maxIndex)
                    .map { j -> values[i] to values[j] }
            }
            .filterNot { (i, j) -> i == 0 || j == 0 }
            .count { (i, j) -> i > j }
    }

    override fun check(): Boolean =
        countInversion % 2 == 0

    override fun solve(): List<Move> {
        TODO("not implemented")
    }

    override fun apply(move: Move): Taquin {
        val hole = values.indexOf(0)
        val (x, y) = indexToPosition(hole, size)

        val availableMoves = availableMoves(x, y)
        if (move !in availableMoves) {
            throw IllegalArgumentException("Illegal move $move, only ${availableMoves.joinToString(", ")} allowed")
        }

        val newPosition = when (move) {
            Move.UP    -> x to (y - 1)
            Move.RIGHT -> (x - 1) to y
            Move.DOWN  -> x to (y + 1)
            Move.LEFT  -> (x + 1) to y
        }
        val newHole = positionToIndex(newPosition, size)

        return TaquinImpl(size, values.swap(hole, newHole))
    }

    private fun availableMoves(x: Int, y: Int): List<Move> {
        val result = mutableListOf<Move>()
        if (x != 0) result.add(Move.LEFT)
        if (x != (size - 1)) result.add(Move.RIGHT)
        if (y != 0) result.add(Move.UP)
        if (y != (size - 1)) result.add(Move.DOWN)
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

    companion object {

        private fun IntArray.swap(i: Int, j: Int): IntArray {
            val result = this.clone()
            result[i] = this[j]
            result[j] = this[i]
            return result
        }

        fun indexToPosition(index: Int, size: Int): Pair<Int, Int> {
            val x = index % size
            val y = index / size
            return x to y
        }

        fun positionToIndex(position: Pair<Int, Int>, size: Int): Int {
            val (x, y) = position
            return x + (size * y)
        }

        fun fromString(size: Int, s: String): Taquin {
            val arrayLen = size * size

            val (array, counts) = s.split(",")
                .map { it.trim() }
                .map { it.toInt() }
                .foldIndexed(IntArray(arrayLen) to IntArray(arrayLen)) { index, (array, counts), i ->
                    // check size (crash if not OK)
                    array[index] = i
                    counts[i] += 1
                    array to counts
                }

            // Check has hole
            counts.forEachIndexed { index, count ->
                if (count != 1) throw IllegalStateException(
                    "Position ${indexToPosition(
                        index,
                        size
                    )} should have one entry, got $count"
                )
            }

            return TaquinImpl(size, array)
        }
    }

}
