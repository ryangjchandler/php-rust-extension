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
    json_decode(DATA, true);
}

$end = microtime(true) - $start;

echo "Time taken: {$end} seconds\n";