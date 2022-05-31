<?php

$start = microtime(true);
$pi = calculate_pi(100_000_000);
$end = microtime(true) - $start;

echo "π = {$pi}\n";
echo "Time taken: {$end} seconds.\n";