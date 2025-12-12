<?php

namespace App\DTO;

class OsdrItemDTO
{
    public function __construct(
        public readonly string|null $id,
        public readonly string|null $dataset_id,
        public readonly string|null $title,
        public readonly string|null $status,
        public readonly string|null $updated_at,
        public readonly string|null $inserted_at,
        public readonly string|null $rest_url,
        public readonly array $raw,
    ) {}
}
