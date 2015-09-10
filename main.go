package main

import (
	"flag"
	"fmt"
	"net/url"
	"os"
	"strings"
)

func main() {
	flag.Parse()

	switch strings.ToLower(flag.Arg(0)) {
	case "e", "escape":
		input := flag.Args()[1:]
		for _, arg := range input {
			fmt.Println(url.QueryEscape(arg))
		}
	case "u", "unescape":
		input := flag.Args()[1:]
		for _, arg := range input {
			unescaped, err := url.QueryUnescape(arg)
			if err != nil {
				fmt.Println("error:", err)
			} else {
				fmt.Println(unescaped)
			}
		}
	default:
		fmt.Fprintln(os.Stderr, "Only accepted commands are: (e)scape, (u)nescape.")
		os.Exit(1)
	}
}
