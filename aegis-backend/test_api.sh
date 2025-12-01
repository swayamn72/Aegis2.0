#!/bin/bash

BASE_URL="http://localhost:8000/api/v1"

echo "🧪 Testing Aegis Gaming Backend API"

# Test health check
echo "1. Health Check"
curl -s "$BASE_URL/../health"
echo -e "\n"

# Test create chat
echo "2. Create Chat"
CHAT_RESPONSE=$(curl -s -X POST "$BASE_URL/chats" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test Gaming Chat",
    "chat_type": "general",
    "created_by": "user-123"
  }')
echo $CHAT_RESPONSE
CHAT_ID=$(echo $CHAT_RESPONSE | jq -r '.data')
echo -e "\n"

# Test send message
echo "3. Send Message"
curl -s -X POST "$BASE_URL/chats/$CHAT_ID/messages" \
  -H "Content-Type: application/json" \
  -d '{
    "sender": "user-123",
    "message": "Hello gaming world!",
    "message_type": "text"
  }'
echo -e "\n"

# Test create post
echo "4. Create Post"
POST_RESPONSE=$(curl -s -X POST "$BASE_URL/posts" \
  -H "Content-Type: application/json" \
  -d '{
    "author": "user-123",
    "title": "Epic Gaming Moment",
    "content": "Just had an amazing tournament match!",
    "post_type": "tournament",
    "tags": ["gaming", "tournament", "epic"]
  }')
echo $POST_RESPONSE
echo -e "\n"

echo "✅ API Tests Complete!"
