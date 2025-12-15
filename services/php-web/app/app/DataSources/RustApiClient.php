<?php

namespace App\DataSources;

use Illuminate\Support\Facades\Http;

class RustApiClient
{
    private string $base;

    public function __construct()
    {
        $this->base = config('services.rust_iss.url', 'http://rust_iss:3000');
    }

    public function get(string $path): array
    {
        try {
            $resp = Http::baseUrl($this->base)
                ->acceptJson()
                ->timeout(2)       // ключевое!
                ->retry(1, 150)    // 1 повтор и всё
                ->get($path);

            return $resp->json() ?? [];
        } catch (\Throwable $e) {
            // важно: возвращаем быстро, чтобы nginx не ждал
            return ['ok' => false, 'error' => ['message' => $e->getMessage()]];
        }
    }
}
