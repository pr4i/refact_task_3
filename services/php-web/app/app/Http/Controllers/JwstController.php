<?php

namespace App\Http\Controllers;

use App\DataSources\JwstDataSource;

class JwstController extends Controller
{
    public function index(JwstDataSource $jwst)
    {
        return view('jwst-feed', [
            'items' => $jwst->getLatest(),
            'filters' => [],
            'perPage' => 9,
        ]);
    }
}
