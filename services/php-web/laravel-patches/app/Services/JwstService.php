<?php

namespace App\Services;

use App\DataSources\JwstDataSource;

class JwstService
{
    public function __construct(
        private JwstDataSource $source
    ) {}

    public function feed(array $filters): array
    {
        $source   = $filters['source'] ?? 'jpg';
        $suffix   = $filters['suffix'] ?? '';
        $program  = $filters['program'] ?? '';
        $inst     = strtoupper(trim($filters['instrument'] ?? ''));
        $page     = max(1, (int)($filters['page'] ?? 1));
        $perPage  = max(1, min(60, (int)($filters['perPage'] ?? 24)));

        // выбор эндпоинта
        $path = match ($source) {
            'suffix'  => "all/suffix/" . ltrim($suffix, '/'),
            'program' => "program/id/" . urlencode($program),
            default   => "all/type/jpg"
        };

        $resp = $source === 'program'
            ? $this->source->fetch($path, ['page' => $page, 'perPage' => $perPage])
            : $this->source->fetch($path, ['page' => $page, 'perPage' => $perPage]);

        $items = $resp['body'] ?? $resp['data'] ?? (is_array($resp) ? $resp : []);

        // фильтр по инструменту
        if ($inst) {
            $items = array_filter($items, function ($item) use ($inst) {
                $instruments = array_map(
                    fn($d) => strtoupper($d['instrument'] ?? ''),
                    $item['details']['instruments'] ?? []
                );
                return in_array($inst, $instruments, true);
            });
        }

        return [
            'items' => $items,
            'page' => $page,
            'perPage' => $perPage,
            'source' => $path
        ];
    }
}
