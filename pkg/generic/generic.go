package generic

import (
	"fmt"
	"log"

	"github.com/gocolly/colly"
	"github.com/sioodmy/schizosearch/pkg/libreurl"
)

type GenericEngine struct {
	Search_url, Res_selector, Url_selector, Title_selector, Snippet_selector string
}
type GenericResult struct {
	Url, Title, Snippet string
}

func (s *GenericEngine) Search(query string) []GenericResult {

	c := colly.NewCollector()

	c.OnError(func(_ *colly.Response, err error) {
		log.Println("Something went wrong: ", err, s.Search_url)
	})

	// TODO: Find a better way to crack anti-scraping soyware
	c.UserAgent = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36"

	target_url := fmt.Sprintf(s.Search_url, query)

	var results []GenericResult

	c.OnHTML(s.Res_selector, func(e *colly.HTMLElement) {
		r := GenericResult{}
		r.Title = e.ChildText(s.Title_selector)
		r.Url = e.ChildAttr(s.Url_selector, "href")
		r.Snippet = e.ChildText(s.Snippet_selector)

		libreurl.LibreUrl(&r.Url)

		// TODO: Find a better way of handling empty results
		if len(r.Url) > 1 {
			results = append(results, r)
		}
	})

	c.Visit(target_url)

	return results

}
