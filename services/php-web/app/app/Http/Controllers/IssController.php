<?php

namespace App\Http\Controllers;

use App\DataSources\IssDataSource;

class IssController extends Controller
{
    public function index(IssDataSource $iss)
    {
        return view('iss', [
            'last' => $iss->getLast(),
            'trend' => [],
            'base' => config('services.rust_iss.url'),
        ]);
    }
}


