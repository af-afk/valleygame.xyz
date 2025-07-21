#!/bin/sh -e

docker build -t markovgeist/valleygame/db -f Dockerfile.db .

docker run \
	-e POSTGRES_USER=${POSTGRES_USER:-markovgeist} \
	-e POSTGRES_PASSWORD=${POSTGRES_PASSWORD:-markovgeist} \
	-p 5432:5432 \
	docker.io/markovgeist/valleygame/db \
	-c log_statement=all
