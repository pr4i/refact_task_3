<?php

namespace App\ViewModels;

class IssViewModel
{
    public function __construct(
        public ?array $last,
        public ?array $trend
    ) {}
}
