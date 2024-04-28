package main

import (
	"embed"
	"io/fs"
	"log"
	"net/http"

	"github.com/sioodmy/schizosearch/internal/results"
)

//go:embed static
var static embed.FS

func serve_static() http.Handler {
	sub, err := fs.Sub(static, "static")
	if err != nil {
		panic(err)
	}
	file_server := http.FileServer(http.FS(sub))
	return file_server
}
func main() {

	http.Handle("/", serve_static())
	http.HandleFunc("/search", results.GetResults)

	err := http.ListenAndServe(":3000", nil)
	if err != nil {
		log.Fatalln(err)
	}

}
