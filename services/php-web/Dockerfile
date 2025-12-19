FROM php:8.3-fpm-alpine

# Runtime пакеты (нужны в контейнере при запуске)
RUN apk add --no-cache \
    bash \
    curl \
    git \
    unzip \
    rsync \
    postgresql-client \
    postgresql-libs \
    zip \
    libzip

# Build deps только на время сборки расширений
RUN apk add --no-cache --virtual .build-deps \
      $PHPIZE_DEPS \
      libzip-dev \
      postgresql-dev \
  && docker-php-ext-configure zip \
  && docker-php-ext-install pdo pdo_pgsql zip \
  && apk del .build-deps

WORKDIR /var/www/html
CMD ["php-fpm"]
