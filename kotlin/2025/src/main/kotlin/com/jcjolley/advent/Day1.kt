package com.jcjolley.advent

import java.util.logging.Logger

enum class Direction {
    Left,
    Right
}

class Dial(startPosition: Int) {
    companion object {
        val Log: Logger = Logger.getLogger(Dial::class.java.name)
    }
    var position = startPosition
    var zeroCount = 0

    fun turnDial(move: Move) {
        val startState = position
        position = (if (move.direction == Direction.Left) position - move.steps else position + move.steps) % 100
        if (position == 0) zeroCount++
        Log.info("Move: $move, start: $startState, end: $position, zeroCount: $zeroCount")
    }

    fun turnDialPartTwo(move: Move) {
        repeat(move.steps) {
            position = (if (move.direction == Direction.Left) position - 1 else position + 1)
            when {
                position == -1 -> position = 99
                position == 0 -> zeroCount++
                position == 100 -> {
                    zeroCount++
                    position = 0
                }
            }
        }
    }
}

data class Move(val direction: Direction, val steps: Int) {
    override fun toString(): String {
        val direction = if (direction == Direction.Left) "L" else "R"
        return "$direction$steps"
    }
}

fun partOne(input: String): Int {
    val dial = Dial(50)
    val moves = parseInput(input)
    moves.forEach { dial.turnDial(it) }
    return dial.zeroCount
}

fun partTwo(input: String): Int {
    val dial = Dial(50)
    val moves = parseInput(input)
    moves.forEach { dial.turnDialPartTwo(it) }
    return dial.zeroCount
}

fun parseInput(input: String): List<Move> = input
    .lines()
    .map {
        val direction = if (it.startsWith("L")) Direction.Left else Direction.Right
        val steps = it.drop(1).toInt()
        Move(direction, steps)
    }
