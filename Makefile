create:
		docker run --name migrate-postgresql \
		-e 'POSTGRES_USER=pg' \
		-e 'POSTGRES_DB=migrate' \
		-p 5777:5432 \
		-d postgres:9.5.0

		docker run --name migrate-mysql \
		-e 'MYSQL_DATABASE=migrate' \
		-e 'MYSQL_USER=mg' \
		-e 'MYSQL_PASSWORD=pass' \
		-e 'MYSQL_ROOT_PASSWORD=root' \
		-p 5789:3306 \
		-d mysql:5.7

remove:
		docker rm migrate-postgresql
		docker rm migrate-mysql

stop:
		docker stop migrate-postgresql
		docker stop migrate-mysql

start:
		docker start migrate-postgresql
		docker start migrate-mysql

recreate: stop remove create start
