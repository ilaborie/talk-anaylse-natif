package io.monkeypatch.taquin.kt


interface Taquin {

    fun check(): Boolean
    fun solve(): List<Move>

    fun apply(move: Move) : Taquin

    fun isSolved(): Boolean

    fun displayString(): String

    fun checkAndSolve(): Result<List<Move>> =
        if (this.check()) Success(this.solve())
        else Failure(IllegalStateException("Unsolvable"))

}


sealed class Result<out T>




enum class Move {
    UP,
    RIGHT,
    DOWN,
    LEFT
}
data class Success<T>(val value: T) : Result<T>()
data class Failure(val cause: Exception) : Result<Nothing>()
