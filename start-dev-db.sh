#!/usr/bin/env bash
docker run --name postgres -p 5432:5432 -e POSTGRES_PASSWORD=exchange -e POSTGRES_USER=exchange -e POSTGRES_DB=exchange -d postgres:10.4