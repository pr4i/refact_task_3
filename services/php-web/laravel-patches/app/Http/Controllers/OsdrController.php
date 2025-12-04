<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use App\Services\OsdrService;
use App\Utils\OsdrFlattener;

class OsdrController extends Controller
{
    public function index(Request $request, OsdrService $service)
    {
        $limit = (int) $request->query('limit', 20);

        // получаем данные
        $items = $service->getItems($limit);

        // нормализуем (flatten)
        $items = OsdrFlattener::flatten($items);

        return view("cms.osdr", [
            "items" => $items,
            "limit" => $limit
        ]);
    }
}
