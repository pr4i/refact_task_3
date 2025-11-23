<?php

namespace App\Utils;

use App\DTO\OsdrItemDTO;

class OsdrFlattener
{
    public static function flatten(array $items): array
    {
        $out = [];

        foreach ($items as $row) {
            $raw = $row['raw'] ?? [];

            if (is_array($raw) && self::looksOsdrDict($raw)) {

                foreach ($raw as $k => $v) {
                    if (!is_array($v)) continue;

                    $rest = $v['REST_URL'] ?? $v['rest_url'] ?? $v['rest'] ?? null;
                    $title = $v['title'] ?? $v['name'] ?? null;

                    if (!$title && is_string($rest)) {
                        $title = basename(rtrim($rest, '/'));
                    }

                    $out[] = new OsdrItemDTO(
                        id:          $row['id'] ?? null,
                        dataset_id:  $k,
                        title:       $title,
                        status:      $row['status'] ?? null,
                        updated_at:  $row['updated_at'] ?? null,
                        inserted_at: $row['inserted_at'] ?? null,
                        rest_url:    $rest,
                        raw:         $v,
                    );
                }

            } else {
                $out[] = new OsdrItemDTO(
                    id:          $row['id'] ?? null,
                    dataset_id:  null,
                    title:       $row['title'] ?? null,
                    status:      $row['status'] ?? null,
                    updated_at:  $row['updated_at'] ?? null,
                    inserted_at: $row['inserted_at'] ?? null,
                    rest_url:    is_array($raw)
                                    ? ($raw['REST_URL'] ?? $raw['rest_url'] ?? null)
                                    : null,
                    raw:         $raw
                );
            }
        }

        return $out;
    }

    private static function looksOsdrDict(array $raw): bool
    {
        foreach ($raw as $k => $v) {
            if (is_string($k) && str_starts_with($k, 'OSD-')) return true;
            if (is_array($v) && (isset($v['REST_URL']) || isset($v['rest_url']))) return true;
        }

        return false;
    }
}
