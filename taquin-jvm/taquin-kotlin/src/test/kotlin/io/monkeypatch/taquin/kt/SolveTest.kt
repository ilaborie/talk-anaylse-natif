package io.monkeypatch.taquin.kt

import io.kotlintest.matchers.collections.atMostSize
import io.kotlintest.should
import io.kotlintest.specs.DescribeSpec


class SolveTest : DescribeSpec() {
    init {

//        describe("Solve 3x3 - 1") {
//            val t = TaquinArray.fromString(3, "1,2,3,  7,5,0,  8,4,6") as TaquinArray
//            val maxDepth = 15
//
//            it("Should be solved in less than $maxDepth moves") {
//                val monitor = Monitor.logger<State, Move>() and Monitor.maxDepth(20)
//                val result = t.solve(monitor)
//
//                result should atMostSize(maxDepth)
//            }
//        }
//
//        describe("Solve 3x3 - 2") {
//            val t = TaquinArray.fromString(3, "5,3,1,  7,2,8,  0,6,4") as TaquinArray
//            val maxDepth = 22
//
//            it("Should be solved in less than $maxDepth moves") {
//                val monitor = Monitor.logger<State, Move>() and Monitor.maxDepth(50)
//                val result = t.solve(monitor)
//
//                result should atMostSize(maxDepth)
//            }
//        }

        describe("Solve 4x4") {
            val t = TaquinList.fromString(4, "2,3,11,4,  1,7,6,0,  9,5,12,8,  10,13,14,15")
            val maxDepth = 31

            it("Should be solved in less than $maxDepth moves") {
                val monitor = Monitor.logger<State, Move>() and Monitor.maxDepth(maxDepth)
                val result = t.solve(monitor)

                result should atMostSize(maxDepth)
            }
        }


    }
}
