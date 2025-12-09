<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use App\Services\JwstService;

class JwstController extends Controller
{
    public function feed(Request $request, JwstService $service)
    {
        $filters = $request->only([
            'source', 'suffix', 'program', 'instrument', 'page', 'perPage'
        ]);

        $data = $service->feed($filters);

        return view('jwst-feed', [
            'items' => $data['items'],
            'page'  => $data['page'],
            'perPage' => $data['perPage'],
            'filters' => $filters,
        ]);
    }
}
