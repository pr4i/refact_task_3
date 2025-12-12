<?php

namespace App\Http\Controllers;

use App\DataSources\IssDataSource;
use App\Services\IssService;
use App\Services\JwstService;

class DashboardController extends Controller
{
    public function index(
        IssDataSource $issSource,
        IssService $issService,
        JwstService $jwstService
    ) {
        return view('dashboard', [
            'iss'   => $issSource->getLast(),
            'trend' => $issService->trend(),
            'summary' => [
                'neo' => [
                    'payload' => [
                        'element_count' => 0
                    ]
                ]
            ],
            'jwst' => $jwstService->dashboardImages(),
        ]);
    }
}
