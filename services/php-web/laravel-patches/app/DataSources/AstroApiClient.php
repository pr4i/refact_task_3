<?php

namespace App\DataSources;

use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;

class AstroApiClient
{
    private string $appId;
    private string $secret;

    public function __construct()
    {
        $this->appId  = env('ASTRO_APP_ID', '');
        $this->secret = env('ASTRO_APP_SECRET', '');

        if ($this->appId === '' || $this->secret === '') {
            throw new \RuntimeException("Missing ASTRO_APP_ID or ASTRO_APP_SECRET");
        }
    }

    public function getEvents(float $lat, float $lon, string $from, string $to): array
    {
        $auth = base64_encode($this->appId . ':' . $this->secret);

        $query = [
            'latitude'  => $lat,
            'longitude' => $lon,
            'from'      => $from,
            'to'        => $to,
        ];

        $response = Http::timeout(10)
            ->withHeaders([
                'Authorization' => 'Basic ' . $auth,
                'Content-Type'  => 'application/json',
                'User-Agent'    => 'kasiopea-dashboard/2.0',
            ])
            ->retry(2, 250)
            ->get('https://api.astronomyapi.com/api/v2/bodies/events', $query);

        return $response->json() ?? [];
    }
}
