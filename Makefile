BINARY=schizosearch

build:
	go build -o ./bin/${BINARY} cmd/schizosearch/main.go

run: build
	./bin/${BINARY}

clean: 
	go clean
	rm bin/${BINARY}
	
