package io.monkeypatch.taquin.kt

import io.monkeypatch.taquin.kt.Move.DOWN

fun main() {
    val t = TaquinArray.fromString(4, "1,2,3,4,  5,6,7,8,  9,10,11,12,  13,14,15,0")

    var previousMove: Move = DOWN
    val nth = 30
    val result = (1..nth).fold(t) { t, _ ->
        val moves = t.availableMoves() - previousMove.inverse
        val move = moves.random()
        previousMove = move
        println(move)
        t.apply(move)
    }

    println("After $nth moves:")
    println(result.displayString())

}
