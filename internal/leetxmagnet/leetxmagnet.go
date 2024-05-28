package leetxmagnet

import (
	"fmt"
	"net/http"
	"net/url"

	"github.com/gocolly/colly"
)

func init() {
	http.HandleFunc("/magnet1337", GetMagnet)
}

// 1337 does not provide magnet urls on search results page,
// so using this workaround we avoid making n requests per search
func GetMagnet(w http.ResponseWriter, r *http.Request) {
	c := colly.NewCollector()

	var magnet string

	target := r.URL.Query().Get("target")
	target_url := fmt.Sprintf("https://1337x.to%s", target)

	found := false
	c.OnHTML("main > div > div > div > div > div > ul > li > a[href]", func(h *colly.HTMLElement) {
		if found {
			return
		}
		magnet = h.Attr("href")
		found = true
	})

	c.Visit(target_url)

	sanitized, err := removeTracker(magnet)
	if err != nil {
		magnet = sanitized
	}

	http.Redirect(w, r, magnet, http.StatusMovedPermanently)
}

func removeTracker(magnet string) (string, error) {
	parsedURL, err := url.Parse(magnet)
	if err != nil {
		return "", err
	}

	query := parsedURL.Query()
	query.Del("tr") // Remove all occurrences of the "tr" parameter
	parsedURL.RawQuery = query.Encode()

	return parsedURL.String(), nil
}
