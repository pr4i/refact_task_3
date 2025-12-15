<?php

namespace App\Services;

use App\DataSources\RustApiClient;

class IssService
{
    public function __construct(
        private RustApiClient $rust
    ) {}

    /** сырой тренд: [[iso, payload], ...] */
    public function trendPoints(int $limit = 20): array
    {
        return $this->rust->get('/iss/trend?limit=' . $limit);
    }

    /** сводка по последним 2 точкам */
    public function movement(int $limit = 20): array
    {
        $raw = $this->trendPoints(max(2, $limit));

        if (!is_array($raw) || count($raw) < 2) {
            return ['ok' => false];
        }

        // rust отдаёт newest first → берём две первые
        $a = $raw[0] ?? null;
        $b = $raw[1] ?? null;

        $pa = is_array($a) ? ($a[1] ?? []) : [];
        $pb = is_array($b) ? ($b[1] ?? []) : [];

        $lat1 = (float)($pa['latitude'] ?? 0);
        $lon1 = (float)($pa['longitude'] ?? 0);
        $lat2 = (float)($pb['latitude'] ?? 0);
        $lon2 = (float)($pb['longitude'] ?? 0);

        $t1 = (int)($pa['timestamp'] ?? 0);
        $t2 = (int)($pb['timestamp'] ?? 0);

        $dt = abs($t1 - $t2);
        $deltaKm = $this->haversineKm($lat1, $lon1, $lat2, $lon2);

        return [
            'ok'           => $dt > 0,
            'movement'     => $deltaKm > 0.01,
            'delta_km'     => round($deltaKm, 3),
            'dt_sec'       => $dt,
            'velocity_kmh' => $pa['velocity'] ?? null,
        ];
    }

    private function haversineKm(float $lat1, float $lon1, float $lat2, float $lon2): float
    {
        $R = 6371.0;
        $dLat = deg2rad($lat2 - $lat1);
        $dLon = deg2rad($lon2 - $lon1);

        $a = sin($dLat/2) * sin($dLat/2)
           + cos(deg2rad($lat1)) * cos(deg2rad($lat2))
           * sin($dLon/2) * sin($dLon/2);

        $c = 2 * atan2(sqrt($a), sqrt(1-$a));
        return $R * $c;
    }
}
