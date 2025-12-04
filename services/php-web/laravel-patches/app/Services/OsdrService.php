<?php

namespace App\Services;

use App\DataSources\OsdrDataSource;

class OsdrService
{
    public function __construct(
        private OsdrDataSource $source
    ) {}

    public function getItems(int $limit): array
    {
        $items = $this->source->list($limit);

        // если у тебя есть DTO – вот тут можно маппить через OsdrItemDTO::fromArray()
        return $items;
    }
}
