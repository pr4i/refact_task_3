<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use Illuminate\Support\Facades\DB;

class TelemetryController extends Controller
{
    public function index(Request $r)
    {
        $q        = trim((string) $r->query('q', ''));              // быстрый поиск
        $keywords = trim((string) $r->query('keywords', ''));       // ключевые слова через пробел
        $dateFrom = trim((string) $r->query('date_from', ''));      // YYYY-MM-DD
        $dateTo   = trim((string) $r->query('date_to', ''));        // YYYY-MM-DD
        $perPage  = (int) $r->query('per_page', 25);
        $perPage  = max(5, min(100, $perPage));

        $sort  = $r->query('sort', 'recorded_at');                 // recorded_at|voltage|temp|source_file
        $order = strtolower((string) $r->query('order', 'desc'));  // asc|desc
        $order = in_array($order, ['asc','desc'], true) ? $order : 'desc';

        $allowedSort = ['recorded_at', 'voltage', 'temp', 'is_ok', 'mode', 'counter', 'source_file'];
        if (!in_array($sort, $allowedSort, true)) {
            $sort = 'recorded_at';
        }

        $query = DB::table('telemetry_legacy');

        // быстрый поиск: по source_file (можно расширить при желании)
        if ($q !== '') {
            $query->where('source_file', 'ilike', '%' . $q . '%');
        }

        // keywords: "solar error anomaly" -> OR по словам в source_file
        if ($keywords !== '') {
            $words = preg_split('/\s+/', $keywords, -1, PREG_SPLIT_NO_EMPTY) ?: [];
            if ($words) {
                $query->where(function ($qq) use ($words) {
                    foreach ($words as $w) {
                        $qq->orWhere('source_file', 'ilike', '%' . $w . '%');
                    }
                });
            }
        }

        // фильтр по датам
        if ($dateFrom !== '') {
            $query->where('recorded_at', '>=', $dateFrom . ' 00:00:00+00');
        }
        if ($dateTo !== '') {
            $query->where('recorded_at', '<=', $dateTo . ' 23:59:59+00');
        }

        $items = $query
            ->orderBy($sort, $order)
            ->paginate($perPage)
            ->withQueryString();

        return view('telemetry', [
            'items'    => $items,
            'q'        => $q,
            'keywords' => $keywords,
            'dateFrom' => $dateFrom,
            'dateTo'   => $dateTo,
            'perPage'  => $perPage,
            'sort'     => $sort,
            'order'    => $order,
        ]);
    }
}
