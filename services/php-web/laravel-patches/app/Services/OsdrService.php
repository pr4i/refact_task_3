<?php

namespace App\Services;

use App\DataSources\OsdrDataSource;
use App\Utils\OsdrFlattener;

class OsdrService
{
    public function __construct(
        private OsdrDataSource $dataSource
    ) {}

    public function load(int $limit): array
    {
        $data  = $this->dataSource->list($limit);
        $items = $data['items'] ?? [];

        return OsdrFlattener::flatten($items);
    }
}
