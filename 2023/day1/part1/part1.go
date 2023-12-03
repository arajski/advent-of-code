package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	inFile, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}

	defer inFile.Close()

	scanner := bufio.NewScanner(inFile)
	calibration := 0

	for scanner.Scan() {
		line := scanner.Text()
		var left string
		var right string

		for i := 0; i < len(line); i++ {
			_, err := strconv.Atoi(string(line[i]))
			if err == nil {
				left = string(line[i])
				//				fmt.Printf("Found left: %s\n", left)
				break
			}
		}

		for i := len(line) - 1; i >= 0; i-- {
			_, err := strconv.Atoi(string(line[i]))
			if err == nil {
				right = string(line[i])
				//				fmt.Printf("Found right: %s\n", right)
				break
			}
		}
		val, err := strconv.Atoi(fmt.Sprintf("%s%s", left, right))
		if err != nil {
			os.Exit(1)
		}
		calibration += val
	}
	fmt.Printf("%d\n", calibration)
}
