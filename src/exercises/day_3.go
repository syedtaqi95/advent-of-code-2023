package exercises

import (
	"fmt"
	"os"
)

func RunDay3() {
	fmt.Println("Running exercise 3...")

	// Open the data file for reading
	filepath := "./data/day_3.data"
	file, err := os.Open(filepath)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	// Loop through each line
}
