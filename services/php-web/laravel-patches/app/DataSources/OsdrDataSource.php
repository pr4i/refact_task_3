<?php

namespace App\DataSources;

class OsdrDataSource
{
    public function __construct(
        private RustApiClient $client
    ) {}

    public function list(int $limit): array
    {
        return $client->get('/osdr/list?limit=' . $limit);
    }
}
