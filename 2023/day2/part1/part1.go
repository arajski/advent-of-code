package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	inFile, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}

	limits := map[string]int{"green": 13, "red": 12, "blue": 14}

	defer inFile.Close()

	scanner := bufio.NewScanner(inFile)

	sumGames := 0
	for scanner.Scan() {
		line := scanner.Text()
		game := strings.Split(line, ":")
		game_details := strings.Split(game[0], " ")
		game_id, _ := strconv.Atoi(game_details[1])
		isPossible := true

		sets := strings.Split(game[1], ";")
		for _, set := range sets {
			cubes := strings.Split(set, ",")
			for _, cube := range cubes {
				vals := strings.Split(strings.Trim(cube, " "), " ")
				cubeVal, _ := strconv.Atoi(vals[0])
				if cubeVal > limits[vals[1]] {
					isPossible = false
				}
			}
		}
		if isPossible {
			sumGames += game_id
		}
	}
	fmt.Printf("%d", sumGames)
}
