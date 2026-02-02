#!/bin/bash

# Script per testare le API

BASE_URL="http://127.0.0.1:3000"

echo "=== Test API Backend - Hexagonal Architecture ==="
echo ""

# Health Check
echo "1. Health Check..."
curl -X GET "$BASE_URL/health"
echo -e "\n"

# Create User
echo "2. Creazione utente..."
USER_RESPONSE=$(curl -s -X POST "$BASE_URL/api/users" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "mario.rossi@example.com",
    "name": "Mario Rossi",
    "password": "password123"
  }')
echo "$USER_RESPONSE" | jq '.'
USER_ID=$(echo "$USER_RESPONSE" | jq -r '.data.id')
echo "User ID: $USER_ID"
echo ""

# Get User
echo "3. Recupero utente..."
curl -s -X GET "$BASE_URL/api/users/$USER_ID" | jq '.'
echo ""

# Get All Users
echo "4. Lista di tutti gli utenti..."
curl -s -X GET "$BASE_URL/api/users" | jq '.'
echo ""

# Update User
echo "5. Aggiornamento utente..."
curl -s -X PUT "$BASE_URL/api/users/$USER_ID" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Mario Verdi"
  }' | jq '.'
echo ""

# Delete User
echo "6. Eliminazione utente..."
curl -s -X DELETE "$BASE_URL/api/users/$USER_ID"
echo ""

echo "=== Test completati ==="
echo "I dati sono persistiti in backend/data/users.json"
