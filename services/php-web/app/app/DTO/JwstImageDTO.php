<?php

namespace App\DTO;

class JwstImageDTO
{
    public function __construct(
        public readonly string $url,
        public readonly string $caption,
        public readonly ?string $program,
        public readonly ?string $obs,
        public readonly array  $instruments,
        public readonly string $suffix,
        public readonly string $link,
    ) {}
}
