package main

import (
	"fmt"
	"bufio"
	"os"
	"log"
	"strconv"
)

func check(e error) {
    if e != nil {
        log.Fatal(e)
    }
}

func main() {

	file, err := os.Open("day1_sln1_input.txt")
	check(err)

	defer file.Close()

	var result int = 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		fmt.Println(scanner.Text())
		
		fuel, str_err := strconv.Atoi(scanner.Text())
		check(str_err)

		module_fuel := (fuel / 3) - 2

		result += module_fuel


	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	

	fmt.Println(result)
}