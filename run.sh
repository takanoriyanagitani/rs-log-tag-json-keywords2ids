#!/bin/sh

export ENV_MATCH_KIND=left-most-longest
export ENV_MATCH_KIND=long

export ENV_BODY_KEY=body
export ENV_TAGS_KEY=tags

keywords=
keywords="${keywords} apple"
keywords="${keywords} market"
keywords="${keywords} 3776"
keywords="${keywords} bought"
keywords="${keywords} time"

jq -c -n '{
	time:"2025-02-07T01:41:54.012345Z",
	level:"INFO",
	body:"User 3776 went to the market and bought an apple.",
	http_status: 200,
	service_name: "user service",
	service_pid: 42,
}' |
	./rs-log-tag-json-keywords2ids $( echo $keywords ) |
	jq

keywords=
keywords="${keywords} disk"
keywords="${keywords} Time"
keywords="${keywords} Timeout"
keywords="${keywords} space"
keywords="${keywords} low"

jq -c -n '{
	time:"2025-02-07T01:41:54.012345Z",
	level:"INFO",
	body:"Timeout: disk space low",
	http_status: 500,
	service_name: "user service",
	service_pid: 42,
}' |
	./rs-log-tag-json-keywords2ids $( echo $keywords ) |
	jq
