<?php

namespace App\DataSources;

use GuzzleHttp\Client;

class JwstDataSource
{
    private Client $client;
    private string $host;
    private string $apiKey;
    private ?string $email;

    public function __construct()
    {
        $this->client = new Client(['timeout' => 10]);

        $this->host  = rtrim(env('JWST_HOST', 'https://api.jwstapi.com'), '/');
        $this->apiKey = env('JWST_API_KEY', '');
        $this->email  = env('JWST_EMAIL', null);
    }

    public function fetch(string $path, array $query = []): array
    {
        $url = $this->host . '/' . ltrim($path, '/');

        try {
            $headers = [
                'x-api-key' => $this->apiKey,
            ];

            if ($this->email) {
                $headers['email'] = $this->email;
            }

            $resp = $this->client->get($url, [
                'headers' => $headers,
                'query' => $query,
            ]);

            $json = json_decode($resp->getBody()->getContents(), true);

            return $json ?? [];
        } catch (\Throwable $e) {
            return [
                'ok' => false,
                'error' => $e->getMessage(),
            ];
        }
    }
}
