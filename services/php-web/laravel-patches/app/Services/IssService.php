<?php

namespace App\Services;

use GuzzleHttp\Client;

class IssService
{
    private Client $client;
    private string $base;

    public function __construct()
    {
        $this->client = new Client(['timeout' => 5]);
        $this->base = env('RUST_BASE', 'http://rust_iss:3000');
    }

    public function getIssData(): array
    {
        $last = $this->fetch('/last');
        $trend = $this->fetch('/iss/trend');

        return [
            'last' => $last['data'] ?? null,
            'trend' => $trend['data'] ?? null
        ];
    }

    private function fetch(string $url)
    {
        try {
            $resp = $this->client->get($this->base . $url);
            return json_decode($resp->getBody()->getContents(), true);
        } catch (\Throwable $e) {
            return ['ok' => false, 'error' => ['message' => $e->getMessage()]];
        }
    }
}
