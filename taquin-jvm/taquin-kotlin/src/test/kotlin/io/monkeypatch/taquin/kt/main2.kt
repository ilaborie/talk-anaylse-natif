package io.monkeypatch.taquin.kt

import io.monkeypatch.taquin.kt.Move.DOWN
import org.slf4j.LoggerFactory
import java.io.File
import java.nio.file.Paths

private val logger = LoggerFactory.getLogger("GENERATOR")

fun main(args: Array<String>) {
//    generateOneWithSolution(3, 1000)
//    generateOneWithSolution(4, 32)

    val count = if (args.isEmpty()) 100 else args[0].toInt()
    listOf(3, 4).forEach {
        generateToFile(it, count, Paths.get("inputs-${it}x$it.txt").toFile())
    }
}

private fun generateToFile(size: Int, count: Int, outputFile: File) {
    logger.info("Generate $count problem into $outputFile...")
    if (outputFile.createNewFile()) {
        logger.debug("Create $outputFile")
    } else {
        logger.debug("Clear $outputFile")
        outputFile.writeText("")
    }

    (1..count)
        .map { TaquinArray.solved(size) }
        .map { it.shuffle() }
        .map { "$it\n" }
        .forEach { outputFile.appendText(it) }

    logger.info("[OK] Generate $count problem into $outputFile")
}


private fun generateOneWithSolution(size: Int, count: Int) {
    val t = TaquinArray.solved(size)

    var previousMove: Move = DOWN
    val moves: MutableList<Move> = mutableListOf()
    val result = (1..count).fold(t) { t, _ ->
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
    println("After $count moves:")
    println(result)

    val solution = moves.reversed()
        .map { it.inverse }
        .joinToString(",")
    println("A solution:")
    println(solution)
}
