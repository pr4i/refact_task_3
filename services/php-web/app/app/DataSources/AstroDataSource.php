<?php

namespace App\DataSources;

use GuzzleHttp\Client;

class AstroDataSource
{
    private Client $client;
    private string $host;
    private string $auth;

    public function __construct()
    {
        $this->host = "https://api.astronomyapi.com/api/v2";
        $this->client = new Client(['timeout' => 10]);

        $appId  = env("ASTRO_APP_ID");
        $secret = env("ASTRO_APP_SECRET");

        $this->auth = "Basic " . base64_encode("$appId:$secret");
    }

    public function getEvents(array $params): array
    {
        try {
            $response = $this->client->get($this->host . "/bodies/events", [
                "headers" => [
                    "Authorization" => $this->auth,
                    "User-Agent" => "space-dashboard/1.0"
                ],
                "query" => $params
            ]);

            $json = json_decode($response->getBody()->getContents(), true);

            return $json["data"] ?? $json;
        } catch (\Throwable $e) {
            return [
                "ok" => false,
                "error" => [
                    "message" => $e->getMessage(),
                    "code" => "ASTRO_API_ERROR"
                ]
            ];
        }
    }
}
