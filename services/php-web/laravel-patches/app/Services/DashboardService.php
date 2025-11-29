<?php

namespace App\Services;

use App\DataSources\RustApiClient;

class DashboardService
{
    public function __construct(
        private RustApiClient $client
    ) {}

    public function loadIss(): DashboardDTO
    {
        $iss = $this->client->get('/last');

        $metrics = new IssMetricDTO(
            speed: $iss['payload']['velocity'] ?? null,
            altitude: $iss['payload']['altitude'] ?? null,
            neo_total: 0
        );

    return new DashboardDTO(
        iss: $iss,
        trend: [],
        metrics: $metrics,
        jw_gallery: [],
        jw_observation_raw: [],
        jw_observation_summary: [],
        jw_observation_images: [],
        jw_observation_files: [],
    );
}

}
