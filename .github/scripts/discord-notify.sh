#!/bin/bash

# Discord notification script for enhanced notifications
# Usage: ./discord-notify.sh <status> <job_name> [additional_info]

STATUS=$1
JOB_NAME=$2
ADDITIONAL_INFO=$3

# Emojis based on status
case $STATUS in
  "success")
    EMOJI="✅"
    COLOR="3066993"  # Green
    TITLE="🎉 Success!"
    ;;
  "failure")
    EMOJI="❌"
    COLOR="15158332"  # Red
    TITLE="🚨 Failure Alert!"
    ;;
  "warning")
    EMOJI="⚠️"
    COLOR="16776960"  # Yellow
    TITLE="⚠️ Warning"
    ;;
  *)
    EMOJI="ℹ️"
    COLOR="3447003"  # Blue
    TITLE="ℹ️ Info"
    ;;
esac

# Get commit info
COMMIT_MSG=$(git log --format='%s' -n 1 $GITHUB_SHA)
COMMIT_AUTHOR=$(git log --format='%an' -n 1 $GITHUB_SHA)

# Create rich embed message
curl -X POST "$DISCORD_WEBHOOK" \
  -H "Content-Type: application/json" \
  -d "{
    \"content\": \"$EMOJI **<@1078639209167990814> CI/CD Update** $EMOJI\",
    \"embeds\": [{
      \"title\": \"$TITLE\",
      \"description\": \"**Repository:** \`$GITHUB_REPOSITORY\`\\n**Job:** \`$JOB_NAME\`\\n**Branch:** \`$GITHUB_REF_NAME\`\",
      \"color\": $COLOR,
      \"fields\": [
        {
          \"name\": \"📝 Commit Message\",
          \"value\": \"\`\`\`$COMMIT_MSG\`\`\`\",
          \"inline\": false
        },
        {
          \"name\": \"👤 Author\",
          \"value\": \"$COMMIT_AUTHOR\",
          \"inline\": true
        },
        {
          \"name\": \"🔗 Details\",
          \"value\": \"[View Run]($GITHUB_SERVER_URL/$GITHUB_REPOSITORY/actions/runs/$GITHUB_RUN_ID)\",
          \"inline\": true
        }
      ],
      \"footer\": {
        \"text\": \"Fish Cake Kitchen CI/CD • $(date -u)\",
        \"icon_url\": \"https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png\"
      }
    }]
  }"