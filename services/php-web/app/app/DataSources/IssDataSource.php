<?php

namespace App\DataSources;

use Illuminate\Support\Facades\Http;

class IssDataSource
{
    protected $client;

    public function __construct()
    {
        $baseUrl = config('services.rust_iss.url');

        if (!$baseUrl) {
            throw new \RuntimeException('RUST_ISS_URL is not configured');
        }

        $this->client = Http::baseUrl($baseUrl);
    }

    public function getLast(): array
    {
        return $this->client
            ->get('/iss/last')
            ->json() ?? [];
    }
}
