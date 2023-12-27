package exercises

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

const (
	numReds      = 12
	numGreens    = 13
	numBlues     = 14
	regexPattern = `Game (\d+):(.+)`
)

var (
	re *regexp.Regexp
)

func init() {
	re, _ = regexp.Compile(regexPattern)
}

type Round struct {
	red   int
	green int
	blue  int
}

func createRound(round_str string) Round {
	round := Round{}

	turns := strings.Split(round_str, ",")
	for _, turn := range turns {
		turn_split := strings.Fields(turn)
		num_cubes_str := strings.TrimSpace(turn_split[0])
		num_cubes, _ := strconv.Atoi(num_cubes_str)
		colour := turn_split[1]

		switch colour {
		case "red":
			round.red += num_cubes
		case "green":
			round.green += num_cubes
		case "blue":
			round.blue += num_cubes
		}
	}
	return round
}

type Game struct {
	id     int
	rounds []Round
}

func createGame(line string) (Game, error) {
	matches := re.FindStringSubmatch(line)
	if matches == nil {
		return Game{}, fmt.Errorf("No matches found")
	}

	id, _ := strconv.Atoi(matches[1])

	var rounds []Round
	round_strings := strings.Split(matches[2], ";")
	for _, round_string := range round_strings {
		round := createRound(round_string)
		rounds = append(rounds, round)
	}

	return Game{
		id:     id,
		rounds: rounds,
	}, nil
}

func isGamePossible(game Game) bool {
	for _, round := range game.rounds {
		if round.red > numReds ||
			round.green > numGreens ||
			round.blue > numBlues {
			return false
		}
	}
	return true
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func findMinCubes(game Game) Round {
	min_cubes := Round{}
	for _, round := range game.rounds {
		min_cubes.red = max(min_cubes.red, round.red)
		min_cubes.green = max(min_cubes.green, round.green)
		min_cubes.blue = max(min_cubes.blue, round.blue)
	}

	return min_cubes
}

func RunDay2() {
	fmt.Println("Running exercise 2...")

	// Open the data file for reading
	filepath := "./data/day_2.data"
	file, err := os.Open(filepath)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	// Loop through each line
	scanner := bufio.NewScanner(file)
	result_1 := 0
	result_2 := 0
	for scanner.Scan() {
		line := scanner.Text()
		game, err := createGame(line)
		if err != nil {
			panic(err)
		}
		
		if isGamePossible(game) {
			result_1 += game.id
		}

		min_cubes := findMinCubes(game)
		result_2 += min_cubes.red * min_cubes.green * min_cubes.blue
	}
	fmt.Println("result_1:", result_1)
	fmt.Println("result_2:", result_2)

	if err := scanner.Err(); err != nil {
		panic(err)
	}
}
