package day01

import (
	"strconv"
	"strings"
)

type Input []int32

func Parse(input string) Input {
	var result Input

	for line := range strings.SplitSeq(input, "\n") {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}

		if len(line) < 2 {
			continue
		}

		number, err := strconv.ParseInt(line[1:], 10, 32)
		if err != nil {
			continue
		}

		switch line[0] {
		case 'R':
			result = append(result, int32(number))
		case 'L':
			result = append(result, -int32(number))
		}
	}

	return result
}

func Part1(input *Input) int32 {
	point := int32(50)
	password := int32(0)

	for _, step := range *input {
		point = mod(point+step, 100)
		if point == 0 {
			password++
		}
	}

	return password
}

func Part2(input *Input) int32 {
	point := int32(50)
	password := int32(0)
	rhs := int32(100)

	for _, step := range *input {
		if step >= 0 {
			password += (point + step) / rhs
		} else {
			reversed := (rhs - point) % rhs
			password += (reversed - step) / rhs
		}

		point = mod(point+step, rhs)
	}

	return password
}

func mod(a, b int32) int32 {
	return ((a % b) + b) % b
}
