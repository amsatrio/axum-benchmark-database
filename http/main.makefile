base_url=http://localhost:8009
total_data=100000
id=dbca23a3-5130-49d9-bcba-1cada288a84b

# main
m-root:
	curl --location "${base_url}" -X GET -i
m-health:
	curl --location "${base_url}/health" -X GET -i

# benchmark
b-get-all:
	curl --location "${base_url}/conditions/benchmark/list" -X GET -i
b-delete-all:
	curl --location "${base_url}/conditions/benchmark/delete" -X DELETE -i
b-generate:
	curl --location "${base_url}/conditions/benchmark/generate/${total_data}" -X GET -i

# benchmark diesel
bd-get-all:
	curl --location "${base_url}/conditions_diesel/benchmark/list" -X GET -i
bd-delete-all:
	curl --location "${base_url}/conditions_diesel/benchmark/delete" -X DELETE -i
bd-generate:
	curl --location "${base_url}/conditions_diesel/benchmark/generate/${total_data}" -X GET -i


# CRUD
c-get-all:
	curl --location "${base_url}/conditions/crud/list" -X GET -i
c-get-one:
	curl --location "${base_url}/conditions/crud/${id}" -X GET -i
c-post:
	curl --location "${base_url}/conditions/crud" -X POST -i \
	-H "Content-Type: application/json" \
	-d '{"id":"${id}", "location":"new test created","temperature":51.901744831125534,"humidity":52.8268956302792}'
c-put:
	curl --location "${base_url}/conditions/crud" -X PUT -i \
	-H "Content-Type: application/json" \
	-d '{"id":"${id}","location":"new test updated","temperature":51.901744831125534,"humidity":52.8268956302792}'

# CRUD Diesel
cd-get-all:
	curl --location "${base_url}/conditions_diesel/crud/list" -X GET -i
