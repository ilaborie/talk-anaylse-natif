package io.monkeypatch.taquin.kt

data class TaquinArray(
    override val size: Int,
    private val values: IntArray
) : Taquin {

    override fun get(position: Position): Int =
        values[position.toIndex(size)]

    internal val holeIndex: Int =
        values.indexOf(0)

    override val holePosition: Position =
        Position.fromIndex(holeIndex, size)

    override fun next(move: Move): Taquin {
        require(move in availableMoves()) {
            "Illegal move $move, only ${availableMoves().joinToString(", ")} allowed"
        }

        val newPosition = holePosition.move(move)
        val newHole = newPosition.toIndex(size)

        return copy(values = values.swap(holeIndex, newHole))
    }

    override fun toString(): String =
        values.toList()
            .chunked(size)
            .joinToString(",  ") { it.joinToString(",") }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false
        other as TaquinArray
        if (!values.contentEquals(other.values)) return false
        return true
    }

    override fun hashCode(): Int =
        values.contentHashCode()

    companion object {

        fun solved(size: Int): Taquin =
            (size * size).let { sq ->
                TaquinArray(
                    size,
                    values = IntArray(sq) { i -> (i + 1) % sq }
                )
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
                    "${Position.fromIndex(index, size)} should have one entry, got $count"
                )
            }

            return TaquinArray(size, array)
        }
    }

}
