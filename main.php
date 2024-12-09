<?php
$start = microtime(true);
$bcrypt_hash = password_hash('password', PASSWORD_BCRYPT);
$time_elapsed = (microtime(true) - $start) * 1000;
echo "Bcrypt hash elapsed time: $time_elapsed ms\n";

$start = microtime(true);
$verify = password_verify('password', $bcrypt_hash);
$time_elapsed = (microtime(true) - $start) * 1000;
echo "Bcrypt verify elapsed time: $time_elapsed ms\n";

$start = microtime(true);
$argon_hash = password_hash('password', PASSWORD_ARGON2ID);
$time_elapsed = (microtime(true) - $start) * 1000;
echo "Argon2id hash elapsed time: $time_elapsed ms\n";

$start = microtime(true);
$verify = password_verify('password', $argon_hash);
$time_elapsed = (microtime(true) - $start) * 1000;
echo "Argon2id verify elapsed time: $time_elapsed ms\n";

echo "--------\n";

echo "Bcrypt cost: " . PASSWORD_BCRYPT_DEFAULT_COST . "\n";
echo "Bcrypt length: " . strlen($bcrypt_hash) . "\n";

echo "Argon2id memory cost: " . PASSWORD_ARGON2_DEFAULT_MEMORY_COST . "\n";
echo "Argon2id time cost: " . PASSWORD_ARGON2_DEFAULT_TIME_COST . "\n";
echo "Argon2id threads: " . PASSWORD_ARGON2_DEFAULT_THREADS . "\n";
echo "Argon2id hash: " . $argon_hash . "\n";
