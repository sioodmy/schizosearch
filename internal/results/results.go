package results

import (
	"html/template"
	"log"
	"net/http"

	"github.com/sioodmy/schizosearch/internal/genericsearch"
	"github.com/sioodmy/schizosearch/internal/imagesearch"
	"github.com/sioodmy/schizosearch/internal/leetx"
	"github.com/sioodmy/schizosearch/pkg/generic"
)

type Result struct {
	Url     string
	Title   string
	Snippet string
}

// type TorrentResult struct {
// 	Name     string
// 	Seeders  string
// 	Leechers string
// 	Magnet   string
// 	Size     string
// 	Source   string
// }

type HeaderData struct {
	Query string
	Type  byte
}

type GenericResultPage struct {
	Header  HeaderData
	Results []generic.GenericResult
}
type ImageResultPage struct {
	Header  HeaderData
	Results []imagesearch.ImageResult
}
type TorrentResultPage struct {
	Header  HeaderData
	Results []leetx.TorrentResult
}

func GetResults(w http.ResponseWriter, r *http.Request) {
	tpl, err := template.New("").ParseGlob("static/templates/*.html")
	if err != nil {
		log.Fatalln("Template error ", err)
	}

	q := r.URL.Query().Get("q")
	t := r.URL.Query().Get("t")

	switch t {
	case "img":
		data := imagesearch.SearchImg(q)

		template_data := ImageResultPage{
			Header: HeaderData{
				Query: q,
				Type:  1,
			},
			Results: data,
		}
		tpl.ExecuteTemplate(w, "images.html", &template_data)
	case "torrent":

		data := leetx.Search(q)

		template_data := TorrentResultPage{
			Header: HeaderData{
				Query: q,
				Type:  2,
			},
			Results: data,
		}
		tpl.ExecuteTemplate(w, "torrents.html", &template_data)
	default:
		data := search.Search(q)

		template_data := GenericResultPage{
			// TODO: Find a better way
			Header: HeaderData{
				Query: q,
				Type:  0,
			},
			Results: data,
		}
		tpl.ExecuteTemplate(w, "results.html", &template_data)
	}
}
