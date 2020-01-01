package io.monkeypatch.taquin.kt

import org.slf4j.LoggerFactory

private val logger = LoggerFactory.getLogger("main")

fun main(args: Array<String>) {
    val (kind, size, pos) = args

    val t = if (kind.startsWith("a")) TaquinArray.fromString(size.toByte(), pos)
    else TaquinList.fromString(size.toByte(), pos)

    println(t.displayString())
    logger.info("Is solved: {}", t.isSolved())

    try {
        val result = t.solve(Monitor.logger())
        println("Solved in ${result.size}")
        var state = t
        result.forEach { move ->
            println("Apply $move:")
            state = state.next(move)
            println(state.displayString())
            println()
        }
    } catch (e: Throwable) {
        logger.error("Cannot solve this taquin!", e)
    }

}
