<?php

namespace App\Services;

use Illuminate\Support\Facades\DB;

class JwstService
{
    /**
     * Картинки для dashboard (из space_cache)
     * Источник чаще всего: APOD (source='apod'), но поддержим и jwst если вдруг пишете туда.
     */
    public function dashboardImages(int $limit = 8): array
    {
        $limit = max(1, min(50, $limit));

        // Берём больше строк, потому что часть будет ошибками/видео без thumbnail и т.п.
        $rows = DB::table('space_cache')
            ->whereIn('source', ['apod', 'jwst'])
            ->orderByDesc('fetched_at')
            ->limit($limit * 8)
            ->get();

        $images = [];
        $seen = [];

        foreach ($rows as $row) {
            $payload = $this->decodePayload($row->payload);
            if (!$payload) continue;

            // NASA rate limit / любые ошибки — пропускаем
            if (!empty($payload['error'])) {
                continue;
            }

            $url = $this->extractImageUrl($payload);
            if (!$url) continue;

            // Нормализуем URL (на всякий)
            $url = $this->normalizeUrl($url);
            if (!$url) continue;

            // Дедуп по URL
            if (isset($seen[$url])) {
                continue;
            }
            $seen[$url] = true;

            $images[] = [
                'url'       => $url,
                'title'     => $payload['title'] ?? $payload['metadata']['study title'] ?? null,
                'source'    => $row->source ?? null,
                'fetched_at'=> (string)($row->fetched_at ?? ''),
            ];

            if (count($images) >= $limit) {
                break;
            }
        }

        return $images;
    }

    private function decodePayload(mixed $payload): ?array
    {
        if (is_array($payload)) return $payload;

        if (is_string($payload)) {
            $decoded = json_decode($payload, true);
            return is_array($decoded) ? $decoded : null;
        }

        // Иногда jsonb из pg прилетает как объект/StdClass
        $decoded = json_decode(json_encode($payload), true);
        return is_array($decoded) ? $decoded : null;
    }

    private function extractImageUrl(array $payload): ?string
    {
        // APOD image
        if (($payload['media_type'] ?? 'image') === 'image') {
            if (!empty($payload['hdurl'])) return (string)$payload['hdurl'];
            if (!empty($payload['url']))   return (string)$payload['url'];
        }

        // APOD video -> thumbnail
        if (($payload['media_type'] ?? null) === 'video') {
            if (!empty($payload['thumbnail_url'])) return (string)$payload['thumbnail_url'];
        }

        // JWST (если вдруг кладёте такие payload в space_cache)
        if (!empty($payload['image_files']) && is_array($payload['image_files'])) {
            $first = $payload['image_files'][0] ?? null;
            if (is_array($first) && !empty($first['file_url'])) {
                return (string)$first['file_url'];
            }
        }

        // Иногда url лежит глубже
        if (!empty($payload['links']['patch']['large'])) {
            return (string)$payload['links']['patch']['large'];
        }

        return null;
    }

    private function normalizeUrl(string $url): ?string
    {
        $url = trim($url);
        if ($url === '') return null;

        // иногда встречается http -> https
        if (str_starts_with($url, 'http://')) {
            $url = 'https://' . substr($url, 7);
        }

        // отсекаем совсем кривые
        if (!preg_match('~^https?://~i', $url)) {
            return null;
        }

        return $url;
    }
}
