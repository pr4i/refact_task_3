<?php

namespace App\Services;

use App\DataSources\IssDataSource;
use App\DataSources\JwstApiClient;
use App\DataSources\RustApiClient;

class DashboardService
{
    public function __construct(
        private IssDataSource $iss,
        private JwstApiClient $jwst,
        private RustApiClient $rust
    ) {}

    /**
     * Основной набор данных для Dashboard
     */
    public function getDashboardData(): array
    {
        $issLast  = $this->iss->getLast();
        $issTrend = $this->iss->getTrend();

        $jwstFeed = $this->jwst->getImages([
            "source" => "jpg",
            "page" => 1,
            "perPage" => 8
        ]);

        $spaceSummary = $this->rust->get("/space/summary");

        return [
            "iss"      => $issLast,
            "trend"    => $issTrend,
            "jwst"     => $jwstFeed["items"] ?? [],
            "summary"  => $spaceSummary["data"] ?? [],
        ];
    }
}
