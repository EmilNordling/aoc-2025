package day01

import "testing"

func TestPart1Day01(t *testing.T) {
	input := Parse("R10\nL5\nR15")
	result := Part1(&input)
	if result < 0 {
		t.Errorf("Part1Day01() returned negative value: %d", result)
	}
}

func TestPart2Day01(t *testing.T) {
	input := Parse("R10\nL5\nR15")
	result := Part2(&input)
	if result < 0 {
		t.Errorf("Part2Day01() returned negative value: %d", result)
	}
}
