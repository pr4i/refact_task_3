<?php

namespace App\Http\Controllers;

use App\DataSources\IssDataSource;
use App\DataSources\RustApiClient;
use App\Services\IssService;
use App\Services\JwstService;
use Illuminate\Support\Facades\Cache;

class DashboardController extends Controller
{
    public function index(
        IssDataSource $issSource,
        IssService $issService,
        JwstService $jwstService,
        RustApiClient $rust
    ) {
        $iss = Cache::remember('iss_last', 5, fn() => $issSource->getLast());
        $move = Cache::remember('iss_move', 5, fn() => $issService->movement());

        // summary самое “тяжелое” → кэшируем дольше
        $summary = Cache::remember('space_summary', 30, fn() => $rust->get('/space/summary'));
        if (!is_array($summary)) $summary = [];

        return view('dashboard', [
            'iss'     => $iss,
            'move'    => $move,
            'trend_points' => $issService->trendPoints(30),
            'summary' => $summary,
            'jwst'    => $jwstService->dashboardImages(),
        ]);
    }
}
