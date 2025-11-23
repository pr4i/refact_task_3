<?php

namespace App\Services;

use App\DataSources\IssDataSource;
use App\DTO\IssDTO;

class IssService
{
    protected IssDataSource $ds;

    public function __construct(IssDataSource $ds)
    {
        $this->ds = $ds;
    }

    public function loadData(): IssDTO
    {
        $last  = $this->ds->getLast();
        $trend = $this->ds->getTrend();

        return new IssDTO($last, $trend);
    }
}
