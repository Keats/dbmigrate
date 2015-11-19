create:
		docker run --name migrate-postgresql -d \
		-e 'POSTGRES_USER=pg' \
		-e 'POSTGRES_PASSWORD=pg' \
		-e 'POSTGRES_DB=migrate' \
		-p 5432:5432 \
		postgres:9.4.4

remove:
		docker rm migrate-postgresql

stop:
		docker stop migrate-postgresql

start:
		docker start migrate-postgresql
