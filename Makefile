BINARY=schizosearch

build: 
	go build -o ./bin/${BINARY} main.go

run: build
	./bin/${BINARY}

clean: 
	go clean
	rm bin/${BINARY}
	
