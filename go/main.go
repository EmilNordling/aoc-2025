package main

import (
	"fmt"
	"os"
	"time"

	dayPkg "aoc/year2025/day02"
)

func main() {
	day := 2
	path := fmt.Sprintf("../input/year2025/day%02d.txt", day)

	data, err := os.ReadFile(path)
	if err != nil {
		fmt.Println("Failed to read input file")
		return
	}

	input := dayPkg.Parse(string(data))

	start1 := time.Now()
	outPart1 := dayPkg.Part1(&input)
	elapsed1 := time.Since(start1)
	fmt.Printf("%d | time: %v\n", outPart1, elapsed1)

	start2 := time.Now()
	outPart2 := dayPkg.Part2(&input)
	elapsed2 := time.Since(start2)
	fmt.Printf("%d | time: %v\n", outPart2, elapsed2)
}
