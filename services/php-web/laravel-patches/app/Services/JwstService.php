<?php

namespace App\Services;

use App\DataSources\JwstApiClient;
use App\DTO\JwstImageDTO;

class JwstService
{
    public function __construct(
        private JwstApiClient $client
    ) {}

    public function feed(array $params): array
    {
        $source = $params['source'] ?? 'jpg';
        $suffix = $params['suffix'] ?? '';
        $program = $params['program'] ?? '';
        $instrument = strtoupper(trim($params['instrument'] ?? ''));
        $page = max(1, (int)($params['page'] ?? 1));
        $perPage = max(1, min(60, (int)($params['perPage'] ?? 24)));

        $path = 'all/type/jpg';
        if ($source === 'suffix' && $suffix !== '') $path = 'all/suffix/' . ltrim($suffix, '/');
        if ($source === 'program' && $program !== '') $path = 'program/id/' . rawurlencode($program);

        $response = $this->client->get($path, [
            'page' => $page,
            'perPage' => $perPage,
        ]);

        $list = $response['body'] ??
                $response['data'] ??
                (is_array($response) ? $response : []);

        $items = [];

        foreach ($list as $it) {
            if (!is_array($it)) continue;

            $url = $it['location'] ?? $it['url'] ?? ($it['thumbnail'] ?? null);

            if (!$url || !preg_match('~\.(jpg|jpeg|png)~i', $url)) continue;

            $instList = [];
            foreach (($it['details']['instruments'] ?? []) as $I) {
                if (!empty($I['instrument'])) {
                    $instList[] = strtoupper($I['instrument']);
                }
            }

            if ($instrument && $instList && !in_array($instrument, $instList, true)) {
                continue;
            }

            $items[] = new JwstImageDTO(
                url: $url,
                caption: ($it['observation_id'] ?? 'obs') . ' P' . ($it['program'] ?? ''),
                program: $it['program'] ?? '',
                obs: $it['observation_id'] ?? '',
                instruments: $instList,
                suffix: $it['details']['suffix'] ?? '',
                link: $it['location'] ?? $url
            );

            if (count($items) >= $perPage) break;
        }

        return [
            'source' => $path,
            'count'  => count($items),
            'items'  => $items,
        ];
    }
}
