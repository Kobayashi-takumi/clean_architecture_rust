up:
	docker compose up -d
down:
	docker compose down --rmi all --volumes --remove-orphans
recreate:
	docker compose stop $(C)
	docker compose rm -f $(C)
	docker compose up -d $(C)
app:
	docker compose exec app bash
db:
	docker compose run --rm postgres sh -c "psql -U sa -W -h postgres -d app"
m-run:
	docker compose run --rm app sh -c "sqlx migrate run"
m-revert:
	docker compose run --rm app sh -c "for i in {1..2}; do sqlx migrate revert; done"
m-add:
	docker compose run --rm app sh -c "sqlx migrate add -r $(ARGS)"
prepare:
	docker compose run --rm app sh -c "cargo sqlx prepare"

