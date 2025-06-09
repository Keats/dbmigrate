DEFINE TABLE companies SCHEMAFULL
    PERMISSIONS
        FOR select, create, update, delete FULL;

DEFINE FIELD OVERWRITE name ON place TYPE string;
