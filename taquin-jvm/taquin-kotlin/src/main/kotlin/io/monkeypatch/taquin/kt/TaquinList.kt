package io.monkeypatch.taquin.kt

data class TaquinList(
    override val size: Byte,
    private val values: List<Byte>
) : Taquin {

    private val holeIndex: Byte =
        values.indexOf(0).toByte()

    override val holePosition: Position =
        Position.fromIndex(holeIndex, size)

    override fun get(position: Position): Byte =
        values[position.toIndex(size).toInt()]

    override fun next(move: Move): Taquin {
        require(move in availableMoves()) {
            "Illegal move $move, only ${availableMoves().joinToString(", ")} allowed"
        }

        val newPosition = holePosition.move(move)
        val newHole = newPosition.toIndex(size)

        return TaquinList(size, values.swap(holeIndex.toInt(), newHole.toInt()))
    }


    override fun toString(): String =
        values.toList()
            .chunked(size.toInt())
            .joinToString(",  ") { it.joinToString(",") }

    companion object {

        fun solved(size: Byte): Taquin =
            (size * size).let { sq ->
                TaquinList(
                    size,
                    values = List(sq) { i -> ((i + 1) % sq).toByte() }
                )
            }

        fun fromString(size: Byte, s: String): Taquin {
            val arrayLen = size * size

            val (lst, counts) = s.split(",")
                .map { it.trim() }
                .map { it.toInt() }
                .fold(emptyList<Byte>() to IntArray(arrayLen)) { (lst, counts), i ->
                    // check size (crash if not OK)
                    counts[i] += 1
                    (lst + i.toByte()) to counts
                }

            // Check has hole
            counts.forEachIndexed { index, count ->
                if (count != 1) throw IllegalStateException(
                    "${Position.fromIndex(index.toByte(), size)} should have one entry, got $count"
                )
            }

            return TaquinList(size, lst)
        }
    }

}
