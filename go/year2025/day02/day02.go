package day02

import (
	"strconv"
	"strings"
)

type Input [][]int64

func Parse(input string) Input {
	var result Input
	input = strings.TrimSpace(input)

	for pair := range strings.SplitSeq(input, ",") {
		if pair == "" {
			continue
		}
		parts := strings.Split(pair, "-")
		var nums []int64
		for _, n := range parts {
			if num, err := strconv.ParseInt(n, 10, 64); err == nil {
				nums = append(nums, num)
			}
		}
		result = append(result, nums)
	}

	return result
}

func Part1(input *Input) int64 {
	var sum int64
	for _, r := range *input {
		if len(r) == 2 {
			for num := r[0]; num <= r[1]; num++ {
				s := strconv.FormatInt(num, 10)
				if isInvalidID(s) {
					sum += num
				}
			}
		}
	}
	return sum
}

func Part2(input *Input) int64 {
	var sum int64
	for _, r := range *input {
		if len(r) == 2 {
			for num := r[0]; num <= r[1]; num++ {
				s := strconv.FormatInt(num, 10)
				if isInvalidIDPart2(s) {
					sum += num
				}
			}
		}
	}
	return sum
}

func isInvalidID(id string) bool {
	length := len(id)
	if length % 2 != 0 {
		return false
	}

	mid := length / 2
	return id[:mid] == id[mid:]
}

func isInvalidIDPart2(id string) bool {
	length := len(id)

	for patternLen := 1; patternLen < length; patternLen++ {
		if length % patternLen == 0 {
			pattern := id[:patternLen]
			allMatch := true

			for i := 0; i < length; i += patternLen {
				chunk := id[i : i+patternLen]
				if chunk != pattern {
					allMatch = false
					break
				}
			}

			if allMatch {
				return true
			}
		}
	}

	return false
}
