<?php

namespace App\Services;

use Illuminate\Support\Facades\DB;

class IssService
{
    public function trend(int $limit = 20): array
    {
        $rows = DB::table('iss_fetch_log')
            ->orderByDesc('fetched_at')
            ->limit($limit)
            ->get()
            ->reverse()
            ->values();

        if ($rows->count() < 2) {
            return ['delta_km' => 0];
        }

        $first = json_decode($rows->first()->payload, true);
        $last  = json_decode($rows->last()->payload, true);

        return [
            'delta_km' => abs(
                ($last['longitude'] ?? 0) - ($first['longitude'] ?? 0)
            ),
        ];
    }
}
