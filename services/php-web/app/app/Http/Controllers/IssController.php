<?php

namespace App\Http\Controllers;

use App\DataSources\IssDataSource;
use App\Services\IssService;
use Illuminate\Support\Facades\Cache;

class IssController extends Controller
{
    public function index(IssDataSource $iss, IssService $svc)
    {

        return view('iss', [
        'last' => Cache::remember('iss:last', 5, fn() => $iss->getLast()),
        'move' => Cache::remember('iss:move', 10, fn() => $svc->movement()),
        'trend_points' => $svc->trendPoints(30),
        ]);

    }
}
