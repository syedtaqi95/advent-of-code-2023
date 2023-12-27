package main

import (
	"fmt"
	"os"
	"strconv"
)

func main() {
	args := os.Args
	fmt.Println(args)

	if len(args) != 2 {
		fmt.Println("Usage: go run . <day_number>")
		fmt.Println("Please provide the day number as a command-line argument.")
		os.Exit(1)
	}

	day_num, err := strconv.Atoi(args[1])
	if err != nil {
		fmt.Println("Could not parse day number. Must be an integer between 2 and 25.")
		os.Exit(1)
	}

	fmt.Println("Running day", day_num)

}
