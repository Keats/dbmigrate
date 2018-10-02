#!/bin/bash
#
# If this is Ubuntu 14.04 (Trusty), git clone SQLCipher and compile it.
# Run cargo run with the corret environment variables so rusqlite will use the
# compiled SQLCipher.

function cargo_run_status () {
        ACTUAL_PKGCONF="$(pkg-config --libs-only-l sqlcipher | xargs)"
        BAD_PKGCONF="-lsqlite3"
        BUILD_DIR="$HOME/sqlcipher-for-dbmigrate"
	CODENAME="$(lsb_release -s -c)"

        SQLCIPHER_URL="https://github.com/sqlcipher/sqlcipher"

	# Necessary for cargo run status
	export DBMIGRATE_PATH=./examples/migrations/
	export DBMIGRATE_URL=sqlcipher://user:password@localhost`pwd`/migrate.sqlcipher

	# If Ubuntu 14.04
        if [ "xtrusty" = "x${CODENAME}" ] && [ "x${ACTUAL_PKGCONF}" = "x${BAD_PKGCONF}" ];
        then
                sudo apt-get -q install -y libssl-dev tclsh make > /dev/null
		if [ ! -f "$BUILD_DIR"/.libs/libsqlcipher.so ];
		then
			pushd . > /dev/null
			rm -rf "$BUILD_DIR" && git clone -q $SQLCIPHER_URL "$BUILD_DIR" && cd "$BUILD_DIR" && ./configure --quiet --enable-tempstore=yes --disable-tcl CFLAGS="-DSQLITE_HAS_CODEC" LDFLAGS="-lcrypto" && make > /dev/null
			popd > /dev/null
		fi
		export SQLCIPHER_INCLUDE_DIR=$BUILD_DIR
		export SQLCIPHER_LIB_DIR=$BUILD_DIR/.libs
		export LD_LIBRARY_PATH=$BUILD_DIR/.libs
        fi
	cargo run --no-default-features --features sqlcipher_support -- status
}

cargo_run_status
