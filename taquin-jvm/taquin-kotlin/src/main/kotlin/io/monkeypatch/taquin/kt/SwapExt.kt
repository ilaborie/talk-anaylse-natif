package io.monkeypatch.taquin.kt


fun ByteArray.swap(i: Int, j: Int): ByteArray {
    val result = this.clone()
    result[i] = this[j]
    result[j] = this[i]
    return result
}

fun <T> List<T>.swap(i: Int, j: Int): List<T> {
    val lst = this.toMutableList()
    val tmp = lst[i]
    lst[i] = lst[j]
    lst[j] = tmp
    return lst.toList() // FIXME check defensive copy
}
