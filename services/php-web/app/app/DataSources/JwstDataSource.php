<?php

namespace App\DataSources;

use Illuminate\Support\Facades\Http;

class JwstDataSource
{
    public function getLatest(int $limit = 8): array
    {
        $response = Http::get('https://api.nasa.gov/planetary/apod', [
            'api_key' => env('NASA_KEY', 'DEMO_KEY'),
            'count' => $limit,
        ]);

        $data = $response->json() ?? [];

        // 🔥 НОРМАЛИЗАЦИЯ ДЛЯ ФРОНТА
        return collect($data)
            ->filter(fn ($item) =>
                ($item['media_type'] ?? null) === 'image'
                && isset($item['url'])
            )
            ->map(fn ($item) => [
                'url'   => $item['hdurl'] ?? $item['url'],
                'title' => $item['title'] ?? '',
                'date'  => $item['date'] ?? '',
            ])
            ->values()
            ->all();
    }
}
