package io.monkeypatch.taquin.kt

sealed class Result<out T>

data class Success<T>(val value: T) : Result<T>()
data class Failure(val cause: Exception) : Result<Nothing>()
