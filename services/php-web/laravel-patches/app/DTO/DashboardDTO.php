<?php

namespace App\DTO;

class DashboardDTO
{
    public function __construct(
        public readonly array $iss,
        public readonly array $trend,
        public readonly IssMetricDTO $metrics,
        public readonly array $jw_gallery,
        public readonly array $jw_observation_raw,
        public readonly array $jw_observation_summary,
        public readonly array $jw_observation_images,
        public readonly array $jw_observation_files,
    ) {}
}
