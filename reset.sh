docker-compose down
docker volume rm suzuya_pgdata
docker-compose up -d postgres
sleep 2
cd server/migration
cargo run

PGPASSWORD=postgres psql -f ./sample_data.sql -p 8765 -U postgres -d suzuya -h localhost
