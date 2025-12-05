package com.jcjolley.advent

import java.util.logging.Logger

class Day3 {
    companion object {
        val Log: Logger = Logger.getLogger(Dial::class.java.name)
    }

    fun partOne(input: String): Long {
        return input
            .parseInput()
            .sumOf { getMaxNDigitNumber(it, 2) }
    }

    fun getMax2DigitNumber(line: List<Int>): Int {
        val tens = line.dropLast(1).max()
        val tensIndex = line.indexOf(tens)
        val ones = line.subList(tensIndex + 1, line.size).max()
        return tens * 10 + ones
    }

    fun partTwo(input: String): Long {
        return input
            .parseInput()
            .sumOf { getMaxNDigitNumber(it, 12) }

    }

    fun getMaxNDigitNumber(line: List<Int>, numDigits: Int): Long {
        var wall = -1
        return (numDigits downTo 1).map { n ->
            val searchSpace = (wall + 1)..<(line.size - n + 1)
            line.maxOfRange(searchSpace)
                .also { wall = it.index }
                .value
        }
            .joinToString("")
            .toLong()
    }

    data class MaxIndexed(
        val index: Int,
        val value: Int
    )

    fun List<Int>.maxOfRange(indices: IntRange): MaxIndexed {
        return indices
            .map { it to this[it] }
            .maxBy { it.second }
            .let { (index, value) -> MaxIndexed(index, value)}
    }

    fun String.parseInput(): List<List<Int>> = this
        .lines()
        .map { line -> line.chunked(1).map { it.toInt() } }
}

