public function list(int $limit): array
{
    $resp = $this->client->get('/osdr/list', ['limit' => $limit]);

    // Если rust_iss отдаёт {"items":[...]}
    $json = $resp->json();

    return $json['items'] ?? [];
}
