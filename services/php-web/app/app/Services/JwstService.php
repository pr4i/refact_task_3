<?php

namespace App\Services;

use Illuminate\Support\Facades\DB;

class JwstService
{
    /**
     * Картинки для dashboard (из space_cache)
     */
    public function dashboardImages(int $limit = 8): array
    {
        $rows = DB::table('space_cache')
            ->where('source', 'apod')   // JWST/APOD приходят туда
            ->orderByDesc('fetched_at')
            ->limit($limit)
            ->get();

        $images = [];

        foreach ($rows as $row) {
            $payload = json_decode($row->payload, true);

            // APOD
            if (!empty($payload['url'])) {
                $images[] = [
                    'url' => $payload['url'],
                    'title' => $payload['title'] ?? null,
                ];
                continue;
            }

            // JWST (если image_files)
            if (!empty($payload['image_files'][0]['file_url'])) {
                $images[] = [
                    'url' => $payload['image_files'][0]['file_url'],
                    'title' => $payload['title'] ?? null,
                ];
            }
        }

        return $images;
    }
}
