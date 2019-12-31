package io.monkeypatch.taquin.kt

data class TaquinList(
    override val size: Int,
    private val values: List<Int>
) : Taquin {

    internal val holeIndex: Int =
        values.indexOf(0)

    override val holePosition: Position =
        Position.fromIndex(holeIndex, size)

    override fun get(position: Position): Int =
        values[position.toIndex(size)]

    override fun next(move: Move): Taquin {
        require(move in availableMoves()) {
            "Illegal move $move, only ${availableMoves().joinToString(", ")} allowed"
        }

        val newPosition = holePosition.move(move)
        val newHole = newPosition.toIndex(size)

        return TaquinList(size, values.toMutableList().swap(holeIndex, newHole).toList())
    }


    override fun toString(): String =
        values.toList()
            .chunked(size)
            .joinToString(",  ") { it.joinToString(",") }

    companion object {

        fun solved(size: Int): Taquin =
            (size * size).let { sq ->
                TaquinList(
                    size,
                    values = List(sq) { i -> (i + 1) % sq }
                )
            }

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
