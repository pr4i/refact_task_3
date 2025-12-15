<?php

namespace App\DataSources;

use Illuminate\Support\Facades\Http;

class IssDataSource
{
    protected $client;

    public function __construct()
    {
        $baseUrl = config('services.rust_iss.url');
        if (!$baseUrl) throw new \RuntimeException('RUST_ISS_URL is not configured');

        $this->client = Http::baseUrl($baseUrl)
            ->acceptJson()
            ->timeout(2)
            ->retry(1, 150);
    }

    public function getLast(): array
    {
        try {
            return $this->client->get('/iss/last')->json() ?? [];
        } catch (\Throwable $e) {
            return ['ok' => false, 'error' => ['message' => $e->getMessage()]];
        }
    }
}
