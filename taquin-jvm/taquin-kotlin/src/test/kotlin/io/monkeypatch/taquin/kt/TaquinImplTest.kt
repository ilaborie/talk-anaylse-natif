package io.monkeypatch.taquin.kt

import io.kotlintest.shouldBe
import io.kotlintest.specs.DescribeSpec


class TaquinImplTest : DescribeSpec() {

    init {

        describe("Simple solved 3x3 Taquin") {
            val t = TaquinImpl.fromString(3, "1,2,3,  4,5,6,  7,8,0")

            it("should be solved") {
                t.isSolved() shouldBe true
            }

            it("should be displayable") {
                t.displayString() shouldBe """1 2 3
                                             |4 5 6
                                             |7 8 ·""".trimMargin("|")
            }

            it("should have no inversion") {
                (t as TaquinImpl).countInversion shouldBe 0
            }
            it("should be ok to solve") {
                t.check() shouldBe true
            }
        }

        describe("Simple unsolvable 3x3 Taquin") {
            val t = TaquinImpl.fromString(3, "1,2,3,  4,5,6,  8,7,0")

            it("should be solved") {
                t.isSolved() shouldBe false
            }

            it("should be displayable") {
                t.displayString() shouldBe """1 2 3
                                             |4 5 6
                                             |8 7 ·""".trimMargin("|")
            }

            it("should have one inversion") {
                (t as TaquinImpl).countInversion shouldBe 1
            }
            it("should not be solvable") {
                t.check() shouldBe false
            }
        }


        describe("Simple solvable 3x3 Taquin") {
            val t = TaquinImpl.fromString(3, "1,3,2,  4,5,6,  8,7,0")

            it("should be solved") {
                t.isSolved() shouldBe false
            }

            it("should be displayable") {
                t.displayString() shouldBe """1 3 2
                                             |4 5 6
                                             |8 7 ·""".trimMargin("|")
            }

            it("should have some inversion") {
                (t as TaquinImpl).countInversion shouldBe 2
            }
            it("should be solvable") {
                t.check() shouldBe true
            }
        }

    }
}
