DEFINE TABLE users SCHEMAFULL
    PERMISSIONS
        FOR select, create, update, delete FULL;

DEFINE FIELD OVERWRITE name ON username TYPE string;
DEFINE FIELD OVERWRITE name ON password TYPE string;
