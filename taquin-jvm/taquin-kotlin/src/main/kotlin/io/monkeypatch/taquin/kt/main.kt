package io.monkeypatch.taquin.kt

fun main(args: Array<String>) {
    val (size, pos)  = args

    val t = TaquinImpl.fromString(size.toInt(), pos)

    println(t.displayString())

    println("Is solved:, ${t.isSolved()}")
}
