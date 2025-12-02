package day01

import "testing"

const TEST_INPUT = `L68
L30
R48
L5
R60
L55
L1
L99
R14
L82`

func TestPart1(t *testing.T) {
	input := Parse(TEST_INPUT)
	result := Part1(&input)
	if result < 0 {
		t.Errorf("Part1Day01() returned negative value: %d", result)
	}
}

func TestPart2(t *testing.T) {
	input := Parse(TEST_INPUT)
	result := Part2(&input)
	if result < 0 {
		t.Errorf("Part2Day01() returned negative value: %d", result)
	}
}
