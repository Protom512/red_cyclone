build: .
	docker build . -t red_cyclone
run: build
	docker run  --env-file ${PWD}/.env red_cyclone