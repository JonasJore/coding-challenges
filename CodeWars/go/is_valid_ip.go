// https://www.codewars.com/kata/515decfd9dcfc23bb6000006
package main

import (
	"strconv"
	"strings"
)

func Is_valid_ip(ip string) bool {
	ip_arr := strings.Split(ip, ".")
  
	if len(ip_arr) != 4 {
		return false
	}
  
	for _, element := range ip_arr {
		if len(element) > 1 && element[0:1] == "0" {
			return false
		}

		element_int, err := strconv.Atoi(element)

		if err != nil {
			return false
		}
		if !(element_int >= 0 && element_int <= 255) {
			return false
		}
	}

	return true
}

func main() {
	test := "01.02.03.04"
	//test_2 := "12.-34.56.78"
	Is_valid_ip(test)
}