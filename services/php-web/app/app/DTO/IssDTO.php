<?php

namespace App\DTO;

class IssDTO
{
    public array $last;
    public array $trend;

    public function __construct(array $last, array $trend)
    {
        $this->last  = $last;
        $this->trend = $trend;
    }
}
