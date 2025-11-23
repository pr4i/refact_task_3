<?php

namespace App\Http\Controllers;

use App\Services\IssService;

class IssController extends Controller
{
    public function index(IssService $service)
    {
        $dto = $service->loadData();

        return view('iss', [
            'last'  => $dto->last,
            'trend' => $dto->trend,
            'base'  => env('RUST_BASE')
        ]);
    }
}
