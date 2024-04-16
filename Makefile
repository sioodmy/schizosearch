BINARY=schizosearch
STYLEDIR=./web

build: styles
	go build -o ./bin/${BINARY} cmd/schizosearch/main.go

styles:
	mkdir -p app
	cp web/index.html app/index.html
	sass ${STYLEDIR}/style.scss:app/style.css

run: build
	./bin/${BINARY}

clean: 
	go clean
	rm bin/${BINARY}
	
