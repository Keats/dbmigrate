#!/bin/bash
#
# If this is Ubuntu 14.04 (Trusty), git clone SQLCipher and compile it.
# Set environment variables so rusqlite will use the compiled SQLCipher.

function trusty_build_sqlcipher () {
        ACTUAL_PKGCONF="$(pkg-config --libs-only-l sqlcipher | xargs)"
        BAD_PKGCONF="-lsqlite3"
        BUILD_DIR="$HOME/sqlcipher-for-dbmigrate"
	CODENAME="$(lsb_release -s -c)"

        SQLCIPHER_URL="https://github.com/sqlcipher/sqlcipher"

        if [ "xtrusty" = "x${CODENAME}" ] && [ "x${ACTUAL_PKGCONF}" = "x${BAD_PKGCONF}" ];
        then
                sudo apt-get -q install -y libssl-dev tclsh make > /dev/null
		if [ ! -f "$BUILD_DIR"/.libs/libsqlcipher.so ];
		then
			pushd . > /dev/null
			rm -rf "$BUILD_DIR" && git clone -q $SQLCIPHER_URL "$BUILD_DIR" && cd "$BUILD_DIR" && ./configure --quiet --enable-tempstore=yes --disable-tcl CFLAGS="-DSQLITE_HAS_CODEC" LDFLAGS="-lcrypto" && make > /dev/null
			popd > /dev/null
		fi
		export LD_LIBRARY_PATH=$HOME/sqlcipher-for-dbmigrate/.libs
        fi
}

trusty_build_sqlcipher
