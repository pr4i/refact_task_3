<?php

namespace App\Services;

use App\DataSources\AstroApiClient;
use App\DTO\AstroEventDTO;

class AstroService
{
    public function __construct(
        private AstroApiClient $client
    ) {}

    public function getEvents(float $lat, float $lon, int $days): array
    {
        $from = now('UTC')->toDateString();
        $to   = now('UTC')->addDays($days)->toDateString();

        $raw = $this->client->getEvents($lat, $lon, $from, $to);

        $events = $raw['data']['events'] ?? [];

        return array_map(function ($ev) {
            return new AstroEventDTO(
                date: $ev['date'] ?? 'unknown',
                type: $ev['type'] ?? 'unknown',
                body: $ev['body'] ?? 'unknown',
                raw:  $ev
            );
        }, $events);
    }
}
