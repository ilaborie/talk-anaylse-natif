package io.monkeypatch.taquin.kt


typealias State = Taquin
typealias StateWithHistory = Pair<State, List<Move>>

interface Taquin {

    fun check(): Boolean

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
                    .map { move -> state.apply(move) to (history + move) }
                    .filter { (state, _) -> state !in visitedStates }
            }
        val newStates = next.map(StateWithHistory::first)
        monitor.foundNewStates(newStates.size)
        val nextVisited = visitedStates + newStates

        return solveAux(next, nextVisited, monitor)
    }


    fun apply(move: Move): Taquin
    fun availableMoves(): Set<Move>

    fun isSolved(): Boolean

    fun displayString(): String

    fun checkAndSolve(monitor: Monitor<State, Move> = Monitor.nop()): Result<List<Move>> =
        if (this.check()) Success(this.solve(monitor))
        else Failure(IllegalStateException("Unsolvable"))

}


