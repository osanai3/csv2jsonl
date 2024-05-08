package main

import (
	"encoding/csv"
	"encoding/json"
	"fmt"
	"log"
	"os"
)

func main() {
	reader := csv.NewReader(os.Stdin)
	lines, err := reader.ReadAll()
	if err != nil {
		log.Fatal(err)
	}
	if len(lines) == 0 {
		log.Fatal("no header")
	}
	header := lines[0]
	for _, record := range lines[1:] {
		r := map[string]string{}
		for i, field := range record {
			r[header[i]] = field
		}
		bytes, err := json.Marshal(r)
		if err != nil {
			log.Fatal(err)
		}
		fmt.Printf("%s\n", string(bytes))
	}
}
