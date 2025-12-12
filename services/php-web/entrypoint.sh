#!/usr/bin/env sh
set -e

echo "[php] init start"

# 1. Если в каталоге нет composer.json – создаём новый проект Laravel
if [ ! -f composer.json ]; then
  echo "[php] no composer.json, creating new Laravel project..."
  composer create-project --no-interaction --prefer-dist laravel/laravel .
fi

# 2. Накатываем патчи из /opt/laravel-patches (если есть)
if [ -d /opt/laravel-patches ]; then
  echo "[php] applying laravel patches..."
  rsync -a /opt/laravel-patches/ /var/www/html/
fi

# 3. Устанавливаем зависимости (на случай, если что-то поменялось)
echo "[php] installing composer deps..."
composer install --no-interaction --no-progress --prefer-dist

echo "[php] starting php-fpm..."
exec php-fpm
