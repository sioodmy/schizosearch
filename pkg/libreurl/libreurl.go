package libreurl

import (
	"fmt"
	"strings"
)

var libre map[string]string = map[string]string{
	"en.wikipedia.org":  "wikiless.org",
	"imgur.com":         "rimgo.projectsegfau.lt",
	"medium.com":        "scribe.rip",
	"tekstowo.pl":       "davilarek.github.io/TekstoLibre/?",
	"stackoverflow.com": "code.whatever.social",
	"mathoverflow.net":  "code.whatever.social/exchange/mathoverflow.net",
}

func LibreUrl(url *string) {
	for spyware, replacement := range libre {
		if strings.Contains(*url, spyware) {
			fmt.Println("found", spyware, " replacing with ", replacement)
			*url = strings.Replace(*url, spyware, replacement, 1)

		}
	}

}
