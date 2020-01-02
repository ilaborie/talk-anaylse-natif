package io.monkeypatch.taquin.kt

import io.kotlintest.shouldBe
import io.kotlintest.specs.DescribeSpec
import io.monkeypatch.taquin.kt.Move.DOWN
import io.monkeypatch.taquin.kt.Move.LEFT
import io.monkeypatch.taquin.kt.Move.RIGHT
import io.monkeypatch.taquin.kt.Move.UP


class TaquinArrayTest : DescribeSpec() {

    init {

        describe("Simple solved 3x3 Taquin") {
            val t = TaquinArray.fromString(3, "1,2,3,  4,5,6,  7,8,0") as TaquinArray
            val t2 = TaquinArray.fromString(3, "1,2,3,  4,5,6,  7,8,0") as TaquinArray

            it("should be equals") {
                t shouldBe t2
            }

            it("should have equals hashcode") {
                t.hashCode() shouldBe t2.hashCode()
            }

            it("should be solved") {
                t.isSolved() shouldBe true
            }

            it("should be displayable") {
                t.displayString() shouldBe """1 2 3
                                             |4 5 6
                                             |7 8 ·""".trimMargin("|")
            }

            it("should have hole index at 8") {
                t.holeIndex shouldBe 8.toByte()
            }

            it("should have hole position at (2,2)") {
                t.holePosition shouldBe Position(2, 2)
            }

            it("can move DOWN and RIGHT") {
                t.availableMoves() shouldBe setOf(DOWN, RIGHT)
            }

            it("move RIGHT") {
                val result = t.next(RIGHT)
                result.displayString() shouldBe """1 2 3
                                                  |4 5 6
                                                  |7 · 8""".trimMargin("|")
            }

            it("move DOWN") {
                val result = t.next(DOWN)
                result.displayString() shouldBe """1 2 3
                                                   |4 5 ·
                                                   |7 8 6""".trimMargin("|")
            }

            it("should have no inversion") {
                t.countInversion() shouldBe 0
            }

            it("should be ok to solve") {
                t.check() shouldBe true
            }
        }

        describe("Simple unsolvable 3x3 Taquin") {
            val t = TaquinArray.fromString(3, "1,2,3,  4,5,6,  8,7,0")

            it("should be solved") {
                t.isSolved() shouldBe false
            }

            it("should be displayable") {
                t.displayString() shouldBe """1 2 3
                                             |4 5 6
                                             |8 7 ·""".trimMargin("|")
            }

            it("should have one inversion") {
                (t as TaquinArray).countInversion() shouldBe 1
            }
            it("should not be solvable") {
                t.check() shouldBe false
            }
        }


        describe("Simple solvable 3x3 Taquin") {
            val t = TaquinArray.fromString(3, "1,2,3,  4,5,0,  7,8,6") as TaquinArray

            it("should be solved") {
                t.isSolved() shouldBe false
            }

            it("should be displayable") {
                t.displayString() shouldBe """1 2 3
                                             |4 5 ·
                                             |7 8 6""".trimMargin("|")
            }

            it("should have some inversion") {
                t.countInversion() shouldBe 2
            }

            it("should be solvable") {
                t.check() shouldBe true
            }

            it("should close to result") {
                val result = t.next(UP)
                result.displayString() shouldBe """1 2 3
                                                  |4 5 6
                                                  |7 8 ·""".trimMargin("|")
                result.isSolved() shouldBe true
            }

            it("should be solved in one move") {
                val moves = t.solve(Monitor.logger())
                moves shouldBe listOf(UP)
            }
        }


        describe("Simple2 solvable 3x3 Taquin") {
            val t = TaquinArray.fromString(3, "1,2,3,  4,5,6,  7,0,8") as TaquinArray

            it("should be solved") {
                t.isSolved() shouldBe false
            }

            it("should have some inversion") {
                t.countInversion() shouldBe 0
            }

            it("should be solvable") {
                t.check() shouldBe true
            }

            it("should close to result") {
                val result = t.next(LEFT)
                result.displayString() shouldBe """1 2 3
                                                  |4 5 6
                                                  |7 8 ·""".trimMargin("|")
                result.isSolved() shouldBe true
            }

            it("should be solved in one move") {
                val moves = t.solve(Monitor.logger())
                moves shouldBe listOf(LEFT)
            }
        }

    }
}
