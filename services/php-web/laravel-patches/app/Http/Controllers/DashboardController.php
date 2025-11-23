<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use App\Services\DashboardService;
use App\Services\JwstService;

class DashboardController extends Controller
{
    public function index(DashboardService $service)
    {
        $data = $service->loadIss();

        return view('dashboard', [
            'iss' => $data['iss'],
            'trend' => [],
            'jw_gallery' => [],
            'jw_observation_raw' => [],
            'jw_observation_summary' => [],
            'jw_observation_images' => [],
            'jw_observation_files' => [],
            'metrics' => $data['metrics'],
        ]);
    }

    public function jwstFeed(Request $r, JwstService $service)
    {
        try {
            $result = $service->feed($r->all());

            return response()->json([
                'ok' => true,
                'data' => $result
            ]);

        } catch (\Exception $e) {
            return response()->json([
                'ok' => false,
                'error' => [
                    'code' => 'JWST_API_ERROR',
                    'message' => $e->getMessage(),
                    'trace_id' => uniqid('jwst_', true)
                ]
            ]);
        }
    }
}
