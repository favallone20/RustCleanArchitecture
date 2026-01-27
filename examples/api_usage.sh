#!/bin/bash

# Script di esempio per testare l'API
# Assicurati che il server sia in esecuzione prima di eseguire questo script

BASE_URL="http://127.0.0.1:3000"

echo "=== Clean Architecture Rust API Examples ==="
echo ""

# 1. Crea un utente
echo "1. Creating a user..."
USER_RESPONSE=$(curl -s -X POST "$BASE_URL/api/users" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "mario.rossi@example.com",
    "name": "Mario Rossi"
  }')

echo "$USER_RESPONSE" | jq '.'
USER_ID=$(echo "$USER_RESPONSE" | jq -r '.id')
echo ""

# 2. Ottieni l'utente per ID
echo "2. Getting user by ID: $USER_ID"
curl -s -X GET "$BASE_URL/api/users/$USER_ID" | jq '.'
echo ""

# 3. Crea un secondo utente
echo "3. Creating another user..."
curl -s -X POST "$BASE_URL/api/users" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "luigi.verdi@example.com",
    "name": "Luigi Verdi"
  }' | jq '.'
echo ""

# 4. Lista tutti gli utenti
echo "4. Listing all users..."
curl -s -X GET "$BASE_URL/api/users" | jq '.'
echo ""

# 5. Aggiorna l'utente
echo "5. Updating user: $USER_ID"
curl -s -X PUT "$BASE_URL/api/users/$USER_ID" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Mario Rossi Updated"
  }' | jq '.'
echo ""

# 6. Prova a creare un utente con email duplicata
echo "6. Trying to create user with duplicate email (should fail)..."
curl -s -X POST "$BASE_URL/api/users" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "mario.rossi@example.com",
    "name": "Another Mario"
  }' | jq '.'
echo ""

# 7. Prova a creare un utente con email invalida
echo "7. Trying to create user with invalid email (should fail)..."
curl -s -X POST "$BASE_URL/api/users" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "invalid-email",
    "name": "Invalid User"
  }' | jq '.'
echo ""

# 8. Elimina l'utente
echo "8. Deleting user: $USER_ID"
curl -s -X DELETE "$BASE_URL/api/users/$USER_ID" -w "\nHTTP Status: %{http_code}\n"
echo ""

# 9. Prova a ottenere l'utente eliminato
echo "9. Trying to get deleted user (should fail)..."
curl -s -X GET "$BASE_URL/api/users/$USER_ID" | jq '.'
echo ""

echo "=== Examples completed ==="
