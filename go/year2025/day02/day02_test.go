package day02

import "testing"

const testInput = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"

func TestIsInvalidIDPart1(t *testing.T) {
	tests := []string{"11", "22", "1010", "1188511885", "222222", "446446", "38593859"}
	for _, test := range tests {
		if !isInvalidID(test) {
			t.Errorf("isInvalidID(%q) = false, want true", test)
		}
	}
}

func TestPart1(t *testing.T) {
	input := Parse(testInput)
	result := Part1(&input)
	expected := int64(1227775554)
	if result != expected {
		t.Errorf("Part1() = %d, want %d", result, expected)
	}
}

func TestIsInvalidIDPart2(t *testing.T) {
	tests := []string{
		"11", "22", "99", "111", "999", "1010",
		"1188511885", "222222", "446446", "38593859",
		"565656", "824824824", "2121212121",
	}
	for _, test := range tests {
		if !isInvalidIDPart2(test) {
			t.Errorf("isInvalidIDPart2(%q) = false, want true", test)
		}
	}
}

func TestPart2(t *testing.T) {
	input := Parse(testInput)
	result := Part2(&input)
	expected := int64(4174379265)
	if result != expected {
		t.Errorf("Part2() = %d, want %d", result, expected)
	}
}
