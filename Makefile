BINARY=schizosearch
STYLEDIR=./web/theme

build: styles
	go build -o ./bin/${BINARY} main.go

styles:
	mkdir -p app
	sass ${STYLEDIR}/style.scss:app/style.css

run: build
	./bin/${BINARY}

clean: 
	go clean
	rm bin/${BINARY}
	rm web/theme/**.css.map
	
