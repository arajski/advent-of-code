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

	defer inFile.Close()

	scanner := bufio.NewScanner(inFile)

	sumGames := 0
	for scanner.Scan() {
		line := scanner.Text()
		game := strings.Split(line, ":")

		sets := strings.Split(game[1], ";")
		power := 0
		limits := map[string]int{"green": 0, "red": 0, "blue": 0}

		for _, set := range sets {
			cubes := strings.Split(set, ",")
			for _, cube := range cubes {
				vals := strings.Split(strings.Trim(cube, " "), " ")
				cubeVal, _ := strconv.Atoi(vals[0])
				if limits[vals[1]] < cubeVal {
					limits[vals[1]] = cubeVal
				}
			}
		}

		power = limits["green"] * limits["blue"] * limits["red"]
		sumGames += power
	}
	fmt.Printf("%d", sumGames)
}
