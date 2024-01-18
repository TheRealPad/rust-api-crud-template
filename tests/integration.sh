## SET VAR
PORT=8085
URL=http://localhost:$PORT
TEST_URLS=("/users" "/null")
TEST_RETURN_CODE=(200 404)
TEST_METHOD=("GET" "GET")
TEST_HEADER=("" "")
TEST_BODY=("" "")
PASS_COUNT=0
FAIL_COUNT=0
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

## SET ENV
export API_PORT=$PORT

test_curl_response() {
  local url="$1"
  local method="$2"
  local expected_status="$3"
  local headers="$4"
  local body="$5"

  http_status=$(curl -s -o /dev/null -w "%{http_code}" -X "$method" -H "$headers" -d "$body" "$url")
  if [[ "$http_status" -ge "$expected_status" && "$http_status" -lt "$((expected_status + 100))" ]]; then
    echo "${GREEN}Test passed${NC}: $method $url HTTP status code is $http_status"
    ((PASS_COUNT++))
    return 0
  else
    echo "${RED}Test failed${NC}: $method $url Expected HTTP status $expected_status, but received $http_status"
    ((FAIL_COUNT++))
    return 1
  fi
}

start_server () {
  cargo run &
  server_pid=$!
  echo "Server started with PID: $server_pid"
}

run_test () {
  for i in "${!TEST_URLS[@]}"; do
    test_curl_response "$URL${TEST_URLS[$i]}" "${TEST_METHOD[$i]}" "${TEST_RETURN_CODE[$i]}" "${TEST_HEADER[$i]}" "${TEST_BODY[$i]}"
  done
  printf "[${GREEN}PASS${NC}]: $PASS_COUNT\n"
  printf "[${RED}FAIL${NC}]: $FAIL_COUNT\n"
}

end_integration_test () {
  kill $server_pid
  if [ "$FAIL_COUNT" -gt 0 ]; then
    exit 1
  else
    exit 0
  fi
}

wait_for_server() {
  local max_attempts=30
  local delay=5
  local count=0

  until curl -s "$URL" &>/dev/null; do
    if [ $count -eq $max_attempts ]; then
      echo "Timed out waiting for server to start."
      exit 1
    fi

    ((count++))
    echo "Waiting for the server to start... Attempt $count/$max_attempts"
    sleep $delay
  done
}

start_integration_test () {
  start_server
  wait_for_server
  run_test
  end_integration_test
}

start_integration_test
