<?php

namespace App\Services;

use App\DataSources\AstroDataSource;

class AstroService
{
    public function __construct(
        private AstroDataSource $source
    ) {}

    public function getEvents(float $lat, float $lon, int $days): array
    {
        $from = now('UTC')->toDateString();
        $to   = now('UTC')->addDays($days)->toDateString();

        $params = [
            "latitude"  => $lat,
            "longitude" => $lon,
            "from"      => $from,
            "to"        => $to
        ];

        return $this->source->getEvents($params);
    }
}
