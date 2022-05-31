<?php

const DATA = <<<'json'
{
    "name": "Ryan",
    "age": 100,
    "alive": true,
    "meta": {
        "tags": ["cool", "rust", "php"]
    }
}
json;

$start = microtime(true);

for ($i = 0; $i < 10_000_000; $i++) {
    fast_json_decode(DATA);
}

$end = microtime(true) - $start;

echo "Time taken: {$end} seconds\n";