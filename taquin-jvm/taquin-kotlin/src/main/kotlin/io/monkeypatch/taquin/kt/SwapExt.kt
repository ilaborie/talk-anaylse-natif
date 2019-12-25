package io.monkeypatch.taquin.kt


fun IntArray.swap(i: Int, j: Int): IntArray {
    val result = this.clone()
    result[i] = this[j]
    result[j] = this[i]
    return result
}

fun <T> MutableList<T>.swap(i: Int, j: Int): MutableList<T> {
    val tmp = this[i]
    this[i] = this[j]
    this[j] = tmp
    return this
}
