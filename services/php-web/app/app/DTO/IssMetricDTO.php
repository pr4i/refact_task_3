<?php

namespace App\DTO;

class IssMetricDTO
{
    public function __construct(
        public readonly ?float $speed,
        public readonly ?float $altitude,
        public readonly ?int   $neo_total,
    ) {}
}
