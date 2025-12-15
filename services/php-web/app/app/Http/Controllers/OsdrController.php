<?php

namespace App\Http\Controllers;

use App\DataSources\RustApiClient;
use Illuminate\Http\Request;

class OsdrController extends Controller
{
    public function index(Request $request, RustApiClient $rust)
    {
        $limit = (int) $request->get('limit', 20);
        $limit = max(1, min(200, $limit));

        $sort  = (string) $request->get('sort', 'inserted_at');
        $order = strtolower((string) $request->get('order', 'desc')) === 'asc' ? 'asc' : 'desc';
        $search = trim((string) $request->get('search', ''));

        // берём с Rust сразу нужное количество (он отдаёт items)
        $data  = $rust->get('/osdr/list?limit=' . $limit);
        $items = $data['items'] ?? [];

        // правильный REST_URL для карточки датасета
        foreach ($items as &$row) {
            $id = $row['dataset_id'] ?? null;
            $row['rest_url'] = $id
                ? "https://visualization.osdr.nasa.gov/biodata/api/v2/dataset/{$id}/?format=json"
                : null;
        }
        unset($row);

        // поиск
        if ($search !== '') {
            $needle = mb_strtolower($search);
            $items = array_values(array_filter($items, function ($r) use ($needle) {
                $hay = mb_strtolower(($r['dataset_id'] ?? '') . ' ' . ($r['title'] ?? ''));
                return str_contains($hay, $needle);
            }));
        }

        // сортировка (форма уже есть — просто оживляем)
        $allowedSort = ['inserted_at', 'updated_at', 'title'];
        if (!in_array($sort, $allowedSort, true)) {
            $sort = 'inserted_at';
        }

        usort($items, function ($a, $b) use ($sort, $order) {
            $va = $a[$sort] ?? null;
            $vb = $b[$sort] ?? null;

            // null в конец
            if ($va === null && $vb === null) return 0;
            if ($va === null) return 1;
            if ($vb === null) return -1;

            // строки времени сортируем как строки (ISO) — работает нормально
            if ($sort === 'title') {
                $cmp = strcasecmp((string)$va, (string)$vb);
            } else {
                $cmp = strcmp((string)$va, (string)$vb);
            }

            return $order === 'asc' ? $cmp : -$cmp;
        });

        return view('osdr', [
            'items' => $items,
        ]);
    }

    public function dashboard(Request $request, RustApiClient $rust)
    {
        $limit = (int) $request->get('limit', 50);
        $limit = max(1, min(500, $limit)); // в blade у тебя есть 500

        $data  = $rust->get('/osdr/list?limit=' . $limit);
        $items = $data['items'] ?? [];

        $total = count($items);

        $today = date('Y-m-d');
        $updatedToday = 0;
        $updatedLast7 = 0;

        // последние 7 дней (включая сегодня)
        $daily = [];
        for ($i = 6; $i >= 0; $i--) {
            $d = date('Y-m-d', strtotime("-{$i} day"));
            $daily[$d] = 0;
        }

        $statusCounts = [];
        foreach ($items as $r) {
            // updated_at обычно строка RFC3339: "2017-09-01T00:00:00Z"
            $u = $r['updated_at'] ?? null;
            if ($u) {
                $d = substr((string)$u, 0, 10);

                if ($d === $today) {
                    $updatedToday++;
                }

                if (array_key_exists($d, $daily)) {
                    $daily[$d]++;
                    $updatedLast7++;
                }
            }

            $st = $r['status'] ?? 'unknown';
            $statusCounts[$st] = ($statusCounts[$st] ?? 0) + 1;
        }

        return view('osdr-dashboard', [
            'items' => $items,
            'total' => $total,
            'updatedToday' => $updatedToday,
            'updatedLast7' => $updatedLast7,

            'dailyLabels' => array_keys($daily),
            'dailyValues' => array_values($daily),

            'statusLabels' => array_keys($statusCounts),
            'statusValues' => array_values($statusCounts),

            'limit' => $limit,
        ]);
    }

}
