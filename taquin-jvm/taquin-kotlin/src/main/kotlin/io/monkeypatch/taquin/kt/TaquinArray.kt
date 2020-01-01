package io.monkeypatch.taquin.kt

data class TaquinArray(
    override val size: Byte,
    private val values: ByteArray
) : Taquin {

    override fun get(position: Position): Byte =
        values[position.toIndex(size).toInt()]

    internal val holeIndex: Byte =
        values.indexOf(0).toByte()

    override val holePosition: Position =
        Position.fromIndex(holeIndex, size)

    override fun next(move: Move): Taquin {
        require(move in availableMoves()) {
            "Illegal move $move, only ${availableMoves().joinToString(", ")} allowed"
        }

        val newPosition = holePosition.move(move)
        val newHole = newPosition.toIndex(size)

        return copy(values = values.swap(holeIndex.toInt(), newHole.toInt()))
    }

    override fun toString(): String =
        values.toList()
            .chunked(size.toInt())
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
                    size.toByte(),
                    values = ByteArray(sq) { i -> ((i + 1) % sq).toByte() }
                )
            }

        fun fromString(size: Byte, s: String): Taquin {
            val arrayLen = size * size

            val (array, counts) = s.split(",")
                .map { it.trim() }
                .map { it.toInt() }
                .foldIndexed(ByteArray(arrayLen) to IntArray(arrayLen)) { index, (array, counts), i ->
                    // check size (crash if not OK)
                    array[index] = i.toByte()
                    counts[i] += 1
                    array to counts
                }

            // Check has hole
            counts.forEachIndexed { index, count ->
                if (count != 1) throw IllegalStateException(
                    "${Position.fromIndex(index.toByte(), size)} should have one entry, got $count"
                )
            }

            return TaquinArray(size, array)
        }
    }

}
