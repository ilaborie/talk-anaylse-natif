package io.monkeypatch.taquin.kt

fun main(args: Array<String>) {
    val (size, pos) = args

    val t = TaquinArray.fromString(size.toInt(), pos)
    println(t.displayString())
    println("Is solved:, ${t.isSolved()}")

    when (val result = t.checkAndSolve()) {
        is Success -> {
            println("Solved in ${result.value.size}")
            var state = t
            result.value.forEach { move ->
                println("Apply $move:")
                state = state.apply(move)
                println(state.displayString())
                println()
            }
        }
        is Failure ->
            println("Cannot solve this taquin!")
    }
}
