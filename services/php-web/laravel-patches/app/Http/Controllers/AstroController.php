<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use App\Services\AstroService;

class AstroController extends Controller
{
    public function events(Request $r, AstroService $service)
    {
        $lat  = (float) $r->query('lat', 55.7558);
        $lon  = (float) $r->query('lon', 37.6176);
        $days = max(1, min(30, (int) $r->query('days', 7)));

        try {
            $events = $service->getEvents($lat, $lon, $days);

            return response()->json([
                'ok'   => true,
                'data' => $events
            ]);

        } catch (\Exception $e) {

            return response()->json([
                'ok'    => false,
                'error' => [
                    'code' => 'ASTRO_API_ERROR',
                    'message' => $e->getMessage(),
                    'trace_id' => uniqid("astro_", true)
                ]
            ]);
        }
    }
}
