package io.monkeypatch.taquin.kt

import io.monkeypatch.taquin.kt.Move.DOWN
import io.monkeypatch.taquin.kt.Move.LEFT
import io.monkeypatch.taquin.kt.Move.RIGHT
import io.monkeypatch.taquin.kt.Move.UP


typealias State = Taquin
typealias StateWithHistory = Pair<State, List<Move>>

interface Taquin {

    val size: Byte // require size <=16
    val maxIndex: Byte
        get() = (size * size - 1).toByte()
    val holePosition: Position

    operator fun get(position: Position): Byte

    fun countInversion(): Int {
        return (0..maxIndex)
            .flatMap { i -> ((i + 1)..maxIndex).map { j -> i to j } }
            .map { (i, j) -> this[Position.fromIndex(i.toByte(), size)] to this[Position.fromIndex(j.toByte(), size)] }
            .filterNot { (i, j) -> i == HOLE || j == HOLE }
            .count { (i, j) -> i > j }
    }

    fun check(): Boolean =
        size <= 16 && countInversion() % 2 == 0

    fun solve(monitor: Monitor<State, Move>): List<Move> {
        require(check()) { "Not Solvable!" }
        return solveAux(listOf(this to emptyList()), setOf(this), monitor)
    }

    private tailrec fun solveAux(
        statesWithHistory: List<StateWithHistory>,
        visitedStates: Set<State>,
        monitor: Monitor<State, Move>
    ): List<Move> {
        val result = statesWithHistory.find { (state, _) -> state.isSolved() }
        monitor.visitedStates(visitedStates.size)
        if (result != null) {
            val moves = result.second
            monitor.found(moves)
            return moves
        }
        monitor.nextDepth()
        val next = statesWithHistory
            .flatMap { (state, history) ->
                val availableMoves =
                    if (history.isEmpty()) state.availableMoves()
                    else state.availableMoves() - history.last()

                availableMoves
                    .map { move -> state.next(move) to (history + move) }
                    .filter { (state, _) -> state !in visitedStates }
            }
        val newStates = next.map(StateWithHistory::first)
        monitor.foundNewStates(newStates.size)
        val nextVisited = visitedStates + newStates

        return solveAux(next, nextVisited, monitor)
    }


    fun next(move: Move): Taquin

    fun availableMoves(): Set<Move> {
        val (x, y) = holePosition

        val result = mutableSetOf<Move>()
        if (x != 0.toByte()) result.add(RIGHT)
        if (x != (size - 1).toByte()) result.add(LEFT)
        if (y != 0.toByte()) result.add(DOWN)
        if (y != (size - 1).toByte()) result.add(UP)
        return result
    }

    fun isSolved(): Boolean =
        (0 until size)
            .flatMap { x ->
                (0 until size).map { y -> Position(x.toByte(), y.toByte()) }
            }
            .map { pos ->
                val value = this[pos]
                val index = pos.toIndex(size)
                val goodNormalValue = value == (index + 1).toByte()
                val goodHolePosition = index == maxIndex && value == HOLE
                goodNormalValue || goodHolePosition
            }.all { it }

    fun displayString(): String =
        maxIndex.toString().length.let { len ->
            (0 until size).joinToString("\n") { y ->
                (0 until size)
                    .map { x -> Position(x.toByte(), y.toByte()) }
                    .joinToString(" ") {
                        val value = this[it]
                        val s = if (value == HOLE) "·" else value.toString()
                        s.padStart(len)
                    }
            }
        }

    fun shuffle(): Taquin {
        val count = size * size * size * size
        var previousMove: Move = DOWN
        val moves: MutableList<Move> = mutableListOf()
        return (1..count).fold(this) { t, _ ->
            val move = (t.availableMoves() - previousMove.inverse).random()
            previousMove = move
            moves += move
            t.next(move)
        }
    }

    companion object {
        const val HOLE: Byte = 0
    }
}


