.PHONY: up down build shell logs db-shell docs

# Docker Compose file location
COMPOSE_FILE := docker/docker-compose.yml

up:
	docker-compose -f $(COMPOSE_FILE) up -d

down:
	docker-compose -f $(COMPOSE_FILE) down

build:
	docker-compose -f $(COMPOSE_FILE) build

logs:
	docker-compose -f $(COMPOSE_FILE) logs -f

shell:
	docker-compose -f $(COMPOSE_FILE) exec app /bin/bash

db-shell:
	docker-compose -f $(COMPOSE_FILE) exec db mysql -u webrust -psecret webrust_app

docs:
	cd docs && npm install && npm run dev
