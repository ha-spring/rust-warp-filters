curl -s http://localhost:8080/
curl -s http://localhost:8080/index.html | grep 'Test file' | tr -d ' '
curl -s http://localhost:8080/todos | jq -c '.[0]'
curl -s http://localhost:8080/todos/123 | jq -c '.'
curl -s -X POST http://localhost:8080/todos -H "Content-Type: application/json" -d '{"id": 34, "title": "todo 34"}'
