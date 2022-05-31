#/usr/bin/env sh
INI=$(php -r 'echo php_ini_loaded_file();')
EXT_DIR=$(php -r 'echo ini_get("extension_dir");')

cargo php remove --ini-path=$INI --install-dir=$EXT_DIR
cargo php install --release --ini-path=$INI --install-dir=$EXT_DIR