package search

import (
	"github.com/sioodmy/schizosearch/pkg/generic"
)

func Search(query string) []generic.GenericResult {
	// results_chan := make(chan []generic.GenericResult)

	engines := []generic.GenericEngine{
		{
			Search_url:       "https://html.duckduckgo.com/html/?q=%s&kd=-1",
			Res_selector:     "div.web-result",
			Url_selector:     "a.result__a",
			Title_selector:   "a.result__a",
			Snippet_selector: "a.result__snippet",
		},
		{
			Search_url:       "https://search.brave.com/search?q=%s&nfpr=1&spellcheck=0",
			Res_selector:     "div.snippet",
			Url_selector:     "a.h",
			Title_selector:   "div.heading-serpresult",
			Snippet_selector: "div.snippet-description",
		},
	}

	results := make(chan []generic.GenericResult)

	for _, engine := range engines {
		go func(e generic.GenericEngine) {
			results <- e.Search(query)
		}(engine)
	}

	return <-results
}
