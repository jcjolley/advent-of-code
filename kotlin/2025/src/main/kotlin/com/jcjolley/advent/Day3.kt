package com.jcjolley.advent

import java.util.logging.Logger

class Day3 {
    companion object {
        val Log: Logger = Logger.getLogger(Dial::class.java.name)
    }

    fun partOne(input: String): Long {
        return input
            .parseInput()
            .sumOf { getMaxNdigitNumber(it, 2) }
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
            .sumOf { getMaxNdigitNumber(it, 12) }

    }

    fun getMaxNdigitNumber(line: List<Int>, numDigits: Int): Long {
        val digits = mutableListOf<Int>()
        var wall = -1
        (numDigits downTo 1).forEach {
            val searchSpace = line.subList(wall + 1, line.size).dropLast(it - 1)
            val digit = searchSpace.max()
            wall += searchSpace.indexOf(digit) + 1
            digits.add(digit)
        }
        return digits.joinToString("").toLong()
    }

    fun String.parseInput(): List<List<Int>> = this
        .lines()
        .map { line -> line.chunked(1).map { it.toInt() } }
}

