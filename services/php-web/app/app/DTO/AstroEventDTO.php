<?php

namespace App\DTO;

class AstroEventDTO
{
    public function __construct(
        public readonly string $date,
        public readonly string $type,
        public readonly string $body,
        public readonly ?array $raw,
    ) {}
}
