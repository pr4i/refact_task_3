<?php

namespace App\DataSources;

use Illuminate\Support\Facades\Http;

class JwstApiClient
{
    private string $base = 'https://jwstapi.com/api/v1/';

    public function get(string $path, array $query = []): array
    {
        $response = Http::timeout(5)
            ->retry(2, 200)
            ->get($this->base . ltrim($path, '/'), $query);

        return $response->json() ?? [];
    }
}
