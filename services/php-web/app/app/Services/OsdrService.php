<?php

namespace App\Services;

use App\DataSources\RustApiClient;

class OsdrService
{
    public function __construct(
        private RustApiClient $rust
    ) {}

    /**
     * Возвращает список датасетов OSDR из rust_iss.
     * Rust обычно отдаёт {"items":[...]}.
     */
    public function getItems(int $limit): array
    {
        // rust_iss может поддерживать ?limit=...
        $data = $this->rust->get('/osdr/list?limit=' . $limit);

        return $data['items'] ?? [];
    }

    /**
     * Принудительная синхронизация (если ручка есть в rust_iss).
     */
    public function sync(): array
    {
        return $this->rust->get('/osdr/sync');
    }
}
