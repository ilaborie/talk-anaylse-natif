package io.monkeypatch.taquin.kt

import io.kotlintest.matchers.collections.atMostSize
import io.kotlintest.should
import io.kotlintest.shouldBe
import io.kotlintest.specs.DescribeSpec
import io.monkeypatch.taquin.kt.Move.*


class SolveTest : DescribeSpec() {
    init {

        describe("Solve 3x3 - 1") {
            val t = TaquinArray.fromString(3, "1,2,3,  7,5,0,  8,4,6") as TaquinArray
            val maxDepth = 15

            it("Should be solved in less than $maxDepth moves") {
                val monitor = Monitor.logger<State, Move>() and Monitor.maxDepth(20)
                val result = t.solve(monitor)

                result should atMostSize(maxDepth)
            }
        }

        describe("Solve 3x3 - 2") {
            val t = TaquinArray.fromString(3, "5,3,1,  7,2,8,  0,6,4") as TaquinArray
            val maxDepth = 30

            it("Should be solved in less than $maxDepth moves") {
                val monitor = Monitor.logger<State, Move>() and Monitor.maxDepth(50)
                val result = t.solve(monitor)

                result should atMostSize(maxDepth)
            }
        }

        describe("Solve 3x3 - 3") {
            val t = TaquinArray.fromString(3, "4,1,3,  5,0,2,  8,7,6")
            val sol= listOf(UP,RIGHT,DOWN,DOWN,LEFT,LEFT,UP,RIGHT,UP,LEFT,DOWN,RIGHT,UP,LEFT,DOWN,RIGHT,DOWN,RIGHT,UP,UP,LEFT,LEFT,DOWN,RIGHT,RIGHT,DOWN,LEFT,LEFT,UP,UP)
            val maxDepth = sol.size

            it("should follow a solution") {
                val r: Taquin = sol.fold(t) { acc, move -> acc.next(move) }

                r.isSolved() shouldBe true
            }

            it("Should be solved in less than $maxDepth moves") {
                val monitor = Monitor.logger<State, Move>() and Monitor.maxDepth(maxDepth)
                val result = t.solve(monitor)

                result should atMostSize(maxDepth)
            }
        }

        describe("Solve 3x3 - 4") {
            val t = TaquinArray.fromString(3, "6,4,0,  2,7,5,  8,1,3")
            val sol= listOf(UP,UP,RIGHT,RIGHT,DOWN,DOWN,LEFT,UP,RIGHT,UP,LEFT,LEFT,DOWN,RIGHT,UP,LEFT,DOWN,RIGHT,DOWN,LEFT,UP,RIGHT,RIGHT,UP,LEFT,LEFT,DOWN,RIGHT,UP,LEFT,DOWN,DOWN,RIGHT,RIGHT,UP,UP,LEFT,DOWN,LEFT,DOWN,RIGHT,RIGHT,UP,LEFT,DOWN,LEFT,UP,RIGHT,UP,LEFT,DOWN,DOWN,RIGHT,RIGHT,UP,LEFT,LEFT,DOWN,RIGHT,UP,RIGHT,DOWN,LEFT,LEFT,UP,UP,RIGHT,RIGHT,DOWN,DOWN,LEFT,UP,LEFT,UP,RIGHT,DOWN,LEFT,DOWN,RIGHT,RIGHT,UP,LEFT,LEFT,UP,RIGHT,DOWN,LEFT,DOWN,RIGHT,RIGHT,UP,UP,LEFT,DOWN,DOWN,RIGHT,UP,UP,LEFT,LEFT)
            val maxDepth = sol.size

            it("should follow a solution") {
                val r: Taquin = sol.fold(t) { acc, move -> acc.next(move) }

                r.isSolved() shouldBe true
            }

            it("Should be solved in less than $maxDepth moves") {
                val monitor = Monitor.logger<State, Move>() and Monitor.maxDepth(maxDepth)
                val result = t.solve(monitor)

                result should atMostSize(maxDepth)
            }
        }

//        describe("Solve 4x4") {
//            val t = TaquinList.fromString(4, "2,3,11,4,  1,7,6,0,  9,5,12,8,  10,13,14,15")
//            val maxDepth = 31
//
//            it("Should be solved in less than $maxDepth moves") {
//                val monitor = Monitor.logger<State, Move>() and Monitor.maxDepth(maxDepth)
//                val result = t.solve(monitor)
//
//                result should atMostSize(maxDepth)
//            }
//        }


    }
}
