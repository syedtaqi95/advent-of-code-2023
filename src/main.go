package main

import (
	"fmt"
	"os"
	"strconv"

	"github.com/syedtaqi95/advent-of-code-2023/exercises"
)

func main() {
	args := os.Args

	if len(args) != 2 {
		fmt.Println("Usage: go run . <day_number>")
		fmt.Println("Please provide the day number as a command-line argument.")
		os.Exit(1)
	}

	day_num, conversion_err := strconv.Atoi(args[1])
	if conversion_err != nil {
		fmt.Println("Could not parse day number. Must be an integer between 2 and 25.")
		os.Exit(1)
	}

	runtime_err := exercises.RunExercise(day_num)
	if runtime_err != nil {
		fmt.Println(runtime_err)
		os.Exit(1)
	}
}
