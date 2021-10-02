#!/bin/bash

source .env

echo "Uploading $FILE";
DATA=$(cat commands/$FILE)
echo "Command: $DATA"
curl --data "@commands.json" \
-H "Authorization: Bot $DISCORD_TOKEN" \
-H "Content-Type: application/json" \
-X PUT \
"https://discord.com/api/v9/applications/$DISCORD_APPID/guilds/836386335840665610/commands";
echo "";

echo "Commands: "

curl -H "Authorization: Bot $DISCORD_TOKEN" \
"https://discord.com/api/applications/$DISCORD_APPID/commands";