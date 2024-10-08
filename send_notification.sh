#!/bin/bash

# Send a POST request to localhost:3000/notification with an empty body
response=$(curl -X POST http://localhost:3000/notification -d '' -v)

# Print the response
echo "Response: $response"
