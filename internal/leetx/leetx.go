package leetx

// go doesnt allow numerical package names :c

import (
	"fmt"
	"log"
	"strings"

	"github.com/gocolly/colly"
)

type TorrentResult struct {
	Name     string
	Seeders  string
	Leechers string
	Magnet   string
	Size     string
	Source   string
}

func Search(query string) []TorrentResult {
	target_url := fmt.Sprintf("https://1337x.to/search/%s/1/", query)
	c := colly.NewCollector()
	c.Async = false

	c.OnError(func(_ *colly.Response, err error) {
		log.Println("Something went wrong ", err)
	})

	// TODO: Find a better way to crack anti-scraping soyware
	c.UserAgent = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36"

	var results []TorrentResult

	c.OnHTML("tr", func(e *colly.HTMLElement) {
		r := TorrentResult{}
		r.Source = "1337x.to"
		r.Name = strings.TrimSpace(e.ChildText("td.coll-1 a"))
		r.Seeders = e.ChildText("td.coll-2")
		r.Leechers = e.ChildText("td.coll-3")

		size := e.ChildText("td.coll-4")
		attrs := e.ChildAttrs("td.coll-1 a", "href")
		if len(attrs) == 2 {
			r.Magnet = fmt.Sprintf("/magnet1337?target=%s", attrs[1])
		}

		r.Size = strings.TrimSuffix(size, r.Seeders)

		if len(r.Name) > 1 {
			results = append(results, r)
		}
	})

	c.Visit(target_url)

	return results
}
