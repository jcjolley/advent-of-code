package com.jcjolley.advent

import java.util.logging.Logger

class Day2 {
    companion object {
        val Log: Logger = Logger.getLogger(Dial::class.java.name)
    }

    fun partOne(input: String): Long {
        val ranges = parseInput(input)
        return ranges.sumOf { range ->
            range.sumOf { n ->
                val str = n.toString()
                val len = str.length
                if (len % 2 == 1) {
                    0
                } else {
                    val first = str.take(len / 2)
                    val second = str.substring(len / 2)
                    if (first == second) n else 0
                }
            }
        }
    }

    fun partTwo(input: String): Long {
        val ranges = parseInput(input)
        return ranges.sumOf { range ->
            val results = range.map { n ->
                val str = n.toString()
                val len = str.length
                val half = len / 2
                (1..half).firstOrNull { size ->
                    val first = str.take(size)
                    (str.chunked(size).all { it == first })
                }?.let { n } ?: 0
            }
            // Log.info("$range -> $results")
            results.sum()
        }
    }

    fun parseInput(input: String): List<LongRange> = input
        .split(",")
        .map { entry ->
            val (first, second) = entry.split("-").map { it.toLong() }
            first..second
        }
}
