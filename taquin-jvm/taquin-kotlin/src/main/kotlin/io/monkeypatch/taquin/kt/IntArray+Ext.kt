package io.monkeypatch.taquin.kt


fun IntArray.swap(i: Int, j: Int): IntArray {
    val result = this.clone()
    result[i] = this[j]
    result[j] = this[i]
    return result
}
