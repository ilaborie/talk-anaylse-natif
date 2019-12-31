package io.monkeypatch.taquin.kt

import org.slf4j.LoggerFactory
import java.text.DecimalFormat


interface Monitor<S, A> {
    fun nextDepth() {}
    fun found(actions: List<A>) {}
    fun foundNewStates(size: Int) {}
    fun visitedStates(size: Int) {}

    infix fun and(monitor: Monitor<S, A>): Monitor<S, A> {
        val that = this
        return object : Monitor<S, A> {
            override fun nextDepth() {
                that.nextDepth()
                monitor.nextDepth()
            }

            override fun found(actions: List<A>) {
                that.found(actions)
                monitor.found(actions)
            }

            override fun foundNewStates(size: Int) {
                that.foundNewStates(size)
                monitor.foundNewStates(size)
            }

            override fun visitedStates(size: Int) {
                that.visitedStates(size)
                monitor.visitedStates(size)
            }
        }
    }


    companion object {

        fun <S, A> nop(): Monitor<S, A> = object : Monitor<S, A> {}

        fun <S, A> maxDepth(max: Int): Monitor<S, A> = object : Monitor<S, A> {
            private var depth = 0

            override fun nextDepth() {
                depth += 1
                if (depth > max) throw IllegalStateException("Too deep ($depth)")
            }
        }

        fun <S, A> logger(): Monitor<S, A> = object : Monitor<S, A> {
            private var depth = 0
            private val logger = LoggerFactory.getLogger("MONITOR")
            private val formtter = DecimalFormat.getNumberInstance().apply {
                isGroupingUsed = true
            }

            override fun nextDepth() {
                depth += 1
                logger.info("Enter next depth {}", depth)
            }

            override fun found(actions: List<A>) {
                logger.info("Found a solution in {} moves:\n{}", actions.size, actions.joinToString(", "))
            }

            override fun foundNewStates(size: Int) {
                logger.info("Found {} new states", formtter.format(size))
            }

            override fun visitedStates(size: Int) {
                logger.info("Visited {} states", formtter.format(size))
            }
        }


    }
}
