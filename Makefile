seed:
	sqlx database reset
	cargo run -- db:seed
run-db:
	docker compose -f docker-compose.db.yml up -d
run:
	docker compose -f docker-compose.db.yml  -f docker-compose.yml up -d
down: 
	docker compose -f docker-compose.db.yml  -f docker-compose.yml down
