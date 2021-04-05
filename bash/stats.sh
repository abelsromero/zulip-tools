#!/bin/bash

set -euo pipefail

readonly DOMAIN="${DOMAIN:?must be set}"
readonly BOT_EMAIL_ADDRESS="${BOT_EMAIL_ADDRESS:?must be set}"
readonly BOT_API_KEY="${BOT_API_KEY:?must be set}"

readonly FULL_DOMAIN="https://$DOMAIN.zulipchat.com/api/v1"

echo "== Users:"
curl -sS -G "$FULL_DOMAIN/users" -u "$BOT_EMAIL_ADDRESS:$BOT_API_KEY" | jq '.members | length'

echo "== Streams:"
readonly streams_data=$(curl -sS -G "$FULL_DOMAIN/streams" -u "$BOT_EMAIL_ADDRESS:$BOT_API_KEY")
echo "Count: $(echo $streams_data | jq '.streams | length')"

readonly stream_ids=$(echo $streams_data | jqn '.streams[].stream_id')
readonly stream_names=$(echo $streams_data | jq '.streams[].name')

stream_ids_arr=($stream_ids)
#stream_names_arr=($stream_names)

echo $stream_names
for id in $stream_ids
do
  curl -sS -G "$FULL_DOMAIN/users/me/$id/topics" -u "$BOT_EMAIL_ADDRESS:$BOT_API_KEY" | jq '.topics | length'
done
