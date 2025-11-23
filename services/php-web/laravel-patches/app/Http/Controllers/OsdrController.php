<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use App\Services\OsdrService;

class OsdrController extends Controller
{
    public function index(Request $request, OsdrService $service)
    {
        $limit = (int) $request->query('limit', 20);
        $items = $service->load($limit);

        return view('osdr', [
            'items' => $items,
            'src'   => env('RUST_BASE') . '/osdr/list?limit='.$limit,
        ]);
    }
}
