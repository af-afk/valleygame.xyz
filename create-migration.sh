#!/bin/sh -u

timestamp="$(date +%s)"

name="$1"

name="db/${timestamp}-${name}.sql"

cat >"$name" <<EOF
-- migrate:up

-- migrate:down
EOF

$EDITOR "$name" &
