package imagesearch

import (
	"io"
	"log"
	"net/http"

	"github.com/tidwall/gjson"
)

type ImageResult struct {
	Title, Url, Image string
}

func SearchImg(query string) [50]ImageResult {

	resp, err := http.Get("https://api.qwant.com/v3/search/images?q=" + query + "&t=images&count=50&locale=en_us&offset=0&device=desktop&tgp=3&safesearch=1")

	if err != nil {
		log.Println("Couldn't fetch images from qwant")
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Println("Couldn't parse request")
	}

	parsed := gjson.Get(string(body), "data.result.items")

	var images [50]ImageResult

	for i, img := range parsed.Array() {

		imgmap := img.Map()

		// TODO: find a better way of parsing json data
		im := ImageResult{
			Title: imgmap["title"].String(),
			Url:   imgmap["url"].String(),
			Image: imgmap["media"].String(),
		}
		images[i] = im
	}

	return images

}
