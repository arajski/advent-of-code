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
	calibration := 0

	for scanner.Scan() {
		line := scanner.Text()
		var left string
		var leftIndex = -1
		var right string
		var rightIndex = 0
		var numbers = []string{"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}

		for i := 0; i < len(line); i++ {
			_, err := strconv.Atoi(string(line[i]))
			if err == nil {
				left = string(line[i])
				leftIndex = i
				//				fmt.Printf("Found left: %s\n", left)
				break
			}
		}

		for i := len(line) - 1; i >= 0; i-- {
			_, err := strconv.Atoi(string(line[i]))
			if err == nil {
				right = string(line[i])
				rightIndex = i
				//				fmt.Printf("Found right: %s\n", right)
				break
			}
		}

		for idx, num := range numbers {
			fi := strings.Index(line, num)
			if fi >= 0 && (fi <= leftIndex || leftIndex < 0) {
				leftIndex = fi
				left = fmt.Sprintf("%d", idx+1)
			}

			ri := strings.LastIndex(line, num)
			if ri > rightIndex {
				rightIndex = ri
				right = fmt.Sprintf("%d", idx+1)
			}
		}

		val, err := strconv.Atoi(fmt.Sprintf("%s%s", left, right))
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		calibration += val
	}
	fmt.Printf("%d\n", calibration)
}
