curl -s http://localhost:8080/
curl -s http://localhost:8080/index.html | grep 'Test file' | tr -d ' '
curl -s http://localhost:8080/todos | jq -c '.[0]'
curl -s http://localhost:8080/todos/123 | jq -c '.'
