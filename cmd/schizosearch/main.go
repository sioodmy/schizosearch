package main

import (
	"github.com/sioodmy/schizosearch/internal/results"
	"log"
	"net/http"
)

func main() {
	file_server := http.FileServer(http.Dir("./app"))

	http.Handle("/", file_server)
	http.HandleFunc("/search", results.GetResults)

	err := http.ListenAndServe(":3000", nil)
	if err != nil {
		log.Fatalln(err)
	}

}
