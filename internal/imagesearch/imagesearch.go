package imagesearch

import (
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
)

type ImageResult struct {
	Title     string `json:"title"`
	Url       string `json:"url"`
	ImageUrl  string `json:"media"`
	Thumbnail string `json:"thumbnail"`
}

func SearchImg(query string) []ImageResult {
	url := "https://api.qwant.com/v3/search/images?q=" + query + "&t=images&count=50&locale=en_us&offset=0&device=desktop&tgp=3&safesearch=1"
	client := &http.Client{}

	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		fmt.Println("Error creating request:", err)
	}
	req.Header.Set("User-Agent", "Mozilla/5.0 (iPad; CPU OS 17_5_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4.1 Mobile/15E148 Safari/604.1")

	resp, err := client.Do(req)
	if err != nil {
		log.Println("Couldn't fetch images from qwant")
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Println("Couldn't parse request")
	}

	type Result struct {
		Items []ImageResult `json:"items"`
	}

	type Data struct {
		Result Result `json:"result"`
	}

	type Response struct {
		Status string `json:"status"`
		Data   Data   `json:"data"`
	}

	var response Response
	if err := json.Unmarshal(body, &response); err != nil {
		fmt.Println("Error unmarshaling JSON:", err)
	}

	// Check if the request was successful
	if response.Status != "success" {
		fmt.Println("Request was not successful", response)
	}

	fmt.Println(response.Data.Result.Items)
	return response.Data.Result.Items

}
