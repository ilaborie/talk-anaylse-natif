package io.monkeypatch.taquin.kt

import io.monkeypatch.taquin.kt.Move.DOWN

fun main() {
//    val t = TaquinArray.fromString(4, "1,2,3,4,  5,6,7,8,  9,10,11,12,  13,14,15,0")
    val t = TaquinArray.solved(3)

    val nth = 1000
    var previousMove: Move = DOWN
    val moves: MutableList<Move> = mutableListOf()
    val result = (1..nth).fold(t) { t, _ ->
        val move = (t.availableMoves() - previousMove.inverse).random()
        previousMove = move
        moves += move
        t.next(move)
//        val pos = t.next(move)
//        val inversions = pos.countInversion()
//        println("Inversion: $inversions")
//        if(inversions %2 ==1) throw IllegalStateException("Oops!")
//        pos
    }

    println("After $nth moves:")
    println(result)

    val solution = moves.reversed()
        .map { it.inverse }
        .joinToString(",")
    println("A solution:")
    println(solution)

}
