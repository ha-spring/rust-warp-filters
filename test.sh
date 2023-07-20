curl -s http://localhost:8080/
curl -s http://localhost:8080/index.html | grep 'Test file' | tr -d ' '
curl -s -H "X-Auth-Token: test" http://localhost:8080/todos | jq -c '.[0]'
curl -s -H "X-Auth-Token: test" http://localhost:8080/todos/123 | jq -c '.'
curl -s -X POST http://localhost:8080/todos -H "Content-Type: application/json" -H "X-Auth-Token: test" -d '{"id": 34, "title": "todo 34"}'
echo
echo "=== Errors ==="
curl -s http://localhost:8080/todos/123
echo
curl -s -H "X-Auth-Token: test000" http://localhost:8080/todos/123
