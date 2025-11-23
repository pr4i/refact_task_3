<?php

namespace App\DataSources;

class IssDataSource
{
    protected RustApiClient $client;

    public function __construct(RustApiClient $client)
    {
        $this->client = $client;
    }

    public function getLast(): array
    {
        return $client->get('/last');
    }

    public function getTrend(): array
    {
        return $client->get('/iss/trend');
    }
}
