#!/bin/bash
for FILE in $(ls commands)
do
    echo "Uploading $FILE";
    DATA=$(cat commands/$FILE)
    echo "Command: $DATA"
    curl --data "@commands/$FILE" \
    -H "Authorization: Bot $DISCORD_TOKEN" \
    -H "Content-Type: application/json" \
    "https://discord.com/api/v9/applications/$APPLICATION_ID/commands";
    echo "";
done

echo "Commands: "

curl -H "Authorization: Bot $DISCORD_TOKEN" \
"https://discord.com/api/applications/$APPLICATION_ID/commands";