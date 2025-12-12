FROM php:8.3-fpm-alpine

# Нужные пакеты (БЕЗ gcc!)
RUN apk add --no-cache \
    bash \
    curl \
    git \
    unzip \
    rsync \
    postgresql-client \
    postgresql-libs \
    postgresql-dev

# Сборка PHP модулей
RUN docker-php-ext-install pdo pdo_pgsql

WORKDIR /var/www/html

CMD ["php-fpm"]
