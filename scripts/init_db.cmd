REM # Check if a custom user has been set, otherwise default to 'postgres'
set DB_USER=postgres
REM # Check if a custom password has been set, otherwise default to 'password'
SET DB_PASSWORD=password
REM # Check if a custom database name has been set, otherwise default to 'newsletter'
SET DB_NAME=newsletter
REM # Check if a custom port has been set, otherwise default to '5432'
SET DB_PORT=5432

REM # Launch postgres using Docker
docker run -e POSTGRES_USER=%DB_USER% -e POSTGRES_PASSWORD=%DB_PASSWORD% -e POSTGRES_DB=%DB_NAME% -p %DB_PORT%:5432 -d postgres postgres -N 1000
REM  # ^ Increased maximum number of connections for testing purposes