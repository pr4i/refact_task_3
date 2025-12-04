public function list(int $limit): array
{
    $resp = $this->client->get('/osdr/list', ['query' => ['limit' => $limit]]);
    $json = json_decode($resp->getBody()->getContents(), true);

    return $json['data']['items'] ?? [];
}
