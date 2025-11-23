<?php

namespace App\DataSources;

use Illuminate\Support\Facades\Http;

class RustApiClient
{
    protected string $base;

    public function __construct()
    {
        $this->base = env('RUST_BASE', 'http://rust_iss:3000');
    }

    public function get(string $path): array
    {
        $response = Http::timeout(3)
            ->retry(2, 200)
            ->get($this->base . $path);

        return $response->json() ?? [];
    }
}
