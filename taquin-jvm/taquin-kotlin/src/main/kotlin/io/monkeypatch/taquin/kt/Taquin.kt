package io.monkeypatch.taquin.kt


typealias State = Taquin
typealias StateWithHistory = Pair<State, List<Move>>

interface Taquin {

    fun check(): Boolean
    fun solve(): List<Move>

    fun apply(move: Move) : Taquin
    fun availableMoves(): Set<Move>

    fun isSolved(): Boolean

    fun displayString(): String

    fun checkAndSolve(): Result<List<Move>> =
        if (this.check()) Success(this.solve())
        else Failure(IllegalStateException("Unsolvable"))

}


